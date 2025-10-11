use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::{get, post}, Router};
use sea_orm::Database;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::{config::EnvironmentVariables, security::AuthKeys};
mod security;
mod entity;
mod errors;
mod config;
use crate::security::AuthService;
use crate::security::{me_handler, register_handler, login_handler};
use crate::security::require_jwt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env = EnvironmentVariables::from_env().unwrap();
    let db = Database::connect(env.database_url.as_ref()).await.unwrap(); 
    let keys = AuthKeys::new(env.secret.as_bytes());
    let auth_service = Arc::new(AuthService::new(db, keys.clone()));

    let auth_router = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .layer(axum::Extension(auth_service.clone()));

    let api_router = Router::new()
        .route("/me", get(me_handler))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));


    let app:Router = Router::new()
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .with_state(keys.clone())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
