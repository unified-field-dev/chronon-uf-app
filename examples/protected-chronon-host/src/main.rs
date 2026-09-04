//! Protected `/chronon` host: session auth gate + in-memory dashboard happy path.
//!
//! Copy surfaces for product hosts: this package's `Cargo.toml` + `main.rs`,
//! plus the product-mount dependency / Leptos sketches in the host README.
//! Oneshot path `/chronon` matches Orbital app id/path `chronon` / `/chronon`
//! (see JSON `inventory`).
//!
//! Mirrors what a real host does before mounting [`chronon_app::ChrononRoutes`]:
//! deny anonymous traffic under `/chronon`, then serve the dashboard KPI shape
//! the UI builds via `chronon-backend::dashboard_stats_from_jobs_and_runs`.
//!
//! ## When to use
//! Smoke the `/chronon` auth + dashboard contract without a full Leptos SSR graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-chronon-uf-app
//! cargo run -p protected-chronon-host
//! ```
//!
//! ## Success
//! Stdout prints `protected_chronon_host: OK — /chronon deny/allow + dashboard KPIs`.
//!
//! ## Look next
//! Mount `<ChrononRoutes />` in a product host; wire Chronon coordinator + scripts.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use chrono::{TimeZone, Utc};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn chronon_dashboard(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    // Empty in-memory job/run lists → zeroed KPIs (same helper the SSR dashboard uses).
    let today_start = Utc
        .with_ymd_and_hms(2026, 7, 27, 0, 0, 0)
        .single()
        .expect("today");
    let stats = chronon_backend::dashboard_stats_from_jobs_and_runs(&[], &[], today_start);
    Json(serde_json::json!({
        "path": "/chronon",
        "user": session.user_id,
        "stats": stats,
        "inventory": {
            "app_id": "chronon",
            "route_path": "/chronon",
            "auth_gate": "RequireAuthenticated",
            "admin_permission": "ChrononAdmin",
        },
    }))
}

fn app() -> Router {
    Router::new()
        .route("/chronon", get(chronon_dashboard))
        .route_layer(from_fn(require_session))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let denied = status_for("/chronon", None).await;
    assert_eq!(denied, StatusCode::UNAUTHORIZED);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/chronon")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/chronon");
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["stats"]["total_jobs"], 0);
    assert_eq!(body["stats"]["active_jobs"], 0);
    assert_eq!(body["inventory"]["app_id"], "chronon");
    assert_eq!(body["inventory"]["route_path"], "/chronon");
    assert_eq!(body["inventory"]["auth_gate"], "RequireAuthenticated");
    assert_eq!(body["inventory"]["admin_permission"], "ChrononAdmin");

    println!("protected_chronon_host: OK — /chronon deny/allow + dashboard KPIs");
}
