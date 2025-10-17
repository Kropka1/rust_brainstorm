use axum::{extract::{State, Request}, http::{StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, Validation, decode};
use crate::security::{AuthKeys, Claim};

fn get_user_data(
    req: &Request,
    keys: &AuthKeys
) -> Result<jsonwebtoken::TokenData<Claim>, StatusCode>{
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
    Ok(data)
}

pub async fn require_jwt(
    State(keys): State<AuthKeys>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>{

    let data: jsonwebtoken::TokenData<Claim> = get_user_data(&req, &keys)?;
    req.extensions_mut().insert::<Claim>(data.claims);
    Ok(next.run(req).await)
}


pub async fn require_admin_jwt(
    State(keys): State<AuthKeys>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>{
    let token = get_user_data(&req, &keys)?;
    match token.claims.is_admin{
        1 =>{
              req.extensions_mut().insert::<Claim>(token.claims);
              Ok(next.run(req).await)
        }
        _ => Err(StatusCode::FORBIDDEN)
    }
}

