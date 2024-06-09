use crate::ServerState;
use axum::{
    routing::{get, post},
    Router,
};
use health::{db_ready, health_check};
use secret::{get_secret, update_secret};

pub mod health;
pub mod secret;

pub fn build_router(state: ServerState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(db_ready))
        .route("/secret/:uuid", get(get_secret))
        .route("/secret/:uuid/contents", post(update_secret))
        .with_state(state)
}
