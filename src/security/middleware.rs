use axum::{extract::{State, Request}, http::{StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, Validation, decode};
use crate::security::{AuthKeys, Claim};



pub async fn require_jwt(
    State(keys): State<AuthKeys>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>{
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let data = decode::<Claim>(token, &keys.decoding, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert::<Claim>(data.claims);
    Ok(next.run(req).await)
}

