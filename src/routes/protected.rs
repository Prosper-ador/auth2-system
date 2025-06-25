use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
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

/// Registers the protected routes into an Axum Router with required shared state.
pub fn router(
    users: Arc<Mutex<Vec<User>>>,
) -> Router {
    Router::new()
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/register", post(register_admin))
        .route("/user/profile", get(user_profile))
        .layer(Extension(users))
}

#[utoipa::path(
    get,
    path = "/admin/dashboard",
    responses(
        (status = 200, description = "Admin dashboard with user stats", body = UserResponse),
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
    let list: Vec<_> = users_guard
        .iter()
        .map(|u| {
            json!({
                "id": u.id,
                "email": u.email,
                "first_name": u.first_name,
                "last_name": u.last_name,
                "role": format!("{:?}", u.role),
            })
        })
        .collect();

    let payload = json!({
        "user_count": users_guard.len(),
        "users": list,
    });

    Ok((StatusCode::OK, Json(payload)))
}

#[utoipa::path(
    post,
    path = "/admin/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Admin user created", body = UserResponse),
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

    let response = json!({
        "id": new_admin.id,
        "email": new_admin.email,
        "first_name": new_admin.first_name,
        "last_name": new_admin.last_name,
        "role": format!("{:?}", new_admin.role),
    });

    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/user/profile",
    responses(
        (status = 200, description = "User profile info", body = UserResponse),
        (status = 403, description = "Forbidden - User access required"),
        (status = 404, description = "Not Found - User not found")
    )
)]

/// GET /user/profile
/// Returns the authenticated user’s profile info — only accessible by Users.
pub async fn user_profile(
    Extension(claims): Extension<Arc<Claims>>,
    Extension(users): Extension<Arc<Mutex<Vec<User>>>>,
) -> impl IntoResponse {
    if claims.role != Role::User {
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "User access required" }))));
    }

    let users_guard = users.lock().unwrap();
    match users_guard.iter().find(|u| u.email == claims.sub) {
        Some(u) => {
            let profile = json!({
                "id": u.id,
                "email": u.email,
                "first_name": u.first_name,
                "last_name": u.last_name,
                "role": format!("{:?}", u.role),
            });
            Ok((StatusCode::OK, Json(profile)))
        }
        None => Err((StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))),
    }
}
