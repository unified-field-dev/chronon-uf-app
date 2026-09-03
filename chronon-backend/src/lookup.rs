//! In-memory list/detail contracts for job/run/script collections.

use crate::types::{Job, Run, Script};

/// Locates a UI job by exact id (used by detail / update contracts).
#[must_use]
pub fn find_job_by_id<'a>(jobs: &'a [Job], job_id: &str) -> Option<&'a Job> {
    jobs.iter().find(|j| j.id == job_id)
}

/// Locates a UI job by exact name (used when id lookup misses).
#[must_use]
pub fn find_job_by_name<'a>(jobs: &'a [Job], job_name: &str) -> Option<&'a Job> {
    jobs.iter().find(|j| j.name == job_name)
}

/// Resolves a job id from an id-or-name key against an in-memory job list.
///
/// Tries exact id first, then exact name — matching `get_job` / `resolve_job_id`.
#[must_use]
pub fn resolve_job_id_from_list(jobs: &[Job], job_id_or_name: &str) -> Option<String> {
    if let Some(j) = find_job_by_id(jobs, job_id_or_name) {
        return Some(j.id.clone());
    }
    find_job_by_name(jobs, job_id_or_name).map(|j| j.id.clone())
}

/// Locates a UI run by exact id (used by `get_run` detail lookups).
#[must_use]
pub fn find_run_by_id<'a>(runs: &'a [Run], run_id: &str) -> Option<&'a Run> {
    runs.iter().find(|r| r.id == run_id)
}

/// Locates a script by exact name (used by script registry detail contracts).
#[must_use]
pub fn find_script_by_name<'a>(scripts: &'a [Script], name: &str) -> Option<&'a Script> {
    scripts.iter().find(|s| s.name == name)
}

/// Sorts jobs by name (stable list contract for `get_jobs` / `get_jobs_page`).
pub fn sort_jobs_by_name(jobs: &mut [Job]) {
    jobs.sort_by(|a, b| a.name.cmp(&b.name));
}

/// Sorts scripts by name (stable list contract for `get_scripts_page`).
pub fn sort_scripts_by_name(scripts: &mut [Script]) {
    scripts.sort_by(|a, b| a.name.cmp(&b.name));
}
