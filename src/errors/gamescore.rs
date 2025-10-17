use thiserror::Error;
use axum::response::{IntoResponse, Response};
use crate::errors::infra::InfraError;

#[derive(Debug, Error)]
pub enum GameScoreError {
    #[error("no gamescore exist")]
    NotExist,
    #[error(transparent)]
    Infra(#[from]InfraError),
}
impl IntoResponse for GameScoreError {
    fn into_response(self) -> Response {
        let body = match self {
            GameScoreError::NotExist=> "game score does not exist",
            GameScoreError::Infra(_) => "database error", 

        };

        body.into_response()
    }
}
