use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
    Extension,
    Json
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use chrono;

pub mod middleware;
pub mod models;
pub mod routes;
pub mod utils;

use crate::{
    middleware::auth::auth_middleware,
    routes::{auth, protected},
    utils::load_env,
    models::*,
};

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy", "timestamp": chrono::Utc::now()})))
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<utils::Config>,
    pub users: Arc<Mutex<Vec<User>>>,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Auth API",
        description = "A secure authentication and authorization API with JWT tokens and role-based access control. Protected endpoints require Bearer token authentication.",
        version = "1.0.0",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    paths(
        auth::login,
        auth::register,
        protected::admin_dashboard,
        protected::register_admin,
        protected::user_profile
    ),
    components(
        schemas(
            User,
            UserResponse,
            Role,
            LoginRequest,
            LoginResponse,
            RegisterRequest,
            RegisterResponse
        )
    ),
    security(
        ("bearer_auth" = [])
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "protected", description = "Protected endpoints requiring Bearer token authentication")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Auth API server...");

    // Load configuration
    let config = match std::panic::catch_unwind(|| load_env()) {
        Ok(config) => {
            tracing::info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            tracing::error!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    // Initialize application state
    let state = AppState {
        config: Arc::new(config),
        users: Arc::new(Mutex::new(vec![
            User {
                id: 1,
                email: "admin@example.com".to_string(),
                first_name: "Nyengka".to_string(),
                last_name: "Prosper".to_string(),
                password: bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap(),
                role: Role::Admin,
            },
            User {
                id: 2,
                email: "user@example.com".to_string(),
                first_name: "King".to_string(),
                last_name: "Joshua".to_string(),
                password: bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap(),
                role: Role::User,
            }
        ])),
    };

    // Create public router (no auth required)
    let public_router = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
        .with_state(state.clone());

    // Create protected router (auth required)
    let protected_router = Router::new()
        .route("/admin/dashboard", get(protected::admin_dashboard))
        .route("/admin/register", post(protected::register_admin))
        .route("/user/profile", get(protected::user_profile))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .layer(Extension(state.users.clone()))
        .with_state(state.clone());

    // Build the main application
    let app = Router::new()
        .merge(public_router)  // Public routes at root level
        .merge(protected_router)  // Protected routes at root level
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start the server
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:{}", host, port);
    
    tracing::info!("Attempting to bind to {}", addr);
    
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            tracing::info!("Successfully bound to {}", addr);
            listener
        }
        Err(e) => {
            tracing::error!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };
    
    tracing::info!("Server listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);
    tracing::info!("OpenAPI spec available at http://{}/api-docs/openapi.json", addr);
    
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}
