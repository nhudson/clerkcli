use crate::clerk::orgs::verify_organization;
use clerk_rs::{apis::users_api::User, clerk::Clerk};
use tabled::{Table, Tabled, settings::Style};
use tracing::{debug, error, info};

/// Represents a row in the user output table, containing organization name, user name, and email.
#[derive(Tabled)]
struct UserRow {
    org_name: String,
    name: String,
    email: String,
}

/// Fetches a user's name and email by user ID and organization name, returning a UserRow for table or email output.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client.
/// * `org_name` - Name of the organization the user belongs to.
/// * `user_id` - The ID of the user to fetch.
///
/// # Returns
/// * `UserRow` containing the organization name, user name, and email address (or error placeholders if not found).
async fn get_user_name_and_email(client: &Clerk, org_name: &str, user_id: &str) -> UserRow {
    match User::get_user(client, user_id).await {
        Ok(user_detail) => {
            let first_name_opt = user_detail.first_name.flatten();
            let name = first_name_opt.as_deref().unwrap_or("<no name>").to_string();
            let email = user_detail
                .email_addresses
                .as_ref()
                .and_then(|emails| emails.first())
                .map(|e| e.email_address.clone())
                .unwrap_or_else(|| "<no email>".to_string());
            UserRow {
                org_name: org_name.to_string(),
                name,
                email,
            }
        }
        Err(_) => UserRow {
            org_name: org_name.to_string(),
            name: "<error fetching name>".to_string(),
            email: "<error>".to_string(),
        },
    }
}

/// Fetches all users for a given organization, ordered by the specified field.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client.
/// * `org_id` - The ID of the organization whose users to fetch.
/// * `order_by` - Field to order users by (e.g., created_at, email_address, etc.).
///
/// # Returns
/// * `Vec<User>` containing all users in the organization, or an empty vector on error.
async fn fetch_org_users(
    client: &Clerk,
    org_id: &str,
    order_by: &str,
) -> Vec<clerk_rs::models::user::User> {
    debug!("Calling Clerk API for users in org {}...", org_id);
    match User::get_user_list(
        client,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(vec![org_id.to_string()]),
        None,
        None,
        None,
        Some(order_by),
    )
    .await
    {
        Ok(users) => users,
        Err(e) => {
            error!("Error fetching users for org {}: {e}", org_id);
            Vec::new()
        }
    }
}

/// Builds a vector of UserRow structs for all users in an organization, fetching each user's name and email.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client.
/// * `org_name` - Name of the organization.
/// * `users` - Vector of user models to process.
///
/// # Returns
/// * `Vec<UserRow>` containing a row for each user in the organization.
async fn build_user_rows(
    client: &Clerk,
    org_name: &str,
    users: Vec<clerk_rs::models::user::User>,
) -> Vec<UserRow> {
    let mut rows = Vec::new();
    for user in users {
        let user_id = match &user.id {
            Some(uid) => uid,
            None => {
                rows.push(UserRow {
                    org_name: org_name.to_string(),
                    name: "<unknown user id>".to_string(),
                    email: "<no email>".to_string(),
                });
                continue;
            }
        };
        let row = get_user_name_and_email(client, org_name, user_id).await;
        rows.push(row);
    }
    rows
}

/// Lists users for one or more organizations, printing either a table or just email addresses depending on the emails_only flag.
///
/// For each organization, verifies the org, fetches users, builds user rows, and prints the result.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client.
/// * `org_ids` - Slice of organization IDs to list users for.
/// * `order_by` - Field to order users by.
/// * `emails_only` - If true, only print email addresses; otherwise, print a formatted table.
///
/// Prints results to stdout.
pub async fn list_orgs_users(
    client: &Clerk,
    org_ids: &[String],
    order_by: &str,
    emails_only: bool,
) {
    let mut all_rows = Vec::new();
    for org_id in org_ids {
        // Verify and fetch org
        let org = match verify_organization(client, org_id).await {
            Some(org) => org,
            None => continue,
        };
        let org_name = org.name.clone();
        // Fetch users
        let users = fetch_org_users(client, org_id, order_by).await;
        debug!("Received {} users for org {}.", users.len(), org_name);
        if users.is_empty() {
            info!("No users found for this organization: {}.", org_name);
            continue;
        }
        // Build user rows
        let rows = build_user_rows(client, &org_name, users).await;
        all_rows.extend(rows);
    }
    if all_rows.is_empty() {
        info!("No users found for any organization.");
        return;
    }
    if emails_only {
        for row in all_rows {
            println!("{}", row.email);
        }
    } else {
        let mut table = Table::new(all_rows);
        table.with(Style::modern());
        println!("{}", table);
    }
}
