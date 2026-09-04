//! Spotlight steps for Run detail (`/chronon/runs/:run_id`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Run title and status badge.
#[help_spotlight_step(
    route = "/chronon/runs/:run_id",
    feature_highlight = "chronon-run-header",
    title = "This attempt",
    spotlight = "chronon-run-detail-header",
    position = "bottom",
    order = 10
)]
#[component]
pub fn ChrononRunHeaderHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-run-header",
        "The title identifies the Run. The status badge is the headline outcome for this attempt.",
        None,
        &[],
    )
}

/// Link to the parent Job.
#[help_spotlight_step(
    route = "/chronon/runs/:run_id",
    feature_highlight = "chronon-run-job-link",
    title = "Open the Job",
    spotlight = "chronon-run-detail-job-link",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononRunJobLinkHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-run-job-link",
        "Opens the schedule that produced this Run. Use it when you need to pause, edit, or trigger another attempt.",
        None,
        &[],
    )
}

/// Timing fields.
#[help_spotlight_step(
    route = "/chronon/runs/:run_id",
    feature_highlight = "chronon-run-timing",
    title = "Timing",
    spotlight = "chronon-run-detail-timing",
    position = "top",
    order = 30
)]
#[component]
pub fn ChrononRunTimingHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-run-timing",
        "Clock fields for this attempt.",
        None,
        &[
            "Started: when the attempt began",
            "Finished: when it ended (if finished)",
            "Duration: how long it took",
            "Parent Run: present when spawned from another run",
        ],
    )
}

/// Output region: error, stdout, stderr.
#[help_spotlight_step(
    route = "/chronon/runs/:run_id",
    feature_highlight = "chronon-run-output",
    title = "Output",
    spotlight = "chronon-run-detail-output",
    position = "top",
    order = 40
)]
#[component]
pub fn ChrononRunOutputHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-run-output",
        "When something fails, start here. Empty means nothing was captured for this attempt.",
        None,
        &[
            "Error banner: short failure reason when present",
            "Stdout: text the Script printed normally",
            "Stderr: text printed as errors",
        ],
    )
}
