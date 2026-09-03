use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::Run;
use crate::utils::format_duration;

const fn run_status_key(status: crate::RunStatus) -> &'static str {
    use crate::RunStatus::{Cancelled, Completed, Failed, Pending, Running};
    match status {
        Pending => "pending",
        Running => "running",
        Completed => "completed",
        Failed => "failed",
        Cancelled => "cancelled",
    }
}

pub fn run_to_record(run: Run) -> DataRecord {
    let id = run.id.clone();
    let mut fields: HashMap<String, DataValue> = HashMap::from([
        (
            "status".into(),
            DataValue::Text(run_status_key(run.status).into()),
        ),
        (
            "started_at".into(),
            DataValue::Text(crate::utils::format_started_at(&run.started_at)),
        ),
        (
            "duration_ms".into(),
            DataValue::Text(format_duration(run.duration_ms)),
        ),
    ]);
    if !run.job_name.is_empty() {
        fields.insert("job_name".into(), DataValue::Text(run.job_name));
    }
    DataRecord::new(id, fields)
}
