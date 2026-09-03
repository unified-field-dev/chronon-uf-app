/// Format duration in milliseconds to a human-readable string.
#[allow(clippy::cast_precision_loss)]
pub fn format_duration(ms: Option<u64>) -> String {
    // Realistic run durations are well within f64's exact-integer range, so the
    // `u64 -> f64` conversion below never loses precision in practice.
    ms.map_or_else(
        || "\u{2014}".to_string(),
        |ms| {
            if ms < 1000 {
                format!("{ms}ms")
            } else if ms < 60_000 {
                format!("{:.1}s", ms as f64 / 1000.0)
            } else {
                let mins = ms / 60_000;
                let secs = (ms % 60_000) / 1000;
                format!("{mins}m {secs}s")
            }
        },
    )
}

/// Format an ISO 8601 `started_at` timestamp for display (HH:MM).
pub fn format_started_at(started_at: &str) -> String {
    if let Some(time_part) = started_at.split('T').nth(1) {
        if let Some(time) = time_part.split('.').next() {
            let parts: Vec<&str> = time.split(':').collect();
            if parts.len() >= 2 {
                return format!("{}:{}", parts[0], parts[1]);
            }
        }
    }
    started_at.to_string()
}
