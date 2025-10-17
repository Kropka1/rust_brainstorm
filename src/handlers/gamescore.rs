use serde::{Deserialize, Serialize};
use axum::{extract::Extension, http::StatusCode, Json};
use crate::security::Claim;
use crate::service::GameScoreService;
use crate::errors::gamescore::GameScoreError;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]            
pub struct GameScoreCreateRequest{
    user_id: i64,
    game_type: String,
    game_level: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScoreUpdateRequest{
    user_id: i64,
    game_type: String,
    game_level: i8,
    new_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScoreGetRequest{
    user_id: i64,
    game_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScoreGetAllRequest{
    game_type: String,
    game_level: i8,
    limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameScoreRespone{
    game_type: String,
    game_level: i8,
    user_id: i64,
    score: i32,
}


pub async fn create_score_handler(
    Extension(service): Extension<Arc<GameScoreService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<GameScoreCreateRequest>,
) -> (
StatusCode, Result<Json<GameScoreRespone>, GameScoreError>
){
    match <GameScoreService as Clone>::clone(&service)
        .create(
            payload.user_id,
            payload.game_type,
            payload.game_level,
        )
        .await{
            Ok(mut resp) => {
                (
                    StatusCode::OK,
                    Ok(
                    Json(
                        GameScoreRespone{
                            game_type: resp.game_type.take().unwrap(),
                            game_level: resp.game_level.take().unwrap(),
                            user_id: payload.user_id,
                            score: resp.score.take().unwrap(),
                        }
                    )

                )
                )

            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)), 
        }
}

pub async fn update_score_handler(
    Extension(service): Extension<Arc<GameScoreService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<GameScoreUpdateRequest>,
) -> (
StatusCode, Result<Json<GameScoreRespone>, GameScoreError>
){
    match <GameScoreService as Clone>::clone(&service)
        .update(
            payload.user_id,
            payload.game_level,
            payload.game_type,
            payload.new_score,
        )
        .await{
            Ok(mut resp) => {
                (
                    StatusCode::OK,
                    Ok(
                    Json(
                        GameScoreRespone{
                            game_type: resp.game_type.take().unwrap(),
                            game_level: resp.game_level.take().unwrap(),
                            user_id: payload.user_id,
                            score: resp.score.take().unwrap(),
                        }
                    )

                )
                )

            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)), 
        }
}

pub async fn get_user_scores_handler(
    Extension(service): Extension<Arc<GameScoreService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<GameScoreGetRequest>,
) -> (
StatusCode, Result<Json<Vec<GameScoreRespone>>, GameScoreError>
){
    match <GameScoreService as Clone>::clone(&service)
        .get_user_scores(
            payload.game_type,
            payload.user_id
        ).await{
            Ok(resp) => {
                let vec_resp: Vec<GameScoreRespone> = resp
                    .iter()
                    .map(|single_resp| (
                            GameScoreRespone{
                                game_type: single_resp.game_type.clone(),
                                game_level: single_resp.game_level,
                                user_id: single_resp.user_id,
                                score: single_resp.score,
                            }
                    ))
                    .collect();
                (
                    StatusCode::OK,
                    Ok(
                        Json(vec_resp)
                    )
                )
            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }

}

pub async fn get_all_scores_handler(
    Extension(service): Extension<Arc<GameScoreService>>,
    Extension(_claim): Extension<Claim>,
    Json(payload): Json<GameScoreGetAllRequest>,
) -> (
StatusCode, Result<Json<Vec<GameScoreRespone>>, GameScoreError>
){
    match <GameScoreService as Clone>::clone(&service)
        .get_all(
            payload.game_type,
            payload.game_level,
            payload.limit,
        ).await{
            Ok(resp) => {
                let vec_resp: Vec<GameScoreRespone> = resp
                    .iter()
                    .map(|single_resp| (
                            GameScoreRespone{
                                game_type: single_resp.game_type.clone(),
                                game_level: single_resp.game_level,
                                user_id: single_resp.user_id,
                                score: single_resp.score,
                            }
                    ))
                    .collect();
                (
                    StatusCode::OK,
                    Ok(
                        Json(vec_resp)
                    )
                )
            },
            Err(err) => (StatusCode::BAD_REQUEST, Err(err)),
        }

}



