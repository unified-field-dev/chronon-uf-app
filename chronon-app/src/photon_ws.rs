//! Standalone export stub for the template Photon WebSocket route.
//!
//! Photon Axum wiring is intentionally absent from this repository.

use axum::Router;

/// Path prefix for per-run live WebSocket connections, kept stable for host wiring.
pub const CHRONON_JOB_RUN_WS_PREFIX: &str = "/ws/chronon-job";

/// Preserve the host integration point while omitting Photon Axum wiring.
pub const fn merge_routes<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router
}
