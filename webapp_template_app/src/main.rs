use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use webapp_template_web::{base_routes, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server = Server {
        address: "0.0.0.0:3000",
        routers: vec![base_routes()],
    };
    server.start().await
}
