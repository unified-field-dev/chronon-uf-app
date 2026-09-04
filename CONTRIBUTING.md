# Contributing to Chronon UF App

Thank you for improving this project.

## Where to look

| Goal | Start here |
|------|------------|
| Pure job/run/script/schedule contracts | `chronon-backend` (`cargo doc -p chronon-backend --open`) |
| Mount `/chronon` in a host | `chronon-app::ChrononRoutes` crate-root rustdoc |
| Auth + dashboard KPI smoke | `examples/protected-chronon-host` |
| Operator UI happy/sad paths | [`docs/VERIFICATION.md`](docs/VERIFICATION.md) Layer 2 |
| CI commands | [`.github/workflows/ci.yml`](.github/workflows/ci.yml) |

Backend contract tests first, then the teaching host, then Playwright:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-chronon-uf-app
cargo test -p chronon-backend
cargo run -p protected-chronon-host
# Optional when the UI graph compiles:
cargo leptos end-to-end --project chronon-uf-app-e2e
```

Full Layer 1 / Layer 2 command lists: [`docs/VERIFICATION.md`](docs/VERIFICATION.md).

## Development setup

1. Clone [unified-field-dev/chronon-uf-app](https://github.com/unified-field-dev/chronon-uf-app) into a Unified Field sibling checkout (path deps expect `L0`/`L1`/`L2` neighbors).
2. Install a nightly toolchain matching CI (Leptos `nightly` feature).
3. From the repository root:

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-chronon-uf-app
cargo check --workspace
cargo check -p chronon-app --features ssr
```

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Security reports: [`SECURITY.md`](SECURITY.md).

## Pull requests

- Prefer small, focused PRs.
- Update [`README.md`](README.md) when user-facing flows or host mounting steps change.
- Keep [`docs/VERIFICATION.md`](docs/VERIFICATION.md) aligned when you change CI gates or test layers.
