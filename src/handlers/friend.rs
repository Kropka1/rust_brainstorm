use sea_orm::DeleteResult;
use serde::{Deserialize, Serialize};
use axum::{extract::Extension, http::StatusCode, Json};
use crate::security::{Claim};
use crate::service::FriendService;
use crate::errors::friends::FriendError;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendRequest{
    pub reciever_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendResponse{
    pub reciever_id: i64,
    pub status: String,
}
pub async fn request_friend_handler(
    Extension(service): Extension<Arc<FriendService>>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<FriendRequest>,
) -> (StatusCode, Result<Json<FriendResponse>, FriendError>){
    match <FriendService as Clone>::clone(&service).send_friend_request(claim.sub, payload.reciever_id).await{
        Ok(mut resp) => {(
                StatusCode::OK,
                Ok(axum::Json(
                FriendResponse{
                    reciever_id: resp.user_reciever.take().unwrap(),
                    status: resp.status.take().unwrap(),
                }
                ))
                
            )
        },
        Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
    }
}

pub async fn approve_friend_request_handler(
    Extension(service): Extension<Arc<FriendService>>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<FriendRequest>,
) -> (StatusCode, Result<Json<FriendResponse>, FriendError>){
    match <FriendService as Clone>::clone(&service).approve_friend_request(claim.sub, payload.reciever_id).await{
        Ok(mut resp) => {(
                StatusCode::OK,
                Ok(axum::Json(
                FriendResponse{
                    reciever_id: resp.user_reciever.take().unwrap(),
                    status: resp.status.take().unwrap(),
                }
                ))
                
            )
        },
        Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
    }
 
    }


pub async fn delete_friend_request_handler(
    Extension(service): Extension<Arc<FriendService>>,
    Extension(claim): Extension<Claim>,
    Json(payload): Json<FriendRequest>,
) -> (StatusCode, Result<Json<u64>, FriendError>){
    match <FriendService as Clone>::clone(&service).delete_friend_request(claim.sub, payload.reciever_id).await{
        Ok(resp) => {(
                StatusCode::OK,
                Ok(axum::Json(resp.rows_affected))
        )
        },
        Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
    }
 
    }




