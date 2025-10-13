use thiserror::Error;
use sea_orm::DbErr;
use axum::response::{IntoResponse, Response};


#[derive(Debug, Error)]
pub enum FriendError {
    #[error("no request exist")]
    NoFriendRequest,
    #[error("database error")]
    DataBaseError,
}
impl From<DbErr> for FriendError {
    fn from(_: DbErr) -> Self {
        FriendError::DataBaseError
    }
}
impl IntoResponse for FriendError {
    fn into_response(self) -> Response {
        let body = match self {
            FriendError::NoFriendRequest => "request does not exist",
            FriendError::DataBaseError => "database error",
        };

        body.into_response()
    }
}
