use clap::Parser;
use sqlx::postgres::PgConnectOptions;
use std::net::SocketAddr;

pub mod errors;
pub mod models;
pub mod routes;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The database URL to connect to. Needs to be a valid PostgreSQL
    /// connection URL, like `postgres://postgres@127.0.0.1/secra`
    #[clap(long, short, env = "DATABASE_URL")]
    pub database_url: PgConnectOptions,

    /// The Socket Address the server should listen on
    #[clap(long, short, env = "LISTEN_ADDR", default_value = "[::1]:3000")]
    pub listen_addr: SocketAddr,
}

#[derive(Clone)]
pub struct ServerState {
    pub db_pool: sqlx::PgPool,
}
