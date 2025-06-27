use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::warn;

use crate::{
    models::Role,
    AppState
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject, typically the user ID, can also be an email
    pub role: Role, // for role-based access control(admin, user, etc.)
    pub exp: usize, // token expiration time as a UNIX timestamp
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());

    let token_data = decode::<Claims>(token, &key, &Validation::default())
        .map_err(|e| {
            warn!("JWT decode error: {:?}", e);
            StatusCode::UNAUTHORIZED
        })?;

    req.extensions_mut().insert(Arc::new(token_data.claims));

    Ok(next.run(req).await)
}
