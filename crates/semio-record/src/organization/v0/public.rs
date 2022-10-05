
use serde::{Serialize, Deserialize};

use super::unfrozen::Organization;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "organization_V0_Public")]
pub struct Public {
  pub name: String,
}

impl From<Organization> for Public {
  fn from(organization: Organization) -> Self {
    Self {
      name: organization.name,
    }
  }
}

impl From<Public> for Organization {
  fn from(public: Public) -> Self {
    Self {
      name: public.name,
      ..Default::default()
    }
  }
}