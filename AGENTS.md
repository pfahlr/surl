
# AGENTS.md — Codex Execution Policy for Shorty

## Role
You are the **CODEX Agent** working in a monorepo under strict **TDD** and **Spec-as-Source-of-Truth** rules. Your job is to complete each task in `codex/TASKS/*.yaml` **end-to-end**.

## Mandatory Rules
1. **TDD First**
   - Write or update **failing tests** before implementation.
   - Keep tests **deterministic** and fast; prefer unit/integration over e2e unless necessary.
2. **Scope Discipline**
   - Implement only what is required to make the current tests pass.
   - Do not pre-emptively add features.
3. **Compile-Time DB Features**
   - This project supports **two compile-time DB backends** via Cargo features: `postgres` and `sqlite`.
   - All code must compile and tests must pass under **both** features unless a task explicitly narrows scope.
4. **Build & Test Matrix**
   - Run `cargo fmt --check` and `cargo clippy -D warnings`.
   - Run `cargo test --features postgres` and `cargo test --features sqlite` for tasks affecting code.
   - For migrations, run `sqlx migrate run` per dialect.
5. **Security**
   - **Never log plaintext account tokens** or other secrets. Redact sensitive fields.
   - Store only **argon2 hashes** of tokens.
6. **Commit Discipline**
   - Prepare a Git commit with **only** relevant changes (no temp files, no unrelated edits).
   - Use the `commit_message_template` from the task when provided.
   - **Commit locally** at the end of each task. **Do not push** unless explicitly instructed.
7. **Output**
   - At task completion, output:
     - The **full commit message** used.
     - A **summary of modified files**.
     - A **`git status`** report.

## Suggested Commands
- Formatting & lint:
  - `cargo fmt --all`
  - `cargo clippy -- -D warnings`
- Test matrix:
  - `cargo test --features postgres`
  - `cargo test --features sqlite`
- Migrations:
  - PG: `sqlx database create --database-url $DATABASE_URL && sqlx migrate run --source migrations/postgres`
  - SQLite: `sqlx database create --database-url $DATABASE_URL && sqlx migrate run --source migrations/sqlite`

## Libraries & Building Blocks
- Web: `axum`, `tower`, `tower-http`
- Async: `tokio`
- DB: `sqlx` (PgPool/SqlitePool)
- Cache: `moka`
- Auth/Hash: `argon2`
- Regex/Validation: `regex`, `url`
- Sessions/Cookies: `axum-extra` or `cookie`
- Security: `tower-governor`, `ipnet`
- Observability: `tracing`, `tracing-subscriber`

## Quality Bar
- Keep functions small and focused; prefer pure helpers.
- Add **doc comments** for non-trivial utils (e.g., slug policy intersection rules).
- Ensure **feature parity** for `postgres` and `sqlite` builds.
- Prefer **dialect-specific SQL** behind `#[cfg]` where it improves clarity or performance.

## Failure Handling
- If a test fails, **tighten** the test or implementation; do not disable checks.
- If a migration conflicts, create a new migration; never edit an applied migration.

---

Proceed task-by-task, keeping commits tightly scoped and verifiable.
