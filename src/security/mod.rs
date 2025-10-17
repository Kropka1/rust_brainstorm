use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey};
mod service;
mod middleware;
mod handlers;
pub use service::AuthService;
pub use handlers::{login_handler, register_handler, me_handler};
pub use middleware::{require_jwt, require_admin_jwt};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claim{
    pub sub: i64,
    pub exp: usize,
    pub iat: usize,
    pub username: String,
    pub is_admin: i8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest{
    pub username: String,
    pub password: String,
    pub referer_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse{
    pub token: String,
    pub user: UserResponse,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse{
    pub id: String,
    pub username: String,
}
#[derive(Clone)]
pub struct AuthKeys{
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}
impl AuthKeys{
    pub fn new(secret: &[u8]) -> AuthKeys{
        Self{
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
