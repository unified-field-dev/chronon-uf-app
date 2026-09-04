//! Lazy-loaded route views for WASM code-splitting (`cargo leptos --split`).

#![allow(clippy::used_underscore_binding)]

use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::{
    ChrononDashboardPage, ChrononJobCreatePage, ChrononJobDetailPage, ChrononJobsPage,
    ChrononLayout, ChrononRunDetailPage, ChrononRunsPage, ChrononScriptsPage,
};

/// Prefetch the chronon family WASM chunk (leaf pages share split modules).
pub async fn prefetch_family() {
    ChrononDashboardRoute::preload().await;
}

/// Eager layout shell for `/chronon/*` ParentRoute (auth gate lives inside [`ChrononLayout`]).
#[component]
pub fn ChrononLayoutRouteView() -> impl IntoView {
    view! { <ChrononLayout /> }
}

/// Lazy `/chronon` dashboard.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononDashboardRoute;

#[lazy_route]
impl LazyRoute for ChrononDashboardRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ChrononDashboardPage /> }.into_any()
    }
}

/// Lazy `/chronon/jobs`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononJobsRoute;

#[lazy_route]
impl LazyRoute for ChrononJobsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ChrononJobsPage /> }.into_any()
    }
}

/// Lazy `/chronon/jobs/new` (email-verified).
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononVerifiedJobCreateRoute;

#[lazy_route]
impl LazyRoute for ChrononVerifiedJobCreateRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <uf_product::routes::RequireAuthenticated requires_email_verification=true>
                <ChrononJobCreatePage />
            </uf_product::routes::RequireAuthenticated>
        }
        .into_any()
    }
}

/// Lazy `/chronon/jobs/:job_id` (email-verified).
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononVerifiedJobDetailRoute;

#[lazy_route]
impl LazyRoute for ChrononVerifiedJobDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! {
            <uf_product::routes::RequireAuthenticated requires_email_verification=true>
                <ChrononJobDetailPage />
            </uf_product::routes::RequireAuthenticated>
        }
        .into_any()
    }
}

/// Lazy `/chronon/runs`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononRunsRoute;

#[lazy_route]
impl LazyRoute for ChrononRunsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ChrononRunsPage /> }.into_any()
    }
}

/// Lazy `/chronon/runs/:run_id`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononRunDetailRoute;

#[lazy_route]
impl LazyRoute for ChrononRunDetailRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ChrononRunDetailPage /> }.into_any()
    }
}

/// Lazy `/chronon/scripts`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChrononScriptsRoute;

#[lazy_route]
impl LazyRoute for ChrononScriptsRoute {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <ChrononScriptsPage /> }.into_any()
    }
}
