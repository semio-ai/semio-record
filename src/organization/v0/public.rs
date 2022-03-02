
use serde::{Serialize, Deserialize};

use super::unfrozen::Organization;

#[derive(Debug, Serialize, Deserialize)]
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