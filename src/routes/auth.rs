use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use utoipa::{OpenApi};
use bcrypt::hash_with_salt;

use crate::middleware::auth::Claims;
use crate::models::user::{RegisterRequest, RegisterResponse, User, UserResponse};
use crate::models::{LoginRequest, LoginResponse, Role};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(paths(login, register), components(schemas(LoginRequest, LoginResponse, RegisterRequest)))]
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

    let user = user.unwrap();
    let claims = Claims {
        sub: payload.email.clone(),
        role: user.role.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let config = state.config.clone();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    let response = LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        message: "Login successful".to_string(),
        user: UserResponse {
            id: user.id,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            role: user.role.clone(),
        },
        
    };

    return (StatusCode::OK, Json(response)).into_response();
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
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
    let hashed_password = bcrypt::hash_with_salt(payload.password, bcrypt::DEFAULT_COST, config.jwt_salt).unwrap();

    let mut users = state.users.lock().unwrap();

    if users.iter().any(|u| u.email == payload.email) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "User already exists"})),
        )
        .into_response();
    }

    let first_name = payload.first_name.clone();
    let last_name = payload.last_name.clone();

    let new_user = User {
        id: users.len() as i32 + 1,
        email: payload.email.clone(),
        first_name: first_name.clone(),
        last_name: last_name.clone(),
        password: hashed_password.to_string(),
        role: Role::User,
    };

    users.push(new_user.clone());

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

    let response = LoginResponse {
        access_token: token.clone(),
        message: "User registered successfully".to_string(),
        token_type: "Bearer".to_string(),
        user: UserResponse {
            id: new_user.id,
            email: new_user.email.clone(),
            first_name: new_user.first_name.clone(),
            last_name: new_user.last_name.clone(),
            role: new_user.role.clone(),
        },
    };

    (
        StatusCode::CREATED,
        Json(response),
    )
    .into_response()
}
