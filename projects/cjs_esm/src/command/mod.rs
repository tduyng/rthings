pub mod add_js;

use crate::command::add_js::AddJsArgs;
use clap::{Parser, Subcommand};

/// A CLI program for adding .js to import/export paths in TypeScript projects
#[derive(Parser)]
#[command(
    name = "cesm",
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct CliArgs {
    /// Subcommands to execute specific tasks
    #[command(subcommand)]
    pub cmd: Option<CliCommand>,
}

/// Enumeration of available subcommands
#[derive(Subcommand)]
pub enum CliCommand {
    /// Add .js to import/export paths in TypeScript projects
    #[command(name = "addjs")]
    AddJs(AddJsArgs),
}
