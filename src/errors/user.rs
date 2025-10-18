use thiserror::Error;
use axum::response::{IntoResponse, Response};
use crate::errors::infra::InfraError;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("no user exist")]
    NoUserExist,
    #[error("not allowed image type")]
    NotAllowedImageType,
    #[error("internal server error")]
    InternalError,
    #[error(transparent)]
    Infra(#[from]InfraError),
}
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let body = match self {
            UserError::NoUserExist => "user does not exist",
            UserError::NotAllowedImageType => "not allowed image type",
            UserError::InternalError => "internal server error",
            UserError::Infra(_) => "database error", 

        };

        body.into_response()
    }
}
