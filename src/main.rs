use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::{delete, get, post}, Router};
use sea_orm::Database;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::{config::EnvironmentVariables, security::AuthKeys};
mod security;
mod entity;
mod errors;
mod config;
mod repository;
mod service;
mod handlers;
use crate::security::AuthService;
use crate::security::{me_handler, register_handler, login_handler};
use crate::security::require_jwt;
use crate::handlers::friend::{request_friend_handler, delete_friend_request_handler, approve_friend_request_handler};
use crate::repository::FriendRepository;
use crate::service::FriendService;


async fn ping() -> &'static str{
    "pong"
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env = EnvironmentVariables::from_env().unwrap();
    let db = Database::connect(env.database_url.as_ref()).await.unwrap(); 
    let keys = AuthKeys::new(env.secret.as_bytes());
    let auth_service = Arc::new(AuthService::new(db.clone(), keys.clone()));
    let friend_repository = FriendRepository::new(db.clone());
    let friend_service = Arc::new(FriendService::new(friend_repository));

    let auth_router = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .layer(axum::Extension(auth_service.clone()));

    let api_router = Router::new()
        .route("/me", get(me_handler))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));

    let friend_router = Router::new()
        .route("/request", post(request_friend_handler))
        .route("/approve", post(approve_friend_request_handler))
        .route("/delete", delete(delete_friend_request_handler))
        .layer(axum::Extension(friend_service.clone()))
        .layer(from_fn_with_state(keys.clone(), require_jwt));


    let app:Router = Router::new()
        .route("/ping", get(ping))
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .nest("/friend", friend_router)
        .with_state(keys.clone())
        .layer(TraceLayer::new_for_http());
    let port = env.port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
