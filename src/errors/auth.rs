use thiserror::Error;
use sea_orm::DbErr;
use axum::{response::IntoResponse, http::{StatusCode}};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("database error")]
    DataBaseError,
    #[error("user already exist")]
    UserAlreadyExist,
    #[error("invalid creds")]
    InvalidCredentials,
    #[error("hashing error")]
    HashingError,
    #[error("invalid access token")]
    InvalidToken,
    #[error("token creation error")]
    TokenCreationError,
    #[error("invalid referal code")]
    InvalidRefCode,
    
    
}
impl From<DbErr> for AuthError {
    fn from(_: DbErr) -> Self {
        AuthError::DataBaseError
    }
}


