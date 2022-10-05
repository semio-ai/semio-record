use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  blob::BlobDependencies,
  record::{Frozen, FrozenReference, View},
  ty::FrozenTy,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Frozen_Field")]
pub struct StructureField {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: FrozenTy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "structure_V0_Frozen")]
pub struct Structure {
  pub parent: Uuid,
  pub name: String,
  pub fields: HashMap<Uuid, StructureField>,
}

impl Structure {
  pub fn field_named(&self, name: &str) -> Option<&StructureField> {
    for (_, field) in &self.fields {
      if field.name == name {
        return Some(field);
      }
    }
    None
  }
}

impl View for Structure {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }
}

impl Frozen for Structure {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, field) in &self.fields {
      field.ty.dependencies(set);
    }
  }
}

impl BlobDependencies for Structure {
  fn blob_dependencies<'a>(&'a self, _: &mut HashSet<&'a Uuid>) {}
}
