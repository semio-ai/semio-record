use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::unfrozen::{Enumeration, EnumerationVariant, IdEnumerationVariant};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[graphql_object(name = "EnumerationPublic")]
impl Public {
  fn name(&self) -> &str {
    &self.name
  }

  fn parent(&self) -> &Uuid {
    &self.parent
  }

  fn variants(&self) -> Vec<IdEnumerationVariant> {
    self
      .variants
      .iter()
      .map(|(id, variant)| IdEnumerationVariant {
        id: id.clone(),
        variant: variant.clone(),
      })
      .collect()
  }

  #[graphql(name = "variantNamed")]
  fn gql_variant_named(&self, name: String) -> Option<&EnumerationVariant> {
    self.variant_named(&name)
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
