use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use webapp_template_feature_a::register_feature;
use webapp_template_web::{create_template_env, Server};

// FIXME
// #[derive(Clone)]
// struct AppState {
//     template_env: &mut Environment
// }

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();


    let mut env = create_template_env();

    let server = Server {
        address: "0.0.0.0:3000",
        routers: vec![register_feature(&mut env)],
    };
    server.start().await
}
