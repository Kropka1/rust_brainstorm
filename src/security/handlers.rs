use std::sync::Arc;
use serde_json::json;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use crate::security::{Claim, LoginRequest, RegisterRequest};
use crate::security::service::AuthService;
use crate::errors::auth::AuthError;

pub async fn register_handler(
    Extension(auth): Extension<Arc<AuthService>>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<serde_json::Value>){
    match auth.register(payload).await {

        Ok(resp) => {
            match serde_json::to_value(resp){
                Ok(value) => (StatusCode::OK, Json(value)),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "serialize failed"})),
                ),
            }
        },
        Err(err) => map_auth_error(err),
    }
}

pub async fn login_handler(
    Extension(auth): Extension<Arc<AuthService>>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    match auth.login(payload).await {
        Ok(resp) => {
            match serde_json::to_value(resp) {
                Ok(value) => (StatusCode::OK, Json(value)),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "serialize failed"})),
                )
                
            }
        },
        Err(err) => map_auth_error(err),
    }
}


pub async fn me_handler(
    Extension(claim): Extension<Claim>,
) -> impl IntoResponse {
    let body = serde_json::json!({
        "sub": claim.sub,
        "username": claim.username,
        "issued_at": claim.iat,
        "expires_at": claim.exp,
    });
    (StatusCode::OK, Json(body))
}


fn map_auth_error(err: AuthError) -> (StatusCode, Json<serde_json::Value>) {
    use AuthError::*;
    let (code, msg) = match err {
        UserAlreadyExist => (StatusCode::CONFLICT, "user already exists"),
        InvalidRefCode => (StatusCode::BAD_REQUEST, "invalid referral code"),
        InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid credentials"),
        HashingError => (StatusCode::INTERNAL_SERVER_ERROR, "hash error"),
        TokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "token error"),
        DataBaseError => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "unknown error"),
    };

    (code, Json(serde_json::json!({ "error": msg })))
}
