use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use schemars::JsonSchema;

use super::unfrozen::{IdStructureField, Structure, StructureField};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
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

#[graphql_object(name = "StructurePublic")]
impl Public {
  fn name(&self) -> &str {
    &self.name
  }

  fn parent(&self) -> &Uuid {
    &self.parent
  }

  fn fields(&self) -> Vec<IdStructureField> {
    self
      .fields
      .iter()
      .map(|(id, field)| IdStructureField {
        id: id.clone(),
        field: field.clone(),
      })
      .collect()
  }

  #[graphql(name = "fieldNamed")]
  fn gql_field_named(&self, name: String) -> Option<&StructureField> {
    self.field_named(&name)
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
