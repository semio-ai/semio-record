use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

use super::unfrozen::User;

#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
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