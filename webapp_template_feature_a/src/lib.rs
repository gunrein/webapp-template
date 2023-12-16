use axum::routing::get;
use axum::Router;
use minijinja::{context, Environment};

pub fn register_feature(template_env: &mut Environment) -> Router {
    template_env.add_template("webapp_template_feature_a:hello", "Hello {{ name }}!").expect("FIXME with a Result ?");
    Router::new().route("/", get(root))
}

async fn root() -> String {
    let mut env = Environment::new();
    env.add_template("hello", "Hello {{ name }}!").unwrap();
    let tmpl = env.get_template("hello").unwrap();
    tmpl.render(context!(name => "mini jinja world")).unwrap()
}
