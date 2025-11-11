# CODEX_TODO — SURL

Authoritative, ordered checklist for running Codex tasks on **SURL**.
Follow **TDD**, keep commits scoped, and ensure both DB feature builds pass.

---

## Global rules

- **TDD first**: write/update failing tests before implementation.
- **Dual builds**: every code task must compile & test under both:
  - `cargo test --features postgres`
  - `cargo test --features sqlite`
- **Migrations**:
  - PG: `sqlx migrate run --source migrations/postgres`
  - SQLite: `sqlx migrate run --source migrations/sqlite`
- **Security**:
  - Never log plaintext account tokens.
  - Store only **argon2** (or equivalent) hashes.
- **Commits**:
  - Use the `commit_message_template` from each task when present.
  - Commit locally at task end; **do not push** unless explicitly directed.

Branch naming (suggested): `codex/<task_id>` — e.g., `codex/01_scaffold`.

---

## Execution order

- [ ] **01_scaffold** — Project scaffold: Cargo, features, deps, `/healthz`
  `codex/TASKS/01_scaffold.yaml`
  **Verify:** `cargo run --features sqlite` → `GET /healthz == "ok"`

- [ ] **02_db_bootstrap** — DB connector + migrator wiring (feature-gated)
  `codex/TASKS/02_db_bootstrap.yaml`
  **Verify:** `cargo check --features {postgres,sqlite}` passes

- [ ] **03_migrations_pg** — Postgres migrations (accounts, links, settings, link_visits)
  `codex/TASKS/03_migrations_pg.yaml`
  **Verify:** `sqlx migrate run --source migrations/postgres` succeeds

- [ ] **04_migrations_sqlite** — SQLite migrations (parity with PG)
  `codex/TASKS/04_migrations_sqlite.yaml`
  **Verify:** `sqlx migrate run --source migrations/sqlite` succeeds

- [ ] **05_models_and_types** — DTOs & shared types
  `codex/TASKS/05_models_and_types.yaml`
  **Verify:** `cargo check --features {postgres,sqlite}`

- [ ] **06_slug_policy_util** — Regex → URL-safe charset + min/max
  `codex/TASKS/06_slug_policy_util.yaml`
  **Verify:** `cargo test --features sqlite -- slug_policy`

- [ ] **07_create_link_endpoint** — `POST /shorten` (new/existing account flow)
  `codex/TASKS/07_create_link_endpoint.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- create_link`

- [ ] **08_redirect_hot_path** — `GET /{slug}` with cache + `count_only`
  `codex/TASKS/08_redirect_hot_path.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- redirect`

- [ ] **09_login_and_session** — Token login (signed cookie) + `/me/links` skeleton
  `codex/TASKS/09_login_and_session.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- session`

- [ ] **10_links_crud_ui** — `/me/links` SSR CRUD + cache invalidation
  `codex/TASKS/10_links_crud_ui.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- links_crud`

- [ ] **11_analytics_full** — Visits table + anonymization toggles
  `codex/TASKS/11_analytics_full.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- analytics_full`

- [ ] **12_admin_settings** — Admin settings (regex, analytics mode, reserved slugs)
  `codex/TASKS/12_admin_settings.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- admin`

- [ ] **13_security_hardening** — Rate limit, proxy trust, CSP, reserved slugs
  `codex/TASKS/13_security_hardening.yaml`
  **Verify:** `cargo test --features {postgres,sqlite} -- security`

- [ ] **14_observability_ci** — Request tracing + CI matrix (pg/sqlite)
  `codex/TASKS/14_observability_ci.yaml`
  **Verify:** CI passes for both features

- [ ] **15_docker_and_compose** — Multi-stage Dockerfile + Compose (PG) + Make targets
  `codex/TASKS/15_docker_and_compose.yaml`
  **Verify:** `docker compose up -d` → `GET /healthz == "ok"`

- [ ] **16_load_tests** — k6/wrk scenarios & baseline PERF.md
  `codex/TASKS/16_load_tests.yaml`
  **Verify:** k6 scripts run; record baseline in `docs/PERF.md`

- [ ] **17_docs_ops** — README/OPERATIONS/SECURITY
  `codex/TASKS/17_docs_ops.yaml`
  **Verify:** Docs lint clean; cover pg/sqlite instructions and security posture

- [ ] **18_polish_and_qol** — Reserved slugs mgmt, optional URL probe, UX copy
  `codex/TASKS/18_polish_and_qol.yaml`
  **Verify:** `cargo test --features postgres -- url_validation` (and spot-check UI text)

---

## Quick invocation cheats

```bash
# Format & lint
cargo fmt --all
cargo clippy -- -D warnings

# Test matrix
cargo test --features postgres
cargo test --features sqlite

# Migrations
export DATABASE_URL="$SURL_DATABASE_URL"
sqlx database create
sqlx migrate run --source migrations/postgres   # or migrations/sqlite
````

> **Output requirement (per task):** print the **commit message**, a **summary of modified files**, and `git status` after committing.
