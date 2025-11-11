# AGENTS.md — Codex Execution Policy for Shorty

## Role

You are the **CODEX Agent** working in a monorepo under strict **TDD** and **Spec-as-Source-of-Truth** rules. Your task is to complete each instruction in `codex/TASKS/*.yaml` **end-to-end**, using automated tests as the definitive measure of correctness and progress.

---

## 🔁 Development Paradigm: Test-Driven Feedback Loop

Codex must follow a **Test-Driven Development (TDD)** lifecycle for **every iteration**:

1. **Begin with Tests**

   * Derive expected behavior from the task spec.
   * Write **failing tests** that clearly capture success criteria.
   * Mark test TODOs if full behavior is not yet implemented.

2. **Write Minimal Implementation**

   * Only implement logic sufficient to pass the current test(s).
   * No speculative coding or out-of-scope functionality.

3. **Run Tests Continuously**

   * Use test output to **drive implementation changes**.
   * If a test fails, **analyze the cause**, then revise only the minimal amount of code to resolve it.

4. **Treat Test Failures as Signals**

   * Use test failures and coverage gaps as **feedback triggers** for the next code iteration.
   * If test coverage is insufficient to prevent regression or ambiguity, **pause to improve the tests**.

5. **Test-Guided Refactoring**

   * Once all tests pass, suggest or perform **safe refactors** without breaking coverage.
   * Any optimization or code cleanup must preserve test integrity.

6. **Always verify both compile-time DB targets** (`postgres`, `sqlite`) before and after changes.

---

## ✅ Mandatory Rules

1. **TDD First**

   * Write or update **failing tests** before any implementation.
   * Prefer fast, isolated unit/integration tests over E2E unless explicitly required.
   * Use test results as **feedback checkpoints**.

2. **Scope Discipline**

   * Implement only what is necessary to make **current tests pass**.
   * Never implement features or paths not covered by failing or TODO-marked tests.

3. **Compile-Time DB Features**

   * All changes must **build and test cleanly** under both Cargo features: `postgres` and `sqlite`.
   * Use `#[cfg]` selectively when dialect-specific SQL provides clarity or performance.

4. **Build & Test Matrix**

   * Formatting: `cargo fmt --check`
   * Linting: `cargo clippy -D warnings`
   * Tests:

     * `cargo test --features postgres`
     * `cargo test --features sqlite`
   * Migrations:

     * PG: `sqlx migrate run --source migrations/postgres`
     * SQLite: `sqlx migrate run --source migrations/sqlite`

5. **Security**

   * Never log or persist plaintext tokens. Always redact.
   * Use **argon2** for token hashing. Never log token contents.

6. **Commit Discipline**

   * Commit only relevant, scoped changes.
   * Follow the `commit_message_template` from each task if provided.
   * Do not push code unless explicitly instructed.

7. **Output**

   * At task completion, output the following:

     * ✅ **Full commit message**
     * 📂 **Summary of modified files**
     * 🔍 **`git status` output**
     * 🧪 **Test suite results**, with notes on added tests, updated tests, and remaining gaps

---

## ⚙ Suggested Commands

```sh
# Formatting & Lint
cargo fmt --all
cargo clippy -- -D warnings

# Test Matrix
cargo test --features postgres
cargo test --features sqlite

# Migrations
sqlx database create --database-url $DATABASE_URL
sqlx migrate run --source migrations/postgres

sqlx database create --database-url $DATABASE_URL
sqlx migrate run --source migrations/sqlite
```

---

## 📚 Libraries & Building Blocks

* Web: `axum`, `tower`, `tower-http`
* Async: `tokio`
* DB: `sqlx` (`PgPool`, `SqlitePool`)
* Cache: `moka`
* Auth/Hash: `argon2`
* Validation: `regex`, `url`
* Sessions: `axum-extra`, `cookie`
* Security: `tower-governor`, `ipnet`
* Observability: `tracing`, `tracing-subscriber`

---

## 🧪 Quality & Style

* Keep functions short, focused, and pure where possible.
* Add **doc comments** for utility functions, particularly ones implementing domain-specific policies (e.g., slug rules).
* Maintain **feature parity** between dialects unless scope explicitly diverges.
* Prefer dialect-specific SQL behind `#[cfg]` guards when it enhances clarity or performance.

---

## 🧨 Failure Handling

* **Tighten tests or implementation** on test failure—do not disable tests or checks.
* For migration conflicts, create new migration files; **do not modify applied migrations**.

---

Proceed task-by-task. Use tests as your compass. Keep commits surgical, outputs complete, and behavior traceable through specs and test cases.
