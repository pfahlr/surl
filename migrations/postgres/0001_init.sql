-- SURL Postgres schema (v1)

CREATE TABLE IF NOT EXISTS accounts (
  id           BIGSERIAL PRIMARY KEY,
  token_hash   BYTEA        NOT NULL,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS links (
  id           BIGSERIAL PRIMARY KEY,
  account_id   BIGINT       NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  slug         TEXT         NOT NULL UNIQUE,
  target_url   TEXT         NOT NULL,
  hit_count    BIGINT       NOT NULL DEFAULT 0,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS settings (
  id               INT PRIMARY KEY DEFAULT 1,
  slug_regex       TEXT NOT NULL DEFAULT '^[A-Za-z0-9]{5,10}$',
  analytics_mode   TEXT NOT NULL DEFAULT 'count_only', -- none | count_only | full
  reserved_slugs   TEXT NOT NULL DEFAULT 'admin,login,me,shorten,healthz,assets,static,api',
  ip_anonymize     BOOLEAN NOT NULL DEFAULT TRUE
);

INSERT INTO settings (id)
  VALUES (1)
  ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS link_visits (
  id         BIGSERIAL PRIMARY KEY,
  link_id    BIGINT       NOT NULL REFERENCES links(id) ON DELETE CASCADE,
  ts         TIMESTAMPTZ  NOT NULL DEFAULT now(),
  ip         TEXT,
  user_agent TEXT,
  referer    TEXT
);

CREATE INDEX IF NOT EXISTS idx_link_visits_link_ts ON link_visits (link_id, ts);
