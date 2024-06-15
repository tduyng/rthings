use anyhow::Result;
use clap::Parser;
use cli::{interactive_mode, Cli};
use state::load_state;

mod cli;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let state = load_state(&args.url, &args.model).await?;

    interactive_mode(&state).await?;
    Ok(())
}
