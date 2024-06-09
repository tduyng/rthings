use clap::Parser;
use secra::{routes::build_router, Cli, ServerState};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let db_pool = get_db_pool(cli.database_url).await?;
    sqlx::migrate!().run(&db_pool).await?;

    let router = build_router(ServerState { db_pool });
    let listener = TcpListener::bind(cli.listen_addr).await?;

    info!("Server listening on {}...", cli.listen_addr);
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

pub async fn get_db_pool(connect_options: PgConnectOptions) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(
            num_cpus::get()
                .try_into()
                .expect("number of CPU cores should fit into an u32"),
        )
        .connect_with(connect_options)
        .await
}

async fn shutdown_signal() {
    let sigint = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("creating SIGINT handler should not fail")
            .recv()
            .await;
    };

    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("creating SIGTERM handler should not fail")
            .recv()
            .await;
    };

    tokio::select! {
        () = sigint => {},
        () = sigterm => {},
    }
}
