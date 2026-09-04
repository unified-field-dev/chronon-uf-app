//! Eager `/chronon` routes for the Playwright host.
//!
//! Production [`chronon_app::ChrononRoutes`] wraps leaf pages in `Lazy` for
//! wasm-split. Nested `Lazy` under `ParentRoute` still panics on
//! `hydrate_body` in this Leptos pin, so the lab host mounts the same page
//! components without `Lazy`.

use chronon_app::{
    ChrononDashboardPage, ChrononJobCreatePage, ChrononJobDetailPage, ChrononJobsPage,
    ChrononLayout, ChrononRunDetailPage, ChrononRunsPage, ChrononScriptsPage,
};
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route},
    path,
};
use uf_product::routes::RequireAuthenticated;

/// Same paths as [`chronon_app::ChrononRoutes`], without Lazy route views.
#[component(transparent)]
pub fn ChrononRoutesEager() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("chronon") view=ChrononLayout>
            <Route path=path!("") view=ChrononDashboardPage />
            <Route path=path!("jobs") view=ChrononJobsPage />
            <Route path=path!("jobs/new") view=ChrononVerifiedJobCreateEager />
            <Route path=path!("jobs/:job_id") view=ChrononVerifiedJobDetailEager />
            <Route path=path!("runs") view=ChrononRunsPage />
            <Route path=path!("runs/:run_id") view=ChrononRunDetailPage />
            <Route path=path!("scripts") view=ChrononScriptsPage />
        </ParentRoute>
    }
    .into_inner()
}

#[component]
fn ChrononVerifiedJobCreateEager() -> impl IntoView {
    view! {
        <RequireAuthenticated requires_email_verification=true>
            <ChrononJobCreatePage />
        </RequireAuthenticated>
    }
}

#[component]
fn ChrononVerifiedJobDetailEager() -> impl IntoView {
    view! {
        <RequireAuthenticated requires_email_verification=true>
            <ChrononJobDetailPage />
        </RequireAuthenticated>
    }
}
