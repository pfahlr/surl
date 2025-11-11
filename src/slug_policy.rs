use regex::Regex;

/// RFC3986 path-safe: ALPHA / DIGIT / "-" / "." / "_" / "~"
#[allow(dead_code)]
pub const URL_SAFE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SlugPolicy {
  pub allow_chars: Vec<char>,
  pub min_len: usize,
  pub max_len: usize,
}

#[allow(dead_code)]
pub fn from_regex(regex_str: &str) -> SlugPolicy {
  // Very light inference: if we can find {m,n}, take those; else fallback 5..=10
  let re = Regex::new(r"\{(\d+),(\d+)\}").ok();
  let (min_len, max_len) = re
    .and_then(|re| {
      re.captures(regex_str)
        .map(|c| (c[1].parse().ok(), c[2].parse().ok()))
    })
    .and_then(|(a, b)| match (a, b) {
      (Some(x), Some(y)) => Some((x, y)),
      _ => None,
    })
    .unwrap_or((5, 10));

  // Intersect with URL_SAFE
  let allow_chars: Vec<char> = URL_SAFE.chars().collect();
  // (For bootstrap we accept URL_SAFE outright; future task will refine by classes in regex.)
  SlugPolicy {
    allow_chars,
    min_len,
    max_len,
  }
}
