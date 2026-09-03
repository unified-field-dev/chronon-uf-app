# Chronon UF App

[![CI](https://github.com/unified-field-dev/chronon-uf-app/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/chronon-uf-app/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/chronon-uf-app) · `cargo doc -p chronon-backend --open`

## About

Chronon UF App is the Unified Field **operations UI** for Chronon jobs, runs,
and scripts under `/chronon`. Chronon itself has no built-in UI; hosts mount
this crate so operators can inspect and operate scheduled work.

- **UI (`chronon-app`)** — pages, Higgs `#[server]` wrappers, `ChrononRoutes`,
  `uf_app!` registration
- **Backend (`chronon-backend`)** — pure job/run/script/schedule/dashboard
  helpers (no Leptos); primary Layer 1 CI surface
- **E2E (`chronon-uf-app-e2e`)** — lab Leptos host + Playwright for `/chronon`
  ops workflows (`cargo leptos end-to-end --project chronon-uf-app-e2e`)

Hosts supply a Chronon coordinator and auth guard context. Enable `ssr` /
hydrate to match your host. Crate-root rustdoc owns Concern → route → server fn
tables; prefer `cargo doc -p chronon-backend --open` for the mapping contract.
UI rustdoc is pin-dependent on Orbital / host graphs. Poll-based refresh is live
today; `photon_ws` / `live` are the stubbed integration points for Photon push
once a host wires them.

## Getting started

```toml
[dependencies]
# Pin tag or rev — do not use branch = "main".
chronon-app = { git = "https://github.com/unified-field-dev/chronon-uf-app", package = "chronon-app", rev = "REPLACE_WITH_PIN", default-features = false }
chronon-backend = { git = "https://github.com/unified-field-dev/chronon-uf-app", package = "chronon-backend", rev = "REPLACE_WITH_PIN" }
```

```rust,ignore
use chronon_app::ChrononRoutes;
use leptos_router::components::Routes;

view! {
    <Routes fallback=|| "not found">
        <ChrononRoutes />
    </Routes>
}
```

Wire Chronon coordinator + session extractors in host bootstrap, then mount the
routes above. Full Leptos SSR hosts live outside this repository; use the local
teaching host for the auth + dashboard contract.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-chronon-uf-app
cargo test -p chronon-backend
```

## Workspace

| Crate | Role |
|-------|------|
| [`chronon-app`](chronon-app/) | Leptos ops UI + `ChrononRoutes` + app registration |
| [`chronon-backend`](chronon-backend/) | Pure DTO/mapping helpers for job/run/script/schedule/dashboard |
| [`protected-chronon-host`](examples/protected-chronon-host/) | Teaching host: deny/allow + dashboard KPIs |

## Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-chronon-host`](examples/protected-chronon-host/) | Auth + `/chronon` dashboard API | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-chronon-uf-app cargo run -p protected-chronon-host` | Deny/allow + KPI JSON | Mount `ChrononRoutes` |

Copy table + product mount `Cargo.toml`:
[`examples/protected-chronon-host/README.md`](examples/protected-chronon-host/README.md).
More examples: [`examples/README.md`](examples/README.md).

## Security

Auth-gated `/chronon` routes (job create/edit also require a verified email) and
private vulnerability reporting: [`SECURITY.md`](SECURITY.md). Report
vulnerabilities privately — do not open a public issue for security-sensitive
reports.

## Verify

GitHub Actions (`.github/workflows/ci.yml`) runs the CI subset from
[`docs/VERIFICATION.md`](docs/VERIFICATION.md): fmt, clippy `-D warnings` on
`chronon-backend` (+ teaching host), contract tests, `protected-chronon-host`
check/run, and chronon-backend rustdoc with broken-intra-doc-link deny.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-chronon-uf-app
cargo fmt -p chronon-backend -p chronon-app -p protected-chronon-host -- --check
cargo clippy -p chronon-backend --all-targets -- -D warnings
cargo clippy -p protected-chronon-host --all-targets -- -D warnings
cargo test -p chronon-backend --test workspace_members --test product_surface
cargo test -p chronon-backend
cargo check -p protected-chronon-host
cargo run -p protected-chronon-host
RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc -p chronon-backend --no-deps
```

Teaching host success line:
`protected_chronon_host: OK — /chronon deny/allow + dashboard KPIs`.
Full command block: [`docs/VERIFICATION.md`](docs/VERIFICATION.md). Contribute:
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## FAQ

**Is this a standalone Chronon server?** No. `chronon-app` mounts under a host
`<Routes>` tree. Job scheduling and persistence live in the Chronon coordinator /
core crates.

**Why is there a separate `chronon-backend` crate?** So job/run/script/schedule
and dashboard helpers stay unit-testable without the Leptos/UI dependency graph.
`chronon-app` `#[server]` fns are thin wrappers over those helpers.

**What can operators change from the UI?** Create and edit scheduled jobs (email
verified), and trigger "run now" from a run detail view. List/detail, scripts
catalog, and dashboard views are read paths.

**Where does Chronon core fit?** Scheduling, execution, and IsolatedLab contracts
live in the Chronon coordinator / core repos. This repo maps admin/list/get/update
APIs into UF ops pages.

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md),
[SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
