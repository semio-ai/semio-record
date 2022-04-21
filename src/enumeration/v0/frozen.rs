use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{ty::FrozenTy, record::{View, Frozen, FrozenReference}, blob::BlobDependencies};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, PartialEq, Eq)]
#[graphql(name = "FrozenEnumerationVariant")]
pub struct EnumerationVariant {
  pub name: String,
  #[serde(rename = "type")]
  #[graphql(name = "type")]
  pub ty: FrozenTy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "FrozenIdEnumerationVariant")]
pub struct IdEnumerationVariant {
  pub id: Uuid,
  pub variant: EnumerationVariant,
}

#[graphql_object(name = "FrozenEnumeration")]
impl Enumeration {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn parent(&self) -> &Uuid {
    &self.parent
  }

  pub fn variants(&self) -> Vec<IdEnumerationVariant> {
    self.variants.iter().map(|(id, variant)| IdEnumerationVariant {
      id: id.clone(),
      variant: variant.clone(),
    }).collect()
  }

  #[graphql(name = "variantNamed")]
  pub fn gql_variant_named(&self, name: String) -> Option<&EnumerationVariant> {
    self.variant_named(&name)
  }
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