-- SURL SQLite schema (v1)

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS accounts (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  token_hash   BLOB    NOT NULL,
  created_at   TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS links (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  account_id   INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  slug         TEXT    NOT NULL UNIQUE,
  target_url   TEXT    NOT NULL,
  hit_count    INTEGER NOT NULL DEFAULT 0,
  created_at   TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at   TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS settings (
  id               INTEGER PRIMARY KEY CHECK (id = 1),
  slug_regex       TEXT NOT NULL DEFAULT '^[A-Za-z0-9]{5,10}$',
  analytics_mode   TEXT NOT NULL DEFAULT 'count_only', -- none | count_only | full
  reserved_slugs   TEXT NOT NULL DEFAULT 'admin,login,me,shorten,healthz,assets,static,api',
  ip_anonymize     INTEGER NOT NULL DEFAULT 1
);

INSERT OR IGNORE INTO settings (id) VALUES (1);

CREATE TABLE IF NOT EXISTS link_visits (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  link_id    INTEGER NOT NULL REFERENCES links(id) ON DELETE CASCADE,
  ts         TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  ip         TEXT,
  user_agent TEXT,
  referer    TEXT
);

CREATE INDEX IF NOT EXISTS idx_link_visits_link_ts ON link_visits (link_id, ts);
