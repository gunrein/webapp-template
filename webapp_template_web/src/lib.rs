use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start_server(addr: &str) {
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    info!("Starting server at {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello template"
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("hello");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
