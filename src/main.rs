pub mod clerk;
pub mod cli;

use crate::{cli::dispatch::dispatch_command, cli::types::Cli};
use clap::Parser;
use clerk_rs::{apis::configuration::ClerkConfiguration, clerk::Clerk};
use std::env;
use tracing::error;

/// Entry point for the clerkcli CLI application.
/// Initializes logging, parses CLI arguments, sets up the Clerk client, and dispatches to the appropriate subcommand handler.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let secret_key = match cli.secret_key.or_else(|| env::var("CLERK_SECRET_KEY").ok()) {
        Some(val) => val,
        None => {
            error!(
                "Error: The Clerk API key must be provided via --api-key or the CLERK_SECRET_KEY environment variable."
            );
            std::process::exit(1);
        }
    };
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let client = Clerk::new(config);
    dispatch_command(&client, cli.command).await;
}
