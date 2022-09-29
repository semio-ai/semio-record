use serde::{Serialize, Deserialize};

use super::unfrozen::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "user_v0_Private", rename_all = "camelCase")]
pub struct Private {
  pub user_name: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub email_verified: bool,
}

impl From<User> for Private {
  fn from(user: User) -> Self {
    Self {
      user_name: user.user_name,
      first_name: user.first_name,
      last_name: user.last_name,
      email: user.email,
      email_verified: user.email_verified,
    }
  }
}

impl From<Private> for User {
  fn from(private: Private) -> Self {
    Self {
      user_name: private.user_name,
      first_name: private.first_name,
      last_name: private.last_name,
      email: private.email,
      email_verified: private.email_verified,
      ..Default::default()
    }
  }
}