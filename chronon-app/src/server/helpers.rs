//! Auth / persistence helpers for Chronon server functions.
//!
//! Pure DTO mappers and schedule builders live in [`chronon_backend`].

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
pub(super) use chronon_backend::{
    apply_update_payload_to_job, backend_job_to_job, build_create_job_model, parse_script_params,
};

/// Validates that no existing job already uses the requested name.
#[cfg(feature = "ssr")]
pub(super) async fn ensure_job_name_available(
    backend: &dyn chronon_coordinator::ChrononCoordinatorBackend,
    job_name: &str,
) -> Result<(), ServerFnError> {
    if backend.get_job_by_name(job_name).await.is_some() {
        return Err(ServerFnError::new(format!(
            "A job named '{}' already exists",
            job_name
        )));
    }

    Ok(())
}

/// Loads existing job data (cache-first) for update operations.
#[cfg(feature = "ssr")]
pub(super) async fn load_existing_job_for_update(
    backend: &dyn chronon_coordinator::ChrononCoordinatorBackend,
    actual_job_id: &str,
) -> Result<chronon_coordinator::models::Job, ServerFnError> {
    backend
        .get_job(actual_job_id)
        .await
        .ok_or_else(|| ServerFnError::new(format!("Job {} not found", actual_job_id)))
}

/// Persists updated job config using cache-aware persistence paths.
#[cfg(feature = "ssr")]
pub(super) async fn persist_updated_job_config(
    backend: &dyn chronon_coordinator::ChrononCoordinatorBackend,
    v: &valence::Valence,
    actual_job_id: &str,
    updated_job: chronon_coordinator::models::Job,
) -> Result<(), ServerFnError> {
    backend
        .update_job_config_with_valence(v, actual_job_id, updated_job)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update job: {}", e)))?;
    Ok(())
}
