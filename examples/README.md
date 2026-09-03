# Examples

Runnable teaching hosts for this UF app. Each card: when to use · command ·
success · look next.

## Canonical path

### `protected-chronon-host` — auth + `/chronon` dashboard

**Teaches:** session auth gate on `/chronon` and the in-memory dashboard KPI shape
`chronon-backend` builds for the UI. Inventory names: `chronon` / `/chronon` /
`RequireAuthenticated` / `ChrononAdmin`.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-chronon-uf-app
cargo run -p protected-chronon-host
```

**Success:** stdout prints `protected_chronon_host: OK — /chronon deny/allow + dashboard KPIs`.

**Next step:** Mount `<ChrononRoutes />` in a product host with Chronon
coordinator.

Copy table + product mount `Cargo.toml`:
[`protected-chronon-host/README.md`](protected-chronon-host/README.md).

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`protected-chronon-host`](protected-chronon-host/) | Auth + `/chronon` dashboard API | `cargo run -p protected-chronon-host` | Deny/allow + KPI JSON | Product host with `ChrononRoutes` |
