use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;

use crate::{middleware::auth::Claims, models::UserResponse};
use crate::models::{RegisterRequest, Role, User};
/// Aggregates all protected routes: admin dashboard, admin-only registration, user profile view.
#[derive(OpenApi)]
#[openapi(
    paths(admin_dashboard, register_admin, user_profile),
    components(schemas(User, RegisterRequest, Role, UserResponse)),
)]
pub struct ProtectedApi;

#[utoipa::path(
    get,
    path = "/admin/dashboard",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Admin dashboard with user stats", body = UserResponse),
        (status = 401, description = "Unauthorized - Bearer token required"),
        (status = 403, description = "Forbidden - Admin access required")
    )
)]

/// GET /admin/dashboard
/// Returns system stats and list of users — only accessible by Admins.
pub async fn admin_dashboard(
    Extension(claims): Extension<Arc<Claims>>,
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
) -> impl IntoResponse {
    if claims.role != Role::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "Admin access required"}))));
    }

    let users_guard = users.lock().unwrap();
    let list: Vec<UserResponse> = users_guard
        .iter()
        .map(|u| UserResponse {
            id: u.id,
            email: u.email.clone(),
            first_name: u.first_name.clone(),
            last_name: u.last_name.clone(),
            role: u.role.clone(),
        })
        .collect();

    let payload = json!({
        "user_count": users_guard.len(),
        "users": list,
    });

    Ok((StatusCode::OK, Json(payload)).into_response())
}

#[utoipa::path(
    post,
    path = "/admin/register",
    security(
        ("bearer_auth" = [])
    ),
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Admin user created", body = UserResponse),
        (status = 400, description = "Bad request - Validation error"),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
        (status = 403, description = "Forbidden - Admin access required"),
        (status = 409, description = "Conflict - Email already registered"),
        (status = 500, description = "Internal Server Error - Hash failure")
    )
)]
/// POST /admin/register
/// Allows Admin to create a new Admin user.
pub async fn register_admin(
    Extension(claims): Extension<Arc<Claims>>,
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if claims.role != Role::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Admin access required" }))));
    }

    // Validate input
    if payload.email.is_empty() || payload.first_name.is_empty() || payload.last_name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "All fields are required" }))));
    }

    if payload.password != payload.confirm_password {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Passwords do not match" }))));
    }

    if payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Password must be at least 6 characters" }))));
    }

    let mut users_guard = users.lock().unwrap();
    if users_guard.iter().any(|u| u.email == payload.email) {
        return Err((StatusCode::CONFLICT, Json(json!({ "error": "Email already registered" }))));
    }

    let hashed = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Hash failure" }))))?;

    let new_admin = User {
        id: (users_guard.len() + 1) as i32,
        email: payload.email.clone(),
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        password: hashed,
        role: Role::Admin,
    };
    users_guard.push(new_admin.clone());

    let response = UserResponse {
        id: new_admin.id,
        email: new_admin.email,
        first_name: new_admin.first_name,
        last_name: new_admin.last_name,
        role: new_admin.role,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/user/profile",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User profile info", body = UserResponse),
        (status = 400, description = "Bad request - Invalid user ID"),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
        (status = 403, description = "Forbidden - Authentication required"),
        (status = 404, description = "Not Found - User not found")
    )
)]

/// GET /user/profile
/// Returns the authenticated user's profile info — accessible by both Users and Admins.
pub async fn user_profile(
    Extension(claims): Extension<Arc<Claims>>,
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
) -> impl IntoResponse {
    // Allow both User and Admin roles to access their profile
    if claims.role != Role::User && claims.role != Role::Admin {
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Authentication required" }))));
    }

    let users_guard = users.lock().unwrap();
    let user_id = claims.sub.parse::<i32>().unwrap_or(0);
    
    if user_id == 0 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid user ID" }))));
    }
    
    match users_guard.iter().find(|u| u.id == user_id) {
        Some(u) => {
            let profile = UserResponse {
                id: u.id,
                email: u.email.clone(),
                first_name: u.first_name.clone(),
                last_name: u.last_name.clone(),
                role: u.role.clone(),
            };
            Ok((StatusCode::OK, Json(profile)))
        }
        None => Err((StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))),
    }
}
