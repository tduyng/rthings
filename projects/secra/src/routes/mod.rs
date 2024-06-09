use axum::{response::Html, routing::get, Router};

use crate::ServerState;

pub fn build_router(_state: ServerState) -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
