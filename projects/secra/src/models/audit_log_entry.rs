use std::net::IpAddr;

use sqlx::{postgres::PgQueryResult, types::ipnetwork::IpNetwork, PgExecutor};
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "audit_log_action", rename_all = "snake_case")]
pub enum AuditLogAction {
    ReadSecret,
    WriteSecret,
}

#[derive(Debug)]
pub struct AuditLogEntry {}

impl AuditLogEntry {
    pub async fn log_action<'e>(
        db: impl PgExecutor<'e>,
        client_addr: IpAddr,
        action: AuditLogAction,
        token: Uuid,
        secret: Uuid,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let client_addr = client_addr.to_canonical();
        let ip_net = match client_addr {
            IpAddr::V4(_) => IpNetwork::new(client_addr, 32),
            IpAddr::V6(_) => IpNetwork::new(client_addr, 128),
        }
        .expect("IP address provided here should always be valid");

        sqlx::query!(
            r#"insert into audit_log (client_addr, action, token, secret) values ($1, $2, $3, $4)"#,
            ip_net,
            action as AuditLogAction,
            token,
            secret,
        )
        .execute(db)
        .await
    }
}
