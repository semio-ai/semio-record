use indexmap::IndexMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::unfrozen::{Structure, StructureField};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "structure_V0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  /// `IndexMap` rather than `HashMap` so that field order is preserved during
  /// serialization. See the unfrozen variant for the full rationale.
  pub fields: IndexMap<Uuid, StructureField>,
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

impl From<Public> for Structure {
  fn from(public: Public) -> Self {
    Self {
      name: public.name,
      parent: public.parent,
      fields: public.fields,
      ..Default::default()
    }
  }
}
