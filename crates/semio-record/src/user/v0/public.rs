use serde::{Serialize, Deserialize};

use super::unfrozen::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "user_v0_Public", rename_all = "camelCase")]
pub struct Public {
  pub user_name: String,
}

impl From<User> for Public {
  fn from(user: User) -> Self {
    Self {
      user_name: user.user_name,
    }
  }
}