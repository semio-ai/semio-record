use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

use super::unfrozen::User;

use schemars::JsonSchema;

#[derive(Debug, Clone, GraphQLObject, Serialize, Deserialize, JsonSchema)]
#[graphql(name = "UserPublic")]
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