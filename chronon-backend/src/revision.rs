//! Job revision redaction for client-facing server responses.
//!
//! Mirrors upstream Chronon HTTP behavior: full snapshots remain in the store;
//! wire responses omit actor identity and sensitive snapshot fields.

use crate::types::JobRevision;

/// Strips sensitive fields from a revision snapshot before returning it to clients.
///
/// Nulls `actor_json` and `params_json` while preserving structural fields such as
/// `job_name`, `cron_expr`, and `timezone` needed by the job-detail revision picker.
#[must_use]
pub fn redact_revision_snapshot(mut snapshot: serde_json::Value) -> serde_json::Value {
    if let Some(obj) = snapshot.as_object_mut() {
        obj.insert("actor_json".into(), serde_json::Value::Null);
        obj.insert("params_json".into(), serde_json::Value::Null);
    }
    snapshot
}

/// Applies client-safe redaction to a [`JobRevision`] DTO.
#[must_use]
pub fn redact_job_revision(mut revision: JobRevision) -> JobRevision {
    revision.changed_by_actor_json = serde_json::Value::Null;
    revision.snapshot_json = redact_revision_snapshot(revision.snapshot_json);
    revision
}

#[cfg(test)]
mod tests {
    use super::{redact_job_revision, redact_revision_snapshot};
    use crate::types::JobRevision;

    #[test]
    fn redact_revision_snapshot_nulls_actor_and_params() {
        let snapshot = serde_json::json!({
            "job_name": "daily-sync",
            "cron_expr": "0 * * * *",
            "actor_json": {"role": "admin", "session": "sess-1"},
            "params_json": {"token": "super-secret"},
        });
        let redacted = redact_revision_snapshot(snapshot);
        assert!(redacted.get("actor_json").unwrap().is_null());
        assert!(redacted.get("params_json").unwrap().is_null());
        assert_eq!(
            redacted.get("job_name").and_then(|v| v.as_str()),
            Some("daily-sync")
        );
    }

    #[test]
    fn redact_job_revision_nulls_changed_by_actor_json() {
        let revision = JobRevision {
            revision_id: "rev-1".into(),
            revision_number: 2,
            changed_at: "2026-01-01T00:00:00Z".into(),
            changed_by_actor_json: serde_json::json!({"user_id": "u1"}),
            snapshot_json: serde_json::json!({
                "job_name": "daily-sync",
                "params_json": {"token": "secret"},
            }),
        };
        let redacted = redact_job_revision(revision);
        assert!(redacted.changed_by_actor_json.is_null());
        assert!(redacted.snapshot_json.get("params_json").unwrap().is_null());
        assert_eq!(redacted.revision_number, 2);
    }
}
