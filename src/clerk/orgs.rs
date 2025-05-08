use clerk_rs::{apis::organizations_api::Organization, clerk::Clerk};
use tracing::{debug, error};

/// Attempts to fetch and validate an organization by its ID using the provided Clerk client.
///
/// # Arguments
/// * `client` - Reference to the initialized Clerk API client.
/// * `org_id` - The ID of the organization to verify.
///
/// # Returns
/// * `Some(Organization)` if the organization exists and is accessible.
/// * `None` if the organization does not exist or is inaccessible (e.g., due to permissions or invalid ID).
///
/// Logs debug information on success and error details on failure.
pub async fn verify_organization(
    client: &Clerk,
    org_id: &str,
) -> Option<clerk_rs::models::organization::Organization> {
    debug!("Verifying organization ID: {}", org_id);
    match Organization::get_organization(client, org_id).await {
        Ok(org) => {
            debug!("Organization found: {} (ID: {})", org.name, org.id);
            Some(org)
        }
        Err(e) => {
            error!("Invalid or inaccessible organization ID {}: {e}", org_id);
            None
        }
    }
}
