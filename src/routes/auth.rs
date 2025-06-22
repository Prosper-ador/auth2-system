use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::hash_with_salt;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::middleware::auth::Claims;
use crate::models::user::{RegisterRequest, User};
use crate::models::{LoginRequest, LoginResponse, Role};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(LoginRequest, LoginResponse)))]
pub struct AuthApi;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let users = state.users.lock().unwrap();

    //Check if the user exist and the password matches
    let user = users.iter().find(|u| u.email == payload.email);

    if user.is_none()
        || bcrypt::verify(payload.password.as_bytes(), &user.unwrap().password).ok() != Some(true)
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        )
            .into_response();
    }

    let claims = Claims {
        sub: payload.email.clone(),
        role: user.unwrap().role.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let config = state.config.clone();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    return (StatusCode::OK, Json(LoginResponse { token })).into_response();
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = LoginRequest,
    responses(
        (status = 201, description = "User registered successfully", body = LoginResponse),
        (status = 400, description = "Bad request")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // In production, verify against a database
    if payload.email.is_empty()
        || payload.last_name.is_empty()
        || payload.first_name.is_empty()
        || payload.password.is_empty()
        || payload.confirm_password.is_empty()
        {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Username and password are required"})),
        )
            .into_response();
    }
    if payload.password != payload.confirm_password {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Passwords do not match"})),
        )
        .into_response();
    }

    let config = state.config.clone();
    
    // Here you would typically hash the password and save the user to a database
    let hashed_password =
        hash_with_salt(
            payload.password.as_bytes(),
            bcrypt::DEFAULT_COST,
            config.jwt_salt
        ).unwrap();

    let mut users = state.users.lock().unwrap();

    if users.iter().any(|u| u.email == payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User already exists"})),
        )
        .into_response();
    }

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        email: payload.email.clone(),
        first_name: payload.first_name,
        last_name: payload.last_name,
        password: hashed_password.to_string(),
        role: Role::User,
    };

    users.push(new_user);

    // Generate JWT token for the new user
    let claims = Claims {
        sub: payload.email.clone(),
        role: Role::User,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    ).unwrap();

    (
        StatusCode::CREATED,
        Json(LoginResponse { token }),
    )
    .into_response()
}
