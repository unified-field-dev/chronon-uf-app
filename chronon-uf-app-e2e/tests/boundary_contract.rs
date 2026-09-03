//! Valence + Chronon coordinator boundary contracts for the lab host.
//!
//! These are not Playwright; they assert durable job/run postconditions on the
//! in-process LocalBackend after `init_e2e_valence`.

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use chronon_coordinator::{Job, ScheduleKind};
use chronon_uf_app_e2e::{
    e2e_admin_valence, e2e_chronon_backend, e2e_fixtures, e2e_registry, init_e2e_valence,
};

#[tokio::test]
async fn coordinator_list_seeded_job_happy_path() {
    init_e2e_valence().await;
    let backend = e2e_chronon_backend();
    let fixtures = e2e_fixtures();
    let jobs = backend.list_jobs().await;
    assert!(
        jobs.iter().any(|j| j.job_id == fixtures.job_id),
        "seeded job must appear in list_jobs"
    );
    let detail = backend
        .get_job(&fixtures.job_id)
        .await
        .expect("seeded job detail");
    assert_eq!(detail.job_name, fixtures.job_name);
    assert_eq!(detail.script_name, fixtures.script_name);
}

#[tokio::test]
async fn coordinator_unknown_job_is_none_sad() {
    init_e2e_valence().await;
    let backend = e2e_chronon_backend();
    assert!(backend
        .get_job("__chronon_e2e_missing_job__")
        .await
        .is_none());
    assert!(backend
        .get_job_by_name("__chronon_e2e_missing_name__")
        .await
        .is_none());
}

#[tokio::test]
async fn coordinator_upsert_with_valence_persists_happy_path() {
    init_e2e_valence().await;
    let backend = e2e_chronon_backend();
    let valence = e2e_admin_valence();
    let mut job = Job::new("e2e-boundary-create", "e2e_echo");
    job.schedule_kind = ScheduleKind::Manual;
    job.script_sig_hash = "e2e-sig".into();
    let job_id = job.job_id.clone();
    backend
        .upsert_job_with_valence(&valence, job)
        .await
        .expect("upsert with valence");
    let stored = backend.get_job(&job_id).await.expect("persisted job");
    assert!(
        !stored.actor_json.is_null(),
        "actor_json must be snapshotted"
    );
    assert_eq!(stored.job_name, "e2e-boundary-create");
}

#[tokio::test]
async fn coordinator_run_now_creates_run_happy_path() {
    init_e2e_valence().await;
    let backend = e2e_chronon_backend();
    let fixtures = e2e_fixtures();
    let run_id = backend.run_now(&fixtures.job_id).await.expect("run_now");
    let run = backend
        .get_run(&run_id)
        .await
        .expect("get_run")
        .expect("run row");
    assert_eq!(run.job_id.as_deref(), Some(fixtures.job_id.as_str()));
    assert_eq!(run.script_name, fixtures.script_name);
}

#[tokio::test]
async fn coordinator_run_now_unknown_job_sad() {
    init_e2e_valence().await;
    let backend = e2e_chronon_backend();
    let err = backend
        .run_now("__chronon_e2e_missing_job__")
        .await
        .expect_err("missing job must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("not found") || msg.contains("NotFound") || msg.contains("JobNotFound"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn script_registry_contains_e2e_echo_happy_path() {
    init_e2e_valence().await;
    let registry = e2e_registry();
    assert!(
        registry.get("e2e_echo").is_some(),
        "lab script must be registered"
    );
    assert!(
        registry.get("__no_such_script__").is_none(),
        "unknown script must be absent"
    );
}
