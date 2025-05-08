use clap::{Parser, Subcommand};

/// CLI argument parser for the clerkcli application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Clerk secret key (overrides CLERK_SECRET_KEY env var)
    #[arg(long, global = true)]
    pub secret_key: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands for the clerkcli application.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Operations related to users
    #[command(alias = "u")]
    Users {
        #[command(subcommand)]
        command: UsersCommand,
    },
    // Placeholder for future subcommands, e.g. Organizations
}

/// Subcommands for user-related operations in the CLI.
///
/// This enum defines all subcommands available under the 'users' top-level command,
/// such as listing users. Additional user-related subcommands can be added here in the future.
#[derive(Subcommand, Debug)]
pub enum UsersCommand {
    /// List users in a Clerk organization
    #[command(alias = "l")]
    List {
        /// Organization ID to query
        #[arg(long, value_name = "ORG_ID", help = "Organization ID to query")]
        org_id: String,
        /// Order users by a field. Options: created_at, updated_at, email_address, web3wallet, first_name, last_name, phone_number, username, last_active_at, last_sign_in_at. Use + or - prefix for ascending/descending. Default: last_sign_in_at
        #[arg(
            long,
            value_name = "ORDER_BY",
            default_value = "last_sign_in_at",
            help = "Order users by a field. Options: created_at, updated_at, email_address, web3wallet, first_name, last_name, phone_number, username, last_active_at, last_sign_in_at. Use + or - prefix for ascending/descending. Default: last_sign_in_at"
        )]
        order_by: String,
        /// Only print email addresses, one per line
        #[arg(long, help = "Only print email addresses, one per line")]
        emails_only: bool,
    },
}
