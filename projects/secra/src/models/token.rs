use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, Executor, PgExecutor};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{errors::ResponseError, ServerState};

#[derive(Debug)]
pub struct Token {
    pub uuid: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    pub token: String,
    pub superuser: bool,
}

impl Token {
    pub async fn try_query_with_token<'e>(
        db: impl Executor<'e>,
        token: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "select uuid, expires_at, token, superuser from tokens where token = $1",
            token
        )
        .fetch_optional(db)
        .await
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at < Utc::now(),
            None => false,
        }
    }

    pub async fn update_used_timestamp<'e>(
        &mut self,
        db: impl Executor<'e>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "update tokens set used_at = $1 where uuid = $2",
            Utc::now(),
            self.uuid
        )
        .execute(db)
        .await
    }

    pub async fn can_read_secret<'e>(
        &self,
        db: impl PgExecutor<'e>,
        secret_uuid: Uuid,
    ) -> Result<bool, sqlx::Error> {
        if self.superuser {
            return Ok(true);
        }

        Ok(sqlx::query!(
            r#"select token from token_permissions where token = $1 and secret = $2 and can_read = true"#,
            self.uuid,
            secret_uuid
        ).fetch_optional(db).await?.is_some())
    }

    pub async fn can_write_secret<'e>(
        &self,
        db: impl PgExecutor<'e>,
        secret_uuid: Uuid,
    ) -> Result<bool, sqlx::Error> {
        if self.superuser {
            return Ok(true);
        }

        Ok(sqlx::query!(
            r#"select token from token_permissions where token = $1 and secret = $2 and can_write = true"#,
            self.uuid,
            secret_uuid
        ).fetch_optional(db).await?.is_some())
    }
}

#[derive(Debug)]
pub struct ExtractValidToken(pub Token);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractValidToken
where
    ServerState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ResponseError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = ServerState::from_ref(state);

        let token_header = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await?
            .0;

        let token = Token::try_query_with_token(&app_state.db_pool, token_header.token()).await?;
        let Some(mut token) = token else {
            info!("use of invalid token=`{}`", token_header.token());
            return Err(Self::Rejection::Unauthorized());
        };

        let _ = token.update_used_timestamp(&app_state.db_pool).await?;

        if token.is_expired() {
            warn!("use of expired token=`{}`", token.uuid);
            return Err(Self::Rejection::Unauthorized());
        }

        Ok(Self(token))
    }
}
