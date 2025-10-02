use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::{HeaderMap, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::security::{AuthKeys, Claim};
use crate::errors;

