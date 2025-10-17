use serde::{Deserialize, Serialize};
use axum::{extract::Extension, http::StatusCode, Json};
use crate::security::{Claim};
use crate::service::InviteCodeService;
use crate::errors::invitecode::InviteCodeError;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeRequest{
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeReloadRequest{
    pub code: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResponse{
    pub code: String,
    pub is_used: i8,
}


pub async fn create_code_handler(
    Extension(service): Extension<Arc<InviteCodeService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<CodeRequest>,
) -> (StatusCode, Result<Json<CodeResponse>, InviteCodeError>){
    match <InviteCodeService as Clone>::clone(&service).create_new_code(payload.user_id).await{
        Ok(mut resp) => {(
                StatusCode::OK,
                Ok(axum::Json(
                CodeResponse{
                    code: resp.code.take().unwrap(),
                    is_used: resp.is_used.take().unwrap(),
                }
                ))
                
            )
        },
        Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
    }
}


pub async fn reload_code_handler(
    Extension(service): Extension<Arc<InviteCodeService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<CodeReloadRequest>,
) -> (StatusCode, Result<Json<CodeResponse>, InviteCodeError>){
    match <InviteCodeService as Clone>::clone(&service).reload_code(payload.code).await{
        Ok(mut resp) => {(
                StatusCode::OK,
                Ok(axum::Json(
                CodeResponse{
                    code: resp.code.take().unwrap(),
                    is_used: resp.is_used.take().unwrap(),
                }
                ))
                
            )
        },
        Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
    }
}
