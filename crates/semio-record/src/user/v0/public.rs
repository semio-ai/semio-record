use serde::{Serialize, Deserialize};

use super::unfrozen::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "user_V0_Public", rename_all = "camelCase")]
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

impl From<Public> for User {
  fn from(public: Public) -> Self {
    Self {
      user_name: public.user_name,
      ..Default::default()
    }
  }
}