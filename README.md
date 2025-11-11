# SURL — SURL URL Redirector for Links

Ultra-fast Rust link shortener with cache-first redirects, compile-time-selectable databases (Postgres **or** SQLite), admin-configurable slug policy, and privacy-aware analytics.

> **Why SURL?** Single binary, no heavy JS, no traditional accounts—just a user-kept token to manage links. Built for speed and simplicity.

---

## Features

- ⚡ **Blazing hot path**: cache-first lookup → optional analytics → 301/302 redirect
- 🧰 **Dual DB builds** (compile-time): `--features postgres` (default) or `--no-default-features --features sqlite`
- 🧪 **SQLx macros**: compile-time checked queries per backend
- 🔤 **Admin slug policy**: regex-driven; intersected with URL-safe chars
- 🔍 **Analytics modes**: `none`, `count_only`, `full` (IP/UA/Referer w/ anonymization)
- 🔑 **Account-lite**: user supplies/receives a random token to manage their links
- 🚫 **Reserved slugs**: configurable blocklist
- 🧱 **SSR-first UI**: simple forms; optional htmx sprinkle
- 📦 **Docker-ready**: separate images for PG and SQLite builds

---

## Quickstart

### 1) Prereqs
- Rust 1.81+  
- For Postgres: a running PG 16+ and `sqlx-cli` (`cargo install sqlx-cli`)  
- For SQLite: nothing extra (uses file/`tmpfs` path)

### 2) Clone & build
```bash
git clone https://github.com/pfahlr/surl.git
cd surl
```

#### Build (Postgres)

```bash
cargo build --release --features postgres
```

#### Build (SQLite)

```bash
cargo build --release --no-default-features --features sqlite
```

### 3) Configure env

Create `.env` (example):

```env
SURL_ADDR=0.0.0.0:8080
SURL_DATABASE_URL=postgres://postgres:postgres@localhost:5432/surl
SURL_POOL_MAX=16
SURL_FORCE_STATUS_301=true
SURL_RESERVED_SLUGS=admin,login,me,shorten,healthz,assets,static,api
SURL_ANALYTICS_MODE=count_only
SURL_SLUG_REGEX=^[A-Za-z0-9]{5,10}$
SURL_IP_ANONYMIZE=true
SURL_PROXY_TRUST_CIDRS=127.0.0.1/32
SURL_ADMIN_TOKEN=change-me
```

**SQLite on tmpfs (fast demo):**

```env
SURL_DATABASE_URL=sqlite:////dev/shm/surl.sqlite?mode=rwc
```

### 4) Migrate database

**Postgres**

```bash
export DATABASE_URL="$SURL_DATABASE_URL"
sqlx database create
sqlx migrate run --source migrations/postgres
```

**SQLite**

```bash
export DATABASE_URL="$SURL_DATABASE_URL"
sqlx database create
sqlx migrate run --source migrations/sqlite
```

### 5) Run

**Postgres**

```bash
SURL_ADDR=0.0.0.0:8080 \
SURL_DATABASE_URL=postgres://postgres:postgres@localhost:5432/surl \
SURL_POOL_MAX=16 \
cargo run --features postgres
```

**SQLite**

```bash
SURL_ADDR=0.0.0.0:8080 \
SURL_DATABASE_URL=sqlite:////dev/shm/surl.sqlite?mode=rwc \
SURL_POOL_MAX=8 \
cargo run --no-default-features --features sqlite
```

Open: [http://localhost:8080/healthz](http://localhost:8080/healthz) → `ok`

### Docker Compose (PG)

Use the provided `compose.yaml` to start a Postgres database plus the app with matching env defaults:

```bash
docker compose up -d
curl -sf http://localhost:8080/healthz
```

---

## Usage

### Create a short link (no token supplied → new token returned once)

```bash
curl -X POST http://localhost:8080/shorten \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'url=https://example.com/some/long/path'
```

**Response (example)**

```
slug: a8K9P
account_token: h9Y5p... (save this; not shown again)
```

### Create under an existing account token

```bash
curl -X POST http://localhost:8080/shorten \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'url=https://example.com/x&account_token=h9Y5p...'
```

### Redirect

```
GET /a8K9P  →  301 Location: https://example.com/some/long/path
```

### Login (token)

* UI: Top-right **Login** → paste token → `/me/links`
* Lists links with actions: `[edit] [delete] [stats]`, plus create form

---

## API (HTTP)

* `GET /healthz` → 200 `"ok"`
* `GET /` → Landing form (url + optional account token)
* `POST /shorten` (form)

  * `url`: string (absolute `http`/`https`)
  * `account_token?`: string
  * Returns: `slug` and (if new) `account_token`
* `GET /{slug}` → 301/302 redirect (`SURL_FORCE_STATUS_301`)
* `POST /login` (form) → Sets signed, httpOnly session cookie
* `GET /me/links` → SSR list + create form
* `POST /me/links` → Create under session account
* `POST /me/links/{id}/edit` → Update target URL
* `POST /me/links/{id}/delete` → Delete link
* `GET /me/links/{id}/stats` → Basic stats (depends on analytics mode)

### Admin (header-guarded)

* `GET /admin/settings`
* `POST /admin/settings`

  * Header `X-SURL-Admin-Token: <SURL_ADMIN_TOKEN>`
  * Body fields:

    * `slug_regex`
    * `analytics_mode` (`none|count_only|full`)
    * `reserved_slugs` (CSV)
    * `ip_anonymize` (`true|false`)

---

## Slug Policy

* Admin supplies `SURL_SLUG_REGEX` (default `^[A-Za-z0-9]{5,10}$`).
* SURL **intersects** regex character classes with RFC3986 path-safe chars:
  `[A–Z a–z 0–9 - . _ ~]` (anything else is ignored)
* `min_len`/`max_len` inferred from the regex.
* On creation:

  * Random slug generation from `allow_chars` (retry up to 8 on collision, else `409`).
  * Reserved slugs rejected.

---

## Analytics

**Modes**

* `none` — no write
* `count_only` — `hit_count++` per visit (one fast `UPDATE`)
* `full` — write to `link_visits` with toggles:

  * `ip` (anonymized if `SURL_IP_ANONYMIZE=true`; IPv4 /24, IPv6 /64)
  * `user_agent`
  * `referer`

**Stats UI**

* `count_only`: totals
* `full`: daily counts + optional UA breakdown

---

## Build Matrix (compile-time DB)

Cargo features keep SQLx macros and per-DB fast paths:

```toml
[features]
default = ["postgres"]
postgres = ["sqlx/postgres", "sqlx/runtime-tokio-rustls"]
sqlite   = ["sqlx/sqlite",   "sqlx/runtime-tokio-rustls"]
```

* **Postgres build**

  ```bash
  cargo build --release --features postgres
  ```
* **SQLite build**

  ```bash
  cargo build --release --no-default-features --features sqlite
  ```

> Postgres is the default feature, so always pass `--no-default-features` when targeting SQLite-only builds.

---

## Docker

**Multi-stage Dockerfile** builds either target via `--build-arg DB=postgres|sqlite`.

```bash
# Postgres image
docker build -t surl:pg --build-arg DB=postgres .

# SQLite image
docker build -t surl:sqlite --build-arg DB=sqlite .
```

**compose.yaml (PG)** — run with `docker compose up -d` for a full Postgres stack.

```yaml
version: "3.9"
services:
  db:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: surl
    ports: ["5432:5432"]

  app:
    build:
      context: .
      args: { DB: postgres }
    environment:
      SURL_ADDR: "0.0.0.0:8080"
      SURL_DATABASE_URL: "postgres://postgres:postgres@db:5432/surl"
      SURL_POOL_MAX: "16"
      SURL_FORCE_STATUS_301: "true"
    ports: ["8080:8080"]
    depends_on: [db]
```

---

## Security & Privacy

* **Tokens**: only **argon2** hashes stored. Plaintext token is shown **once** to the user.
* **Cookies/sessions**: signed httpOnly cookie; never store token in cookie.
* **Logging**: redact secrets/token-like values in request logs.
* **Rate limits**: apply to `POST /shorten`, login, edit/delete endpoints.
* **Proxy trust**: respect `X-Forwarded-For` **only** from `SURL_PROXY_TRUST_CIDRS`.
* **CSP**: minimal locked-down policy.
* **Analytics**: `none` disables all writes; `full` supports IP anonymization.

---

## Performance

* **Cache**: Moka cache `(slug → (id, target_url))`, invalidate on edit/delete.
* **SQLite tips**: WAL + `synchronous=NORMAL`; fine for demos/small sites; prefer PG in prod.
* **Indices**: unique `links.slug`, time-series index for `link_visits (link_id, ts)` if heavy stats.
* **Load testing**: k6 scripts for cached/cold redirects and create bursts.

---

## Development

### Repo layout (suggested)

```
surl/
  src/
    main.rs
    config.rs
    db.rs
    models.rs
    slug_policy.rs
    routes/
      mod.rs
      public.rs
      account.rs
      session.rs
      admin.rs
  migrations/
    postgres/0001_init.sql
    sqlite/0001_init.sql
  templates/...
  .github/workflows/ci.yaml
```

### Commands

```bash
# Format & lint
cargo fmt --all
cargo clippy -- -D warnings

# Tests (both backends)
cargo test --features postgres
cargo test --no-default-features --features sqlite
```

### Makefile (optional)

```make
build-pg:
  cargo build --release --features postgres
build-sqlite:
  cargo build --release --no-default-features --features sqlite
run-pg:
  SURL_ADDR=0.0.0.0:8080 SURL_DATABASE_URL=postgres://... cargo run --features postgres
run-sqlite:
  SURL_ADDR=0.0.0.0:8080 SURL_DATABASE_URL=sqlite:////dev/shm/surl.sqlite?mode=rwc cargo run --no-default-features --features sqlite
```

---

## Libraries

Core:

* `axum`, `tower`, `tower-http`
* `tokio`
* `sqlx` (+ `sqlx-cli`), `moka`
* `regex`, `url`, `serde`
* `argon2`, `rand`, `uuid`
* `config`, `tracing`, `tracing-subscriber`

Optional/alternatives:

* **Templating**: `askama`, `maud`
* **Rate-limit**: `tower-governor`, `governor`
* **UA parsing**: `uaparser`
* **ORM** (alt): `SeaORM` (multi-DB), `Diesel` (strong types)
* **Bench**: `criterion`, `k6`, `wrk`

---

## Roadmap

* Per-link custom slugs (user-chosen)
* Expiration & soft-delete
* One-time/limited-use links
* QR code generation
* Simple REST API + API keys
* Webhook on click
* Geo/ASN stats (privacy-respecting)

---

## FAQ

**Why compile-time DB selection instead of runtime?**
Keeps SQLx compile-time checks and enables dialect-specific fast paths (`RETURNING`, PRAGMAs), with minimal overhead and clearer migrations.

**Will a 301 cache forever?**
Browsers/CDNs may cache. If you expect frequent target changes, set `SURL_FORCE_STATUS_301=false` to return a 302/307.

---

## License

MIT. See `LICENSE`. (Feel free to switch to Apache-2.0/MIT dual-license if you prefer.)

---

## Contributing

Issues and PRs welcome. Please keep changes small, include tests, and ensure both PG/SQLite builds pass:

```bash
cargo test --features postgres
cargo test --no-default-features --features sqlite
```
