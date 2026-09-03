//! Integration contracts for dashboard KPI / run-trend helpers.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use chrono::{TimeZone, Timelike, Utc};
use chronon_backend::{
    align_run_bucket, dashboard_stats_from_jobs_and_runs, recent_run_from_model,
    run_bucket_granularity, run_stats_series_from_runs, RunBucketGranularity, RunStatus,
};
use chronon_core::{Job as CoreJob, Run as CoreRun, RunStatus as CoreRunStatus};

fn sample_core_run(
    id: &str,
    status: CoreRunStatus,
    scheduled_for: chrono::DateTime<Utc>,
) -> CoreRun {
    let mut run = CoreRun::for_job("job-1", "cleanup", scheduled_for);
    run.run_id = id.into();
    run.status = status;
    run.started_at = Some(scheduled_for);
    run.duration_ms = Some(10);
    run
}

#[test]
fn dashboard_stats_aggregates_counts_happy_path() {
    let mut active = CoreJob::new("a", "s");
    active.enabled = true;
    let mut paused = CoreJob::new("b", "s");
    paused.enabled = false;
    let today = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap();
    let runs = vec![
        sample_core_run(
            "r1",
            CoreRunStatus::Success,
            Utc.with_ymd_and_hms(2026, 1, 2, 1, 0, 0).unwrap(),
        ),
        sample_core_run(
            "r2",
            CoreRunStatus::Timeout,
            Utc.with_ymd_and_hms(2026, 1, 2, 2, 0, 0).unwrap(),
        ),
        sample_core_run(
            "r3",
            CoreRunStatus::Running,
            Utc.with_ymd_and_hms(2026, 1, 2, 3, 0, 0).unwrap(),
        ),
    ];
    let stats = dashboard_stats_from_jobs_and_runs(&[active, paused], &runs, today);
    assert_eq!(stats.total_jobs, 2);
    assert_eq!(stats.active_jobs, 1);
    assert_eq!(stats.paused_jobs, 1);
    assert_eq!(stats.total_runs_today, 3);
    assert_eq!(stats.successful_runs_today, 1);
    assert_eq!(stats.failed_runs_today, 1);
    assert_eq!(stats.running_now, 1);
}

#[test]
fn run_stats_series_24h_includes_success_and_failed_happy_path() {
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
    let runs = vec![
        sample_core_run(
            "r1",
            CoreRunStatus::Success,
            now - chrono::Duration::hours(1),
        ),
        sample_core_run(
            "r2",
            CoreRunStatus::Canceled,
            now - chrono::Duration::hours(2),
        ),
    ];
    let series = run_stats_series_from_runs(&runs, now, 86_400);
    assert_eq!(series[0].id, "successful");
    assert_eq!(series[1].id, "failed");
    assert!(series[0].points.iter().any(|p| p.value > 0.0));
    assert!(series[1].points.iter().any(|p| p.value > 0.0));
}

#[test]
fn run_stats_series_all_outside_window_zero_success_sad() {
    let now = Utc.with_ymd_and_hms(2026, 1, 10, 12, 0, 0).unwrap();
    let runs = vec![sample_core_run(
        "r-old",
        CoreRunStatus::Success,
        Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
    )];
    let series = run_stats_series_from_runs(&runs, now, 86_400);
    assert!(series[0].points.iter().all(|p| p.value == 0.0));
    assert!(series[1].points.iter().all(|p| p.value == 0.0));
}

#[test]
fn run_bucket_granularity_and_align_happy_path() {
    assert_eq!(run_bucket_granularity(86_400), RunBucketGranularity::Hourly);
    assert_eq!(run_bucket_granularity(86_401), RunBucketGranularity::Daily);
    let ts = Utc.with_ymd_and_hms(2026, 1, 1, 10, 45, 0).unwrap();
    let hourly = align_run_bucket(ts, RunBucketGranularity::Hourly);
    assert_eq!(hourly.hour(), 10);
    assert_eq!(hourly.minute(), 0);
    let daily = align_run_bucket(ts, RunBucketGranularity::Daily);
    assert_eq!(daily.hour(), 0);
}

#[test]
fn recent_run_from_model_shape_happy_path() {
    let mut run = sample_core_run(
        "r1",
        CoreRunStatus::Success,
        Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap(),
    );
    run.duration_ms = Some(99);
    let recent = recent_run_from_model(&run, "nightly".into());
    assert_eq!(recent.id, "r1");
    assert_eq!(recent.job_name, "nightly");
    assert_eq!(recent.status, RunStatus::Completed);
    assert_eq!(recent.duration_ms, Some(99));
}
