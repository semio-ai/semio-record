use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::unfrozen::{Structure, StructureField};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "structure_v0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub fields: HashMap<Uuid, StructureField>,
}

impl Public {
  pub fn field_named(&self, name: &str) -> Option<&StructureField> {
    for (_, field) in &self.fields {
      if field.name == name {
        return Some(field);
      }
    }
    None
  }
}

impl From<Structure> for Public {
  fn from(structure: Structure) -> Self {
    Self {
      name: structure.name,
      parent: structure.parent,
      fields: structure.fields,
    }
  }
}
