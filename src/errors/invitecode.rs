use thiserror::Error;
use sea_orm::DbErr;
use axum::response::{IntoResponse, Response};


#[derive(Debug, Error)]
pub enum InviteCodeError {
    #[error("database error")]
    DataBaseError,
    #[error("does not exist")]
    NotExistError,
}

impl From<DbErr> for InviteCodeError {
    fn from(_: DbErr) -> Self {
        InviteCodeError::DataBaseError
    }
}
impl IntoResponse for InviteCodeError {
    fn into_response(self) -> Response {
        let body = match self {
            InviteCodeError::DataBaseError => "database error",
            InviteCodeError::NotExistError => "does not exist",
        };

        body.into_response()
    }
}
