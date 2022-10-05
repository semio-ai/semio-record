use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::unfrozen::{Enumeration, EnumerationVariant};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub variants: HashMap<Uuid, EnumerationVariant>,
}

impl Public {
  pub fn variant_named(&self, name: &str) -> Option<&EnumerationVariant> {
    for (_, variant) in &self.variants {
      if variant.name == name {
        return Some(variant);
      }
    }
    None
  }
}

impl From<Enumeration> for Public {
  fn from(enumeration: Enumeration) -> Self {
    Self {
      name: enumeration.name,
      parent: enumeration.parent,
      variants: enumeration.variants,
    }
  }
}

impl From<Public> for Enumeration {
  fn from(public: Public) -> Self {
    Self {
      name: public.name,
      parent: public.parent,
      variants: public.variants,
      ..Default::default()
    }
  }
}