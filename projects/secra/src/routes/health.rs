use crate::{errors::ResponseError, ServerState};
use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[axum::debug_handler]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, Body::empty())
}

#[axum::debug_handler]
pub async fn db_ready(State(state): State<ServerState>) -> Result<Response, ResponseError> {
    let _ = sqlx::query!(
        r#"select
        now() as test_timestamp,
        uuid_generate_v4() as test_uuid,
        encode(gen_random_bytes(1), 'hex') as random_byte;"#
    )
    .fetch_all(&state.db_pool)
    .await?;

    Ok((StatusCode::NO_CONTENT, Body::empty()).into_response())
}
