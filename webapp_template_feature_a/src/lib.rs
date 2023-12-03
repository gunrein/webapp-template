use axum::routing::get;
use axum::Router;

pub fn base_routes() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello template"
}
