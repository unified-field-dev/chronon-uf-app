//! Spotlight steps for Job detail (`/chronon/jobs/:job_id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Job title and status badge.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-header",
    title = "This Job",
    spotlight = "chronon-job-detail-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ChrononJobHeaderHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-header",
        "The title is the Job name. The status badge shows whether the schedule is active or paused.",
        None,
        &[],
    )
}

/// Revision selector.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-revision",
    title = "Revisions",
    spotlight = "chronon-job-detail-revision",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononJobRevisionHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-revision",
        "Each successful Save keeps a snapshot. Pick an older revision to read what the Job looked like then. Older revisions are read-only.",
        None,
        &[],
    )
}

/// Enabled toggle.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-enabled",
    title = "Armed or paused",
    spotlight = "chronon-job-detail-enabled",
    position = "bottom",
    order = 30
)]
#[component]
pub fn ChrononJobEnabledHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-enabled",
        "When enabled, Chronon follows the schedule. When paused, automatic runs stop until you turn it back on. Persist changes with Save when you are editing.",
        None,
        &[],
    )
}

/// Run Now button.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-run-now",
    title = "Run Now",
    spotlight = "chronon-job-detail-run-now",
    position = "bottom",
    order = 40
)]
#[component]
pub fn ChrononJobRunNowHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-run-now",
        "Starts one attempt immediately without waiting for the clock. A dialog may let you override parameters for this attempt only. Needs Chronon admin.",
        None,
        &[],
    )
}

/// Edit button.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-edit",
    title = "Edit the Job",
    spotlight = "chronon-job-detail-edit",
    position = "bottom",
    order = 50
)]
#[component]
pub fn ChrononJobEditHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-edit",
        "Edit switches this page into a form so you can change schedule, parameters, or enabled state. Save and Cancel appear after Edit.",
        None,
        &[],
    )
}

/// Save button (edit mode).
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-save",
    title = "Save changes",
    spotlight = "chronon-job-detail-save",
    position = "top",
    order = 60
)]
#[component]
pub fn ChrononJobSaveHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-save",
        "Save writes a new revision of this Job. Visible while editing. Needs Chronon admin.",
        None,
        &[],
    )
}

/// Cancel edit button.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-cancel-edit",
    title = "Cancel edit",
    spotlight = "chronon-job-detail-cancel",
    position = "top",
    order = 70
)]
#[component]
pub fn ChrononJobCancelEditHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-cancel-edit",
        "Cancel leaves edit mode and discards unsaved form changes. The last saved revision stays as it was.",
        None,
        &[],
    )
}

/// Configuration snapshot card.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-config",
    title = "Configuration",
    spotlight = "chronon-job-detail-config",
    position = "top",
    order = 80
)]
#[component]
pub fn ChrononJobConfigHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-config",
        "Read-only snapshot of the selected revision.",
        None,
        &[
            "Script: recipe this Job runs",
            "Schedule: cron, run once, or manual",
            "Timezone: clock used for cron / run once",
            "Parameters: values passed into the Script",
            "Last / Next: most recent and upcoming attempt",
        ],
    )
}

/// Recent runs for this Job.
#[help_spotlight_step(
    route = "/chronon/jobs/:job_id",
    feature_highlight = "chronon-job-recent",
    title = "Runs for this Job",
    spotlight = "chronon-job-detail-recent-runs",
    position = "top",
    order = 90
)]
#[component]
pub fn ChrononJobRecentHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-job-recent",
        "History for this Job only. Click a row to open that attempt's timing and output.",
        None,
        &[],
    )
}
