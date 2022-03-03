use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

use super::unfrozen::User;

#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
#[graphql(name = "UserPrivate")]
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