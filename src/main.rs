use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::{delete, get, patch, post}, Router};
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
use tower_http::{services::ServeDir};
use tower_http::limit::RequestBodyLimitLayer;
use crate::security::AuthService;
use crate::security::{me_handler, register_handler, login_handler};
use crate::security::{require_jwt, require_admin_jwt};
use crate::handlers::invitecode::{create_code_handler, reload_code_handler};
use crate::handlers::gamescore::{create_score_handler, update_score_handler, get_user_scores_handler, get_all_scores_handler};
use crate::handlers::friend::{request_friend_handler, delete_friend_request_handler, approve_friend_request_handler};
use crate::handlers::user::{get_user_handler,
change_role_handler,
change_username_handler,
change_logo_handler,
}; 
use crate::errors::auth::AuthError;
use crate::repository::{FriendRepository, UserRepository, InviteCodeRepository, GameScoreRepository};
use crate::service::{FriendService, InviteCodeService, GameScoreService, UserService};


async fn ping() -> &'static str{
    "pong"
}


async fn create_default_admin(
    auth: Arc<AuthService>,
    username: &str,
    password: &str,
) -> Result<(), AuthError> {
    auth.create_super_admin(username.to_string(), password.to_string()).await
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
    let invitecode_repository = InviteCodeRepository::new(db.clone());
    let invitecode_service = Arc::new(InviteCodeService::new(invitecode_repository));
    let gamescore_repository = GameScoreRepository::new(db.clone());
    let gamescore_service = Arc::new(GameScoreService::new(gamescore_repository));
    let user_repository = UserRepository::new(db.clone());
    let user_service = Arc::new(UserService::new(user_repository));

    if let Err(e) = create_default_admin(
        auth_service.clone(),
        &env.default_admin_username,
        &env.default_admin_password,
    ).await {
        tracing::warn!("default admin init failed (maybe exists already): {e:?}");
    }

    let auth_router = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .layer(axum::Extension(auth_service.clone()));

    let api_router = Router::new()
        .route("/me", get(me_handler))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));

    let user_router = Router::new()
        .route("/get", get(get_user_handler))
        .route("/patch_username", patch(change_username_handler))
        .route("/patch_logo", patch(change_logo_handler))
        .layer(axum::Extension(user_service.clone()))
        .layer(RequestBodyLimitLayer::new(8 * 1024 * 1024))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));

    let admin_router = Router::new()
        .route("/patch_role", patch(change_role_handler))
        .layer(axum::Extension(user_service.clone()))
        .route_layer(from_fn_with_state(keys.clone(), require_admin_jwt));
    
    
    let files = Router::new().nest_service("/images", ServeDir::new("./uploads"));

        
    let invitecode_router = Router::new()
        .route("/new", post(create_code_handler))
        .route("/reload", post(reload_code_handler))
        .layer(axum::Extension(invitecode_service.clone()))
        .route_layer(from_fn_with_state(keys.clone(), require_admin_jwt));

    let friend_router = Router::new()
        .route("/request", post(request_friend_handler))
        .route("/approve", post(approve_friend_request_handler))
        .route("/delete", delete(delete_friend_request_handler))
        .layer(axum::Extension(friend_service.clone()))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));

    let gamescore_router = Router::new()
        .route("/create", post(create_score_handler))
        .route("/update", patch(update_score_handler))
        .route("/get_user_scores", get(get_user_scores_handler))
        .route("/get_all", get(get_all_scores_handler))
        .layer(axum::Extension(gamescore_service.clone()))
        .route_layer(from_fn_with_state(keys.clone(), require_jwt));


    let app:Router = Router::new()
        .route("/ping", get(ping))
        .nest("/auth", auth_router)
        .nest("/api", api_router)
        .nest("/friend", friend_router)
        .nest("/invitecode", invitecode_router)
        .nest("/gamescore", gamescore_router)
        .nest("/user", user_router)
        .nest("/admin", admin_router)
        .with_state(keys.clone())
        .merge(files)
        .layer(TraceLayer::new_for_http());
    let port = env.port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
