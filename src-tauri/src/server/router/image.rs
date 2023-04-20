use axum::{
    extract::{BodyStream, Path},
    http::StatusCode,
};

use crate::server::helper::image::stream_to_file;

pub async fn upload(
    Path(file_name): Path<String>,
    body: BodyStream,
) -> Result<(), (StatusCode, String)> {
    stream_to_file(&file_name, body).await
}
