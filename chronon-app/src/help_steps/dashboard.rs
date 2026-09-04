//! Spotlight steps for the Chronon dashboard (`/chronon`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Centered intro: control room metaphor and Job / Script / Run vocabulary.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-intro",
    title = "Welcome to Chronon",
    order = 10
)]
#[component]
pub fn ChrononIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-intro",
        "Chronon is the control room for work that should run on a schedule. Think of it like a shared calendar for automated tasks: pick what to run, choose when, and keep a history of every attempt.",
        Some("Anyone signed in can browse these pages. Changing a schedule or pressing Run Now needs Chronon admin. We will walk the screens one piece at a time."),
        &[
            "Job: the calendar entry (what + when)",
            "Script: the recipe the job runs",
            "Run: one time that recipe actually ran",
        ],
    )
}

/// KPI cards: Total, Active, Paused, Today, Successful, Failed, Running Now.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-dashboard-stats",
    title = "At a glance",
    spotlight = "chronon-dashboard-stats",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononDashboardStatsHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-dashboard-stats",
        "These numbers are today's pulse for scheduled work.",
        Some("Come back here for a quick health check."),
        &[
            "Total Jobs: how many schedules exist",
            "Active: schedules that are armed",
            "Paused: schedules that will not fire",
            "Runs Today: attempts started today",
            "Successful: attempts that finished cleanly",
            "Failed: attempts that need a look",
            "Running Now: attempts still in progress",
        ],
    )
}

/// Run outcomes trend chart.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-run-trend",
    title = "How runs are finishing",
    spotlight = "chronon-run-trend-card",
    position = "top",
    order = 30
)]
#[component]
pub fn ChrononRunTrendHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-run-trend",
        "The chart compares successful vs failed finishes. Use it when you want the reliability story, not just a single error.",
        Some("Tip: switch 24h / 7d to change the window."),
        &[],
    )
}

/// Shortcut to full Runs history.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-view-all-runs",
    title = "View all runs",
    spotlight = "chronon-run-trend-view-all",
    position = "top",
    order = 40
)]
#[component]
pub fn ChrononViewAllRunsHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-view-all-runs",
        "Opens the full Runs history for every Job. Use this when Recent Runs is not enough.",
        None,
        &[],
    )
}

/// Recent Runs table on the dashboard.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-dashboard-recent",
    title = "Latest attempts",
    spotlight = "chronon-dashboard-recent-runs",
    position = "top",
    order = 50
)]
#[component]
pub fn ChrononDashboardRecentHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-dashboard-recent",
        "Each row is one Run. Click a row to read that attempt's output.",
        None,
        &[
            "Job: which schedule produced it",
            "Status: success, failure, or still running",
            "Started: when the attempt began",
            "Duration: how long it took",
        ],
    )
}

/// Left navigation destinations.
#[help_spotlight_step(
    route = "/chronon",
    feature_highlight = "chronon-nav",
    title = "Finding your way",
    spotlight = "chronon-nav",
    position = "right",
    order = 60
)]
#[component]
pub fn ChrononNavHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-nav",
        "Use the left menu to open Dashboard for a health overview, Jobs for schedules and settings, Runs for history of attempts, and Scripts for recipes jobs can call.",
        Some("Help → Replay restarts this page's tour."),
        &[],
    )
}
