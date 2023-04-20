use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest},
    http::{self, HeaderMap, Request},
    response::{IntoResponse, Response},
    Json,
};
use bytes::{BufMut, BytesMut};
use chatgpt::prelude::ResponseChunk;
use futures::Stream;
use futures_util::{stream::BoxStream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Write;
use std::pin::Pin;
use std::task::{Context, Poll};
use validator::Validate;

use crate::error::{Error, FieldValidator};
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        FieldValidator::validate(&value).check()?;
        Ok(ValidatedJson(value))
    }
}

pub struct StreamBodyResponse<'a>(BoxStream<'a, Result<axum::body::Bytes, axum::Error>>);

impl IntoResponse for StreamBodyResponse<'static> {
    fn into_response(self) -> Response {
        let mut res = Response::new(axum::body::boxed(self));
        res.headers_mut().insert(
            http::header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
        res
    }
}

impl<'a> HttpBody for StreamBodyResponse<'a> {
    type Data = axum::body::Bytes;
    type Error = axum::Error;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        Pin::new(&mut self.0).poll_next(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            http::header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );
        Poll::Ready(Ok(Some(header_map)))
    }
}

pub trait StreamingFormat<T> {
    fn to_bytes_stream<'a, 'b>(
        &'a self,
        stream: BoxStream<'b, T>,
    ) -> BoxStream<'b, Result<axum::body::Bytes, axum::Error>>;
}

pub struct ChatGPTStreamRawFormat;

impl ChatGPTStreamRawFormat {
    fn to_bytes_stream<'b>(
        stream: BoxStream<'b, ResponseChunk>,
    ) -> BoxStream<'b, Result<axum::body::Bytes, axum::Error>> {
        let stream_bytes: BoxStream<Result<axum::body::Bytes, axum::Error>> = Box::pin({
            stream.enumerate().map(|(_index, obj)| {
                let rc: Vec<u8> = match obj {
                    ResponseChunk::Content {
                        delta,
                        response_index: _,
                    } => delta.into_bytes(),
                    _ => "\n".as_bytes().to_vec(),
                };
                let mut buf = BytesMut::new().writer();
                buf.write_all(&rc).unwrap();

                Ok(buf.into_inner().freeze())
            })
        });

        Box::pin(stream_bytes)
    }
}

pub struct ChatGPTStreamJsonFormat;

impl ChatGPTStreamJsonFormat {
    fn to_bytes_stream<'b>(
        stream: BoxStream<'b, ResponseChunk>,
    ) -> BoxStream<'b, Result<axum::body::Bytes, axum::Error>> {
        let stream_bytes: BoxStream<Result<axum::body::Bytes, axum::Error>> = Box::pin({
            stream.enumerate().map(|(index, obj)| {
                let rc: ResponseObject<String> = match obj {
                    ResponseChunk::Content {
                        delta,
                        response_index: _,
                    } => ResponseObject {
                        code: 20000,
                        msg: "Content".to_string(),
                        data: Some(delta),
                    },
                    ResponseChunk::BeginResponse {
                        role: _,
                        response_index: _,
                    } => ResponseObject {
                        code: 20000,
                        msg: "BeginResponse".to_string(),
                        data: None,
                    },
                    ResponseChunk::CloseResponse { response_index: _ } => ResponseObject {
                        code: 20000,
                        msg: "CloseResponse".to_string(),
                        data: None,
                    },
                    ResponseChunk::Done => ResponseObject {
                        code: 20000,
                        msg: "Done".to_string(),
                        data: None,
                    },
                };

                let mut buf = BytesMut::new().writer();
                if index != 0 {
                    buf.write_all("\n".as_bytes()).unwrap();
                }
                match serde_json::to_writer(&mut buf, &rc) {
                    Ok(_) => Ok(buf.into_inner().freeze()),
                    Err(e) => Err(axum::Error::new(e)),
                }
            })
        });

        Box::pin(stream_bytes)
    }
}

impl<'a> StreamBodyResponse<'a> {
    pub fn json<S>(stream: S) -> Self
    where
        S: Stream<Item = ResponseChunk> + 'a + Send,
    {
        StreamBodyResponse(ChatGPTStreamJsonFormat::to_bytes_stream(Box::pin(stream)))
    }

    pub fn raw<S>(stream: S) -> Self
    where
        S: Stream<Item = ResponseChunk> + 'a + Send,
    {
        StreamBodyResponse(ChatGPTStreamRawFormat::to_bytes_stream(Box::pin(stream)))
    }
}

#[derive(Debug, Serialize)]
struct ResponseObject<T: Serialize + Send + Sync> {
    code: i32,
    msg: String,
    data: Option<T>,
}
