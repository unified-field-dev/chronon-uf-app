//! Spotlight steps for the Runs list (`/chronon/runs`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Runs page intro: one attempt per Run.
#[help_spotlight_step(
    route = "/chronon/runs",
    feature_highlight = "chronon-runs-intro",
    title = "Run history",
    spotlight = "chronon-runs-page",
    position = "top",
    order = 10
)]
#[component]
pub fn ChrononRunsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-runs-intro",
        "A Run is one attempt to execute a Job's Script. Like one baking of the recipe: success, failure, or still in the oven.",
        None,
        &[],
    )
}

/// Search and filters on the runs list.
#[help_spotlight_step(
    route = "/chronon/runs",
    feature_highlight = "chronon-runs-search",
    title = "Find a Run",
    spotlight = "chronon-runs-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononRunsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-runs-search",
        "Search and filter across all Jobs. Narrow by status when you are hunting failures.",
        None,
        &[],
    )
}

/// Runs data table columns.
#[help_spotlight_step(
    route = "/chronon/runs",
    feature_highlight = "chronon-runs-table",
    title = "Each attempt",
    spotlight = "chronon-runs-data-table",
    position = "top",
    order = 30
)]
#[component]
pub fn ChrononRunsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-runs-table",
        "Compare attempts before you open a row.",
        None,
        &[
            "Job: which schedule produced the attempt",
            "Status: outcome badge",
            "Started: when it began",
            "Duration: how long it took",
        ],
    )
}

/// Open a Run via table row click.
#[help_spotlight_step(
    route = "/chronon/runs",
    feature_highlight = "chronon-runs-open",
    title = "Open a Run",
    spotlight = "chronon-runs-data-table",
    position = "bottom",
    order = 40
)]
#[component]
pub fn ChrononRunsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-runs-open",
        "Click a row for start/finish times and the text the Script printed.",
        None,
        &[],
    )
}
