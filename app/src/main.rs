use web::start_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    start_server().await
}