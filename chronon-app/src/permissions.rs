//! Permission manifest for the Chronon operations app.

use uf_product_macros::UfPermissionManifest;

/// Admin permission for Chronon mutating server functions.
///
/// Synced into the `chronon` domain; gated with
/// `#[uf_product_macros::server(permission = "ChrononAdmin")]`.
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "chronon",
    domain_name = "Chronon",
    domain_description = "Chronon job-scheduling administration"
)]
pub enum ChrononPermission {
    /// Create/update jobs and trigger immediate runs.
    #[permission(description = "Administer Chronon jobs and immediate run triggers")]
    ChrononAdmin,
}
