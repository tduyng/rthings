use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Index"
}

async fn weather() -> &'static str {
    "Weather"
}

async fn stats() -> &'static str {
    "Stats"
}

async fn hello() -> &'static str {
    "Helle world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/stats", get(stats))
        .route("/hello", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 2000));
    println!("Server is running at http://localhost:2000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
