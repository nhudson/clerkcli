use crate::cli::types::{Commands, UsersCommand};
use clerk_rs::clerk::Clerk;
use tracing::error;

/// Dispatches the parsed CLI command to the appropriate handler.
///
/// This function matches on the top-level and nested subcommands, and calls the corresponding
/// handler functions, passing along the Clerk client and any required arguments.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client
/// * `command` - The parsed CLI command to dispatch
pub async fn dispatch_command(client: &Clerk, command: Commands) {
    match command {
        Commands::Users { command } => match command {
            UsersCommand::List {
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
                crate::clerk::users::list_orgs_users(client, &org_ids, &order_by, emails_only)
                    .await;
            }
        },
    }
}
