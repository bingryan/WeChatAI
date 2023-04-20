use axum::{
    extract::rejection::{FormRejection, JsonRejection},
    http::{header::WWW_AUTHENTICATE, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error as ThisError;
use validator::{Validate, ValidationErrors};

pub const SCHEME_PREFIX: &str = "Bearer ";

/// custom result
pub type Result<T, E = Error> = std::result::Result<T, E>;

///  error
#[derive(ThisError, Debug)]
pub enum Error {
    /// Return `400 Unauthorized`
    #[error("client error")]
    BadRequest,

    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("not accept that authentication")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("resource not found")]
    NotFound,

    /// Return `500 Internal Server Error`
    #[error("Internal Server Error")]
    InternalServerError,

    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    UnprocessableEntity {
        /// errors map
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    /// validator
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    /// axum FormRejection
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),

    /// axum JsonRejection
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    /// Source and Display delegate to anyhow::Error
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Axum(#[from] axum::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),

    #[error(transparent)]
    Chatgpt(#[from] chatgpt::err::Error),
}

// for tauri::command
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Error {
    /// unprocessable entity
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Anyhow(_) | Self::Axum(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// map error
    pub fn map_err(self) -> Error {
        match self {
            _ => Error::NotFound,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    [(
                        WWW_AUTHENTICATE,
                        HeaderValue::from_static(SCHEME_PREFIX.trim()),
                    )]
                    .into_iter()
                    .collect::<HeaderMap>(),
                    self.to_string(),
                )
                    .into_response();
            }

            Self::Anyhow(ref e) => {
                tracing::error!("Generic error: {:?}", e);
            }

            Self::Axum(ref e) => {
                tracing::error!("Axum error: {:?}", e);
            }

            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

/// Field Validator
pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    /// validate the model and get  errors
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }
    /// check errors and map errors
    pub fn check(self) -> Result<(), Error> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            use validator::ValidationErrorsKind::{Field, List, Struct};

            let mut error_map = HashMap::new();
            // FIXME: optimize logic
            for (key, val) in self.errors.into_errors() {
                match val {
                    Field(field_errors) => {
                        error_map.insert(
                            Cow::Borrowed(key),
                            field_errors
                                .into_iter()
                                .map(|field_error| field_error.code)
                                .collect(),
                        );
                    }
                    List(list_errors) => {
                        for (_error_key, error_val) in list_errors {
                            for (list_key, list_val) in error_val.into_errors() {
                                if let Field(field_errors) = list_val {
                                    error_map.insert(
                                        Cow::Borrowed(list_key),
                                        field_errors
                                            .into_iter()
                                            .map(|field_error| field_error.code)
                                            .collect(),
                                    );
                                }
                            }
                        }
                    }
                    Struct(struct_errors) => {
                        for (struct_key, struct_val) in struct_errors.into_errors() {
                            if let Field(field_errors) = struct_val {
                                error_map.insert(
                                    Cow::Borrowed(struct_key),
                                    field_errors
                                        .into_iter()
                                        .map(|field_error| field_error.code)
                                        .collect(),
                                );
                            }
                        }
                    }
                }
            }

            Err(Error::UnprocessableEntity { errors: error_map })
        }
    }
}
