use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;

pub mod models;
pub mod routes;
pub mod middleware;

use crate::{routes::{auth, protected, register, user_route}, middleware::auth::auth_middleware};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        info(title = "Auth API", description = "A simple auth API"),
        paths(
            auth::login,
            protected::admin_route
        ),
        components(schemas(
            models::User,
            models::Role,
            models::LoginRequest,
            models::LoginResponse
        ))
    )]
    struct ApiDoc;

    use crate::middleware::auth::auth_middleware;

    // let protected_routes = Router::new()
    //     .route("/admin", get(protected::admin_route))
    //     .route("/user", get(user_route::user_only))
    //     .layer(axum::middleware::from_fn(auth_middleware));

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/login", post(auth::login))
        .route("/register", post(register::register))
        .route("/admin", get(protected::admin_route))
        .route("/user", get(user_route::user_only))
        .layer(axum::middleware::from_fn(auth_middleware))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
