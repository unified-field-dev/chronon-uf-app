//! Display helpers for job-detail snapshot and form rendering.

/// Keeps job params JSON stable for form/edit flows.
pub fn normalized_params(value: &serde_json::Value) -> serde_json::Value {
    if value.is_null() {
        serde_json::json!({})
    } else {
        value.clone()
    }
}

/// Serializes JSON for textarea editing with a safe fallback.
pub fn pretty_json(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
}

/// Reads a string field from a revision snapshot payload.
pub fn snapshot_string(snapshot: &serde_json::Value, key: &str) -> Option<String> {
    snapshot
        .get(key)
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

/// Reads params payload from a revision snapshot.
pub fn snapshot_params(snapshot: &serde_json::Value) -> Option<serde_json::Value> {
    snapshot.get("params_json").cloned()
}

/// Provides Chronon default timezone label for empty values.
pub fn display_timezone(timezone: &str) -> String {
    if timezone.is_empty() {
        "UTC".to_string()
    } else {
        timezone.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        display_timezone, normalized_params, pretty_json, snapshot_params, snapshot_string,
    };

    #[test]
    fn normalized_params_replaces_null() {
        assert_eq!(
            normalized_params(&serde_json::Value::Null),
            serde_json::json!({})
        );
    }

    #[test]
    fn snapshot_helpers_extract_values() {
        let snapshot = serde_json::json!({
            "job_name": "daily-sync",
            "params_json": {"limit": 100}
        });
        assert_eq!(
            snapshot_string(&snapshot, "job_name"),
            Some("daily-sync".to_string())
        );
        assert_eq!(
            snapshot_params(&snapshot),
            Some(serde_json::json!({"limit": 100}))
        );
    }

    #[test]
    fn pretty_json_returns_object_string() {
        let rendered = pretty_json(&serde_json::json!({"a": 1}));
        assert!(rendered.contains("\"a\""));
    }

    #[test]
    fn display_timezone_defaults_to_utc() {
        assert_eq!(display_timezone(""), "UTC");
        assert_eq!(
            display_timezone("America/Los_Angeles"),
            "America/Los_Angeles"
        );
    }
}
