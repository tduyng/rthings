use crate::{
    errors::ResponseError,
    models::{audit_log_entry::AuditLogAction, token::ExtractValidToken, AuditLogEntry, Secret},
    ServerState,
};
use axum::{
    body::{Body, Bytes},
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use tracing::warn;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn get_secret(
    State(state): State<ServerState>,
    Path(uuid): Path<Uuid>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    ExtractValidToken(token): ExtractValidToken,
) -> Result<Response, ResponseError> {
    if !token.can_read_secret(&state.database, uuid).await? {
        warn!(
            "token=`{}` not allowed to read secret=`{}`",
            token.uuid, uuid
        );
        return Err(ResponseError::Unauthorized());
    }

    let secret = Secret::find(&state.database, uuid).await?;
    let _ = AuditLogEntry::log_action(
        &state.database,
        client_addr.ip(),
        AuditLogAction::SecretRead,
        token.uuid,
        secret.uuid,
    )
    .await?;

    Ok(secret.into_response())
}

#[axum::debug_handler]
pub async fn update_secret(
    State(state): State<ServerState>,
    Path(uuid): Path<Uuid>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    ExtractValidToken(token): ExtractValidToken,
    body: Bytes,
) -> Result<Response, ResponseError> {
    if !token.can_write_secret(&state.database, uuid).await? {
        warn!(
            "token=`{}` not allowed to write secret=`{}`",
            token.uuid, uuid
        );
        return Err(ResponseError::Unauthorized());
    }

    let mut secret = Secret::find(&state.database, uuid).await?;
    let _ = AuditLogEntry::log_action(
        &state.database,
        client_addr.ip(),
        AuditLogAction::SecretWrite,
        token.uuid,
        secret.uuid,
    )
    .await?;

    let _ = secret
        .update_contents(&state.database, body.to_vec())
        .await?;

    Ok((StatusCode::NO_CONTENT, Body::empty()).into_response())
}
