use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  acl::action::with_acl,
  acl::Acl,
  action::{name, parent},
  blob::BlobDependencies,
  migrate::Migrate,
  record::{Freeze, Freezer, Unfrozen, UnfrozenReference, View},
  ty::UnfrozenTy,
  unfrozen::impl_unfrozen,
};

use super::{action::Action, frozen};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Variant")]
pub struct EnumerationVariant {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: UnfrozenTy,
}

#[async_trait]
impl<F: Freezer> Freeze<F> for EnumerationVariant {
  type Frozen = frozen::EnumerationVariant;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(frozen::EnumerationVariant {
      name: self.name.clone(),
      ty: self.ty.freeze(freezer).await?,
    })
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "enumeration_V0_Private")]
pub struct Enumeration {
  pub parent: Uuid,
  pub name: String,
  pub acl: Acl,
  pub variants: HashMap<Uuid, EnumerationVariant>,
}

with_acl!(Enumeration);

impl Default for Enumeration {
  fn default() -> Self {
    Self {
      parent: Uuid::default(),
      name: "".to_string(),
      acl: Default::default(),
      variants: HashMap::new(),
    }
  }
}

impl_unfrozen!(Enumeration, Action);

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

impl View for Enumeration {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

name!(Enumeration);
parent!(Enumeration);

impl Unfrozen<Action> for Enumeration {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for field in self.variants.values() {
      field.ty.dependencies(set);
    }
  }
}

impl BlobDependencies for Enumeration {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Enumeration {
  type Frozen = frozen::Enumeration;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut variants = HashMap::new();
    for (id, variant) in &self.variants {
      variants.insert(id.clone(), variant.freeze(freezer).await?);
    }
    Ok(frozen::Enumeration {
      parent: self.parent.clone(),
      name: self.name.clone(),
      variants,
    })
  }
}

impl Migrate for Enumeration {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}
