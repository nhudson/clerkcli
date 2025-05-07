use clap::Parser;
use clerk_rs::{apis::configuration::ClerkConfiguration, clerk::Clerk};
use clerkcli::{
    clerk::users::list_orgs_users,
    cli::types::{Cli, Commands},
};
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

    match cli.command {
        Commands::Users {
            org_id,
            order_by,
            emails_only,
        } => {
            let org_ids: Vec<String> = org_id
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if org_ids.is_empty() {
                error!("No organization IDs provided.");
                std::process::exit(1);
            }
            list_orgs_users(&client, &org_ids, &order_by, emails_only).await;
        }
    }
}
