use std::collections::HashMap;

use leptos::prelude::*;

use crate::server::ScriptParam;

/// Builds the JSON params object for job creation from form string values.
pub fn build_create_params(
    params: &[ScriptParam],
    values: &HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    let mut object = serde_json::Map::new();

    for param in params {
        let raw = values.get(&param.name).map_or("", String::as_str);
        let trimmed = raw.trim();

        if param.required && trimmed.is_empty() {
            return Err(format!("Parameter '{}' is required.", param.name));
        }

        if trimmed.is_empty() {
            continue;
        }

        object.insert(
            param.name.clone(),
            coerce_param_value(&param.param_type, trimmed)?,
        );
    }

    Ok(serde_json::Value::Object(object))
}

fn coerce_param_value(param_type: &str, raw: &str) -> Result<serde_json::Value, String> {
    match param_type {
        "bool" => raw
            .parse::<bool>()
            .map(serde_json::Value::Bool)
            .map_err(|_| format!("Expected true or false, got '{raw}'.")),
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize" => raw
            .parse::<i64>()
            .map(serde_json::Value::from)
            .map_err(|_| format!("Expected an integer, got '{raw}'.")),
        "f32" | "f64" => raw
            .parse::<f64>()
            .map(serde_json::Value::from)
            .map_err(|_| format!("Expected a number, got '{raw}'.")),
        _ => Ok(serde_json::Value::String(raw.to_string())),
    }
}

/// Builds per-param signals seeded from script defaults when the selected script changes.
pub fn seed_param_signals(params: &[ScriptParam]) -> HashMap<String, RwSignal<String>> {
    params
        .iter()
        .map(|p| {
            (
                p.name.clone(),
                RwSignal::new(p.default.clone().unwrap_or_default()),
            )
        })
        .collect()
}

/// Collects current string values from param signals for submission.
pub fn collect_param_values(
    signals: &HashMap<String, RwSignal<String>>,
) -> HashMap<String, String> {
    signals
        .iter()
        .map(|(name, signal)| (name.clone(), signal.get()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{build_create_params, coerce_param_value};
    use crate::server::ScriptParam;

    fn param(name: &str, param_type: &str, required: bool, default: Option<&str>) -> ScriptParam {
        ScriptParam {
            name: name.to_string(),
            param_type: param_type.to_string(),
            default: default.map(str::to_string),
            required,
        }
    }

    #[test]
    fn build_create_params_rejects_missing_required() {
        let defs = vec![param("limit", "i32", true, None)];
        let values = std::collections::HashMap::new();
        let err = build_create_params(&defs, &values).expect_err("required param missing");
        assert!(err.contains("limit"));
    }

    #[test]
    fn build_create_params_coerces_bool_and_number() {
        let defs = vec![
            param("force", "bool", false, None),
            param("retain_days", "i32", false, None),
        ];
        let mut values = std::collections::HashMap::new();
        values.insert("force".to_string(), "true".to_string());
        values.insert("retain_days".to_string(), "7".to_string());

        let json = build_create_params(&defs, &values).expect("valid params");
        assert_eq!(json, serde_json::json!({"force": true, "retain_days": 7}));
    }

    #[test]
    fn build_create_params_empty_script_returns_empty_object() {
        let json = build_create_params(&[], &std::collections::HashMap::new()).expect("empty");
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn coerce_param_value_parses_bool() {
        assert_eq!(
            coerce_param_value("bool", "false").unwrap(),
            serde_json::json!(false)
        );
    }

    #[test]
    fn coerce_param_value_rejects_invalid_bool_and_int() {
        assert!(coerce_param_value("bool", "yes").is_err());
        assert!(coerce_param_value("i32", "1.5").is_err());
        assert!(coerce_param_value("f64", "nope").is_err());
    }

    #[test]
    fn build_create_params_skips_empty_optional() {
        let defs = vec![
            param("limit", "i32", false, None),
            param("label", "String", false, None),
        ];
        let mut values = std::collections::HashMap::new();
        values.insert("limit".to_string(), "  ".to_string());
        values.insert("label".to_string(), "ok".to_string());
        let json = build_create_params(&defs, &values).expect("optional empty skipped");
        assert_eq!(json, serde_json::json!({"label": "ok"}));
    }
}
