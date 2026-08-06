pub mod auth;
pub mod dto;
pub mod error;
pub mod handlers;

use axum::routing::{get, post};
use axum::Router;
use loglens_storage::Database;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dto::*;
use handlers::auth_handlers::*;
use handlers::event_handlers::*;
use handlers::export_handlers::*;
use handlers::group_handlers::*;
use handlers::health::*;
use handlers::source_handlers::*;
use handlers::workspace_handlers::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health::health_handler,
        handlers::health::ready_handler,
    ),
    components(
        schemas(HealthResponse, ProblemDetails, BootstrapAdminRequest, LoginRequest, UserResponse)
    ),
    tags(
        (name = "loglens", description = "LogLens API endpoints")
    )
)]
pub struct ApiDoc;

pub fn create_router(db: Database) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/auth/bootstrap", post(bootstrap_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(me_handler))
        .route("/workspaces", get(list_workspaces_handler).post(create_workspace_handler))
        .route("/sources", get(list_sources_handler))
        .route("/sources/upload", post(upload_source_handler))
        .route("/sources/:id", axum::routing::delete(delete_source_handler))
        .route("/events", get(query_events_handler))
        .route("/events/stream", get(event_sse_stream_handler))
        .route("/groups", get(list_groups_handler))
        .route("/exports", post(export_events_handler));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(db)
}
