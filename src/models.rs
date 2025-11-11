use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateLinkForm {
  pub url: String,
  #[serde(default)]
  pub account_token: Option<String>,
}

// Add row models later as needed (Accounts, Links, Settings, LinkVisits)
