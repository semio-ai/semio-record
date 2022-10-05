use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{ty::FrozenTy, record::{View, Frozen, FrozenReference}, blob::BlobDependencies};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Frozen_Variant", rename_all = "camelCase")]
pub struct EnumerationVariant {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: FrozenTy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Frozen", rename_all = "camelCase")]
pub struct Enumeration {
  pub parent: Uuid,
  pub name: String,
  pub variants: HashMap<Uuid, EnumerationVariant>,
}

impl Enumeration {
  pub fn variant_named(&self, name: &str) -> Option<&EnumerationVariant> {
    for (_, variant) in &self.variants {
      if variant.name == name {
        return Some(variant);
      }
    }
    None
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdEnumerationVariant {
  pub id: Uuid,
  pub variant: EnumerationVariant,
}

impl View for Enumeration {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }
}

impl Frozen for Enumeration {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, variant) in &self.variants {
      variant.ty.dependencies(set);
    }
  }
}

impl BlobDependencies for Enumeration {
  fn blob_dependencies<'a>(&'a self, _: &mut HashSet<&'a Uuid>) {
    
  }
}