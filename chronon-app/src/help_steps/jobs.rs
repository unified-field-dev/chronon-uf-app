//! Spotlight steps for the Jobs catalog (`/chronon/jobs`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Jobs page intro: named schedule bound to one Script.
#[help_spotlight_step(
    route = "/chronon/jobs",
    feature_highlight = "chronon-jobs-intro",
    title = "Jobs catalog",
    spotlight = "chronon-jobs-page",
    position = "top",
    order = 10
)]
#[component]
pub fn ChrononJobsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-jobs-intro",
        "A Job is a named schedule that points at one Script. Like an alarm clock tied to a recipe: the clock says when; the recipe says what happens.",
        Some("If the table is empty, no schedules exist yet."),
        &[],
    )
}

/// Search and filters on the jobs list.
#[help_spotlight_step(
    route = "/chronon/jobs",
    feature_highlight = "chronon-jobs-search",
    title = "Find a Job",
    spotlight = "chronon-jobs-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononJobsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-jobs-search",
        "Use search and filters when the catalog is long. Type part of a name; clear filters to see every Job.",
        None,
        &[],
    )
}

/// Jobs data table columns.
#[help_spotlight_step(
    route = "/chronon/jobs",
    feature_highlight = "chronon-jobs-table",
    title = "Reading the table",
    spotlight = "chronon-jobs-data-table",
    position = "top",
    order = 30
)]
#[component]
pub fn ChrononJobsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-jobs-table",
        "Compare these columns before you open a row.",
        None,
        &[
            "Name: label people search for",
            "Script: which recipe it runs",
            "Schedule: cron, run once, or manual",
            "Status: armed (active) or silenced (paused)",
            "Last / Next: most recent and upcoming attempt",
        ],
    )
}

/// Create Job button.
#[help_spotlight_step(
    route = "/chronon/jobs",
    feature_highlight = "chronon-jobs-create",
    title = "Create a Job",
    spotlight = "chronon-jobs-create-button",
    position = "bottom",
    order = 40
)]
#[component]
pub fn ChrononJobsCreateHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-jobs-create",
        "Opens the form to name a Job, pick a Script, choose when it runs, and save. Needs a verified email and Chronon admin.",
        None,
        &[],
    )
}

/// Open a Job via table row click.
#[help_spotlight_step(
    route = "/chronon/jobs",
    feature_highlight = "chronon-jobs-open",
    title = "Open a Job",
    spotlight = "chronon-jobs-data-table",
    position = "bottom",
    order = 50
)]
#[component]
pub fn ChrononJobsOpenHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-jobs-open",
        "Click any row to open that Job's detail page: pause it, edit the schedule, or press Run Now.",
        None,
        &[],
    )
}
