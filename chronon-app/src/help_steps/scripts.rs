//! Spotlight steps for the Scripts catalog (`/chronon/scripts`).

use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

use super::help_stack;

/// Scripts page intro: registered recipes.
#[help_spotlight_step(
    route = "/chronon/scripts",
    feature_highlight = "chronon-scripts-intro",
    title = "Scripts catalog",
    spotlight = "chronon-scripts-page",
    position = "top",
    order = 10
)]
#[component]
pub fn ChrononScriptsIntroHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-scripts-intro",
        "A Script is a registered recipe your host provides. Jobs point at Scripts; you do not upload code here. This page is the menu of what can be scheduled.",
        None,
        &[],
    )
}

/// Search and filters on the scripts list.
#[help_spotlight_step(
    route = "/chronon/scripts",
    feature_highlight = "chronon-scripts-search",
    title = "Find a Script",
    spotlight = "chronon-scripts-search",
    position = "bottom",
    order = 20
)]
#[component]
pub fn ChrononScriptsSearchHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-scripts-search",
        "Search when many recipes are registered. Clear the filter to see the full menu again.",
        None,
        &[],
    )
}

/// Scripts data table columns.
#[help_spotlight_step(
    route = "/chronon/scripts",
    feature_highlight = "chronon-scripts-table",
    title = "Registered recipes",
    spotlight = "chronon-scripts-data-table",
    position = "top",
    order = 30
)]
#[component]
pub fn ChrononScriptsTableHelp() -> impl IntoView {
    help_stack(
        "help-step-chronon-scripts-table",
        "Use this list before Create Job so you pick a match.",
        None,
        &[
            "Script: recipe name",
            "Signature: shape of inputs it expects",
            "Description: what the recipe is for",
            "Parameters: knobs that become Job form fields",
        ],
    )
}
