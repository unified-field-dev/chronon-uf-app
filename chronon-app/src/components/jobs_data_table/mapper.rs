use std::collections::HashMap;

use orbital_data::{DataRecord, DataValue};

use crate::server::Job;

const fn job_status_key(status: crate::JobStatus) -> &'static str {
    use crate::JobStatus::{Active, Disabled, Paused};
    match status {
        Active => "active",
        Paused => "paused",
        Disabled => "disabled",
    }
}

fn format_time_short(time: Option<&str>) -> String {
    match time {
        Some(t) => {
            if let Some(dt_part) = t.split('T').nth(1) {
                if let Some(time_str) = dt_part.split('.').next() {
                    let parts: Vec<&str> = time_str.trim_end_matches('Z').split(':').collect();
                    if parts.len() >= 2 {
                        return format!("{}:{}", parts[0], parts[1]);
                    }
                }
            }
            t.to_string()
        }
        None => "—".to_string(),
    }
}

pub fn job_to_record(job: Job) -> DataRecord {
    let id = job.id.clone();
    DataRecord::new(
        id,
        HashMap::from([
            ("name".into(), DataValue::Text(job.name)),
            ("script_name".into(), DataValue::Text(job.script_name)),
            ("cron".into(), DataValue::Text(job.cron)),
            (
                "status".into(),
                DataValue::Text(job_status_key(job.status).into()),
            ),
            (
                "last_run".into(),
                DataValue::Text(format_time_short(job.last_run_at.as_deref())),
            ),
            (
                "next_run".into(),
                DataValue::Text(format_time_short(job.next_run_at.as_deref())),
            ),
        ]),
    )
}
