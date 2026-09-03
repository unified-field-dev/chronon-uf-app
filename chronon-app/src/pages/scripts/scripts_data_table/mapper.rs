use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::{Script, ScriptParam};

fn format_params_summary(params: &[ScriptParam]) -> String {
    if params.is_empty() {
        return "—".to_string();
    }
    params
        .iter()
        .map(|param| format!("{}: {}", param.name, param.param_type))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn script_to_record(script: Script) -> DataRecord {
    let name = script.name.clone();
    let params_summary = format_params_summary(&script.params);
    let description = script
        .description
        .clone()
        .filter(|d| !d.trim().is_empty())
        .unwrap_or_else(|| "—".to_string());
    DataRecord::new(
        name.clone(),
        HashMap::from([
            ("name".into(), DataValue::Text(name)),
            ("signature".into(), DataValue::Text(script.signature)),
            ("description".into(), DataValue::Text(description)),
            ("params_summary".into(), DataValue::Text(params_summary)),
        ]),
    )
}
