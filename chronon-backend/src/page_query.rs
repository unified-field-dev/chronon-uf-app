//! In-memory paging, quick-search, and filter helpers for Chronon list endpoints.
//!
//! 0.1.n: filters run over bounded backend fetches (same pattern as valence-app schema index).

use orbital_data::DataValue;
use orbital_paging::{FilterLogicWire, FilterQuery, FilterRuleParam, PageRequest};

use crate::map::{job_status_label, run_status_label};
use crate::types::{Job, Run, Script};

fn filter_rule_text(value: &DataValue) -> String {
    value.display_string()
}

fn text_contains(haystack: &str, needle: &str) -> bool {
    let needle = needle.trim();
    if needle.is_empty() {
        return true;
    }
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn text_equals(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

fn job_matches_filter_rule(job: &Job, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.name, &value),
            "equals" | "is" => text_equals(&job.name, &value),
            "not_equals" | "is_not" => !text_equals(&job.name, &value),
            _ => true,
        },
        "script_name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.script_name, &value),
            "equals" | "is" => text_equals(&job.script_name, &value),
            "not_equals" | "is_not" => !text_equals(&job.script_name, &value),
            _ => true,
        },
        "status" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(job_status_label(job.status), &value),
            "not_equals" | "is_not" => !text_equals(job_status_label(job.status), &value),
            _ => true,
        },
        "cron" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&job.cron, &value),
            "equals" | "is" => text_equals(&job.cron, &value),
            _ => true,
        },
        _ => true,
    }
}

fn apply_job_filter_query(jobs: &mut Vec<Job>, filter: &FilterQuery) {
    jobs.retain(|job| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| job_matches_filter_rule(job, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Applies quick-search and structured filters for the jobs `DataTable`.
pub fn apply_jobs_page_query(jobs: &mut Vec<Job>, request: &PageRequest) {
    if let Some(ref quick) = request.quick_search {
        let q_lower = quick.trim().to_lowercase();
        if !q_lower.is_empty() {
            jobs.retain(|j| {
                j.name.to_lowercase().contains(&q_lower)
                    || j.script_name.to_lowercase().contains(&q_lower)
                    || j.cron.to_lowercase().contains(&q_lower)
            });
        }
    }
    if let Some(ref filter) = request.filter {
        apply_job_filter_query(jobs, filter);
    }
}

fn run_matches_filter_rule(run: &Run, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "job_name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&run.job_name, &value),
            "equals" | "is" => text_equals(&run.job_name, &value),
            "not_equals" | "is_not" => !text_equals(&run.job_name, &value),
            _ => true,
        },
        "status" => match rule.operator.as_str() {
            "equals" | "is" => text_equals(run_status_label(run.status), &value),
            "not_equals" | "is_not" => !text_equals(run_status_label(run.status), &value),
            _ => true,
        },
        "id" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&run.id, &value),
            "equals" | "is" => text_equals(&run.id, &value),
            _ => true,
        },
        _ => true,
    }
}

fn apply_run_filter_query(runs: &mut Vec<Run>, filter: &FilterQuery) {
    runs.retain(|run| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| run_matches_filter_rule(run, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Applies quick-search and structured filters for the runs `DataTable`.
pub fn apply_runs_page_query(runs: &mut Vec<Run>, request: &PageRequest) {
    if let Some(ref quick) = request.quick_search {
        let q_lower = quick.trim().to_lowercase();
        if !q_lower.is_empty() {
            runs.retain(|r| {
                r.job_name.to_lowercase().contains(&q_lower)
                    || r.id.to_lowercase().contains(&q_lower)
            });
        }
    }
    if let Some(ref filter) = request.filter {
        apply_run_filter_query(runs, filter);
    }
}

fn script_matches_filter_rule(script: &Script, rule: &FilterRuleParam) -> bool {
    let value = filter_rule_text(&rule.value);
    match rule.field.as_str() {
        "name" => match rule.operator.as_str() {
            "contains" | "not_contains" => text_contains(&script.name, &value),
            "equals" | "is" => text_equals(&script.name, &value),
            "not_equals" | "is_not" => !text_equals(&script.name, &value),
            _ => true,
        },
        _ => true,
    }
}

fn apply_script_filter_query(scripts: &mut Vec<Script>, filter: &FilterQuery) {
    scripts.retain(|script| {
        let matches: Vec<bool> = filter
            .items
            .iter()
            .map(|rule| script_matches_filter_rule(script, rule))
            .collect();
        match filter.logic {
            FilterLogicWire::And => matches.iter().all(|m| *m),
            FilterLogicWire::Or => matches.iter().any(|m| *m),
        }
    });
}

/// Applies quick-search and structured filters for the scripts `DataTable`.
pub fn apply_scripts_page_query(scripts: &mut Vec<Script>, request: &PageRequest) {
    if let Some(ref quick) = request.quick_search {
        let q_lower = quick.trim().to_lowercase();
        if !q_lower.is_empty() {
            scripts.retain(|s| {
                s.name.to_lowercase().contains(&q_lower)
                    || s.signature.to_lowercase().contains(&q_lower)
                    || s.description
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&q_lower))
                    || s.params
                        .iter()
                        .any(|p| p.name.to_lowercase().contains(&q_lower))
            });
        }
    }
    if let Some(ref filter) = request.filter {
        apply_script_filter_query(scripts, filter);
    }
}

/// True when quick-search or structured filters require an in-memory scan.
#[must_use]
pub fn runs_page_needs_memory_scan(request: &PageRequest) -> bool {
    request
        .quick_search
        .as_ref()
        .is_some_and(|quick| !quick.trim().is_empty())
        || request.filter.is_some()
}
