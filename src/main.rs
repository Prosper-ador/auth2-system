use std::sync::{Arc, Mutex};

use axum::{
    routing::{post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<utils::Config>,
    pub users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Auth API",
            description = "A simple auth API",
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
        )
    )]
    struct ApiDoc;

let state = AppState {
    config: Arc::new(load_env()),
    users: Arc::new(Mutex::new(vec![User {
        id: 1,
        email: "user@example.com".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        password: bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap(),
        role: Role::User,
    }])),
};

let auth_router = Router::new()
    .route("/login", post(auth::login))
    .route("/register", post(auth::register))
    .with_state(state.clone());

let app = Router::new()  // e.g., /auth/login
    .nest("/auth", auth_router)
    .route("/admin/dashboard", get(admin_dashboard))
    .route("/admin/register", post(register_admin))
    .route("/user/profile", get(user_profile))
    .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware))
    .layer(axum::Extension(state.users.clone()))
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .layer(CorsLayer::permissive())
    .with_state(state);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await.unwrap();
}
