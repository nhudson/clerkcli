use clerk_rs::{apis::organizations_api::Organization, clerk::Clerk};
use tracing::{debug, error};

/// Verifies that the given organization ID is valid and accessible.
/// Returns the Organization struct if found, or None if not found or inaccessible.
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
