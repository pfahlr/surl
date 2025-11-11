use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLinkForm {
  pub url: String,
  #[serde(default)]
  pub account_token: Option<String>,
}

// Add row models later as needed (Accounts, Links, Settings, LinkVisits)
