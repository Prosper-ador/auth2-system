use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use crate::models::{Role, User};
use serde_json::json;

pub async fn user_only(Extension(user): Extension<Arc<User>>) -> impl IntoResponse {
    if user.role == Role::User {
        (StatusCode::OK, Json(user)).into_response()
    } else {
        (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "User access required" })),
        ).into_response()
    }
}
