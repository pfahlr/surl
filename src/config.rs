use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
  pub addr: String,
  pub database_url: String,
  pub pool_max: u32,
  #[allow(dead_code)]
  pub force_status_301: bool,
  #[allow(dead_code)]
  pub reserved_slugs: Vec<String>,
  #[allow(dead_code)]
  pub analytics_mode: String, // none | count_only | full (parsed later)
  #[allow(dead_code)]
  pub slug_regex: String,
  #[allow(dead_code)]
  pub ip_anonymize: bool,
  #[allow(dead_code)]
  pub proxy_trust_cidrs: Vec<String>,
  #[allow(dead_code)]
  pub admin_token: String,
}

impl AppConfig {
  pub fn from_env() -> anyhow::Result<Self> {
    // `config` crate lets you layer .env + env vars if desired, but for bootstrap
    // we’ll read directly to keep it simple.
    let addr = env::var("SURL_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let database_url = env::var("SURL_DATABASE_URL")
      .unwrap_or_else(|_| "sqlite:////dev/shm/surl.sqlite?mode=rwc".into());
    let pool_max = env::var("SURL_POOL_MAX")
      .ok()
      .and_then(|s| s.parse().ok())
      .unwrap_or(16);
    let force_status_301 = env::var("SURL_FORCE_STATUS_301")
      .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
      .unwrap_or(true);
    let reserved_slugs = env::var("SURL_RESERVED_SLUGS")
      .unwrap_or_else(|_| "admin,login,me,shorten,healthz,assets,static,api".into())
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();
    let analytics_mode = env::var("SURL_ANALYTICS_MODE").unwrap_or_else(|_| "count_only".into());
    let slug_regex = env::var("SURL_SLUG_REGEX").unwrap_or_else(|_| "^[A-Za-z0-9]{5,10}$".into());
    let ip_anonymize = env::var("SURL_IP_ANONYMIZE")
      .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
      .unwrap_or(true);
    let proxy_trust_cidrs = env::var("SURL_PROXY_TRUST_CIDRS")
      .unwrap_or_else(|_| "127.0.0.1/32".into())
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();
    let admin_token = env::var("SURL_ADMIN_TOKEN").unwrap_or_else(|_| "change-me".into());

    Ok(Self {
      addr,
      database_url,
      pool_max,
      force_status_301,
      reserved_slugs,
      analytics_mode,
      slug_regex,
      ip_anonymize,
      proxy_trust_cidrs,
      admin_token,
    })
  }
}
