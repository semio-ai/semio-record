use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{ty::Ty, entity::{Reference, Entity}};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationVariant {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: Ty,
}

impl EnumerationVariant {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    self.ty.dependencies(set)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enumeration {
  pub variants: HashMap<Uuid, EnumerationVariant>,
}

impl Entity for Enumeration {
  const NAME: &'static str = "enumeration";

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    for (_, variant) in &self.variants {
      variant.dependencies(set);
    }
  }
}