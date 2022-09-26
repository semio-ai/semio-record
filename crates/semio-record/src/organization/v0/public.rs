
use serde::{Serialize, Deserialize};

use super::unfrozen::Organization;

use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Public {
  pub name: String,
}

#[graphql_object(name = "OrganizationPublic")]
impl Public {
  fn name(&self) -> &str {
    &self.name
  }
}

impl From<Organization> for Public {
  fn from(organization: Organization) -> Self {
    Self {
      name: organization.name,
    }
  }
}