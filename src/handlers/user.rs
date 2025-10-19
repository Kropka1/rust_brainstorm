use axum::body::Bytes;
use serde::{Deserialize, Serialize};
use axum::{extract::Extension, http::StatusCode, Json};
use axum::extract::Multipart;
use axum::{http::{header, HeaderMap}};
use crate::entity::user::Role;
use crate::security::Claim;
use crate::service::UserService;
use crate::errors::user::UserError;
use std::sync::Arc;
use axum::{
    extract::Path,
};
use std::path::{PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetRequest{
    user_id: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateUsernameRequest{
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRoleRequest{
    role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse{
    user_id: i64,
    role: String,
    username: String,
    logo: String,
    registration_date: i32,
}

pub async fn get_user_handler(
    Extension(service): Extension<Arc<UserService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<UserGetRequest>,
) -> (
StatusCode, Result<Json<UserResponse>, UserError>
){
    match <UserService as Clone>::clone(&service)
        .get_user(
            payload.user_id,
        ).await{
            Ok(resp) => {
                (
                    StatusCode::OK,
                    Ok(
                        Json(
                            UserResponse {
                                user_id: resp.id,
                                role: resp.role,
                                username: resp.username,
                                logo: resp.logo,
                                registration_date: resp.created_at,
                            }
                        )
                    )
                )
            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }
}

pub async fn change_role_handler(
    Extension(service): Extension<Arc<UserService>>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<UserUpdateRoleRequest>,
) -> (
StatusCode, Result<Json<UserResponse>, UserError>
){
    match <UserService as Clone>::clone(&service)
        .change_role(
            claim.sub,
            payload.role,
        ).await{
            Ok(mut resp) => {
                (
                    StatusCode::OK,
                    Ok(
                        Json(
                            UserResponse {
                                user_id: resp.id.take().unwrap(),
                                role: resp.role.take().unwrap(),
                                username: resp.username.take().unwrap(),
                                logo: resp.logo.take().unwrap(),
                                registration_date: resp.created_at.take().unwrap(),
                            }
                        )
                    )
                )
            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }
}

pub async fn change_username_handler(
    Extension(service): Extension<Arc<UserService>>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<UserUpdateUsernameRequest>,
) -> (
StatusCode, Result<Json<UserResponse>, UserError>
){
    match <UserService as Clone>::clone(&service)
        .update_username(
            claim.sub,
            payload.username,
        ).await{
            Ok(mut resp) => {
                (
                    StatusCode::OK,
                    Ok(
                        Json(
                            UserResponse {
                                user_id: resp.id.take().unwrap(),
                                role: resp.role.take().unwrap(),
                                username: resp.username.take().unwrap(),
                                logo: resp.logo.take().unwrap(),
                                registration_date: resp.created_at.take().unwrap(),
                            }
                        )
                    )
                )
            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }
}

pub async fn change_logo_handler(
    Extension(service): Extension<Arc<UserService>>,
    Extension(claim): Extension<Claim>,
    mut multipart: Multipart,
) -> (
StatusCode, Result<(), UserError>
){
    let mut bytes: Bytes = Default::default();
    while let Ok(Some(field)) = multipart.next_field()
        .await.map_err(|_| UserError::InternalError){
            if field.name() != Some("file"){
                continue;
            }
            bytes = field.bytes().await.map_err(|_| UserError::InternalError)
                .expect("no file");
            break;
    }
    match <UserService as Clone>::clone(&service)
        .upload_file(
            bytes,
            claim.sub,
        ).await{
            Ok(_) => {
                (StatusCode::OK, Ok(()))
            }
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }
}


