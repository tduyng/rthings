use crate::ServerState;
use axum::{routing::get, Router};
use health::{db_ready, health_check};

pub mod health;

pub fn build_router(state: ServerState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(db_ready))
        .with_state(state)
}
