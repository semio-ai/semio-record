use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{UnfrozenReference, FrozenReference}, blob::BlobDependencies};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unfrozen_Container", tag = "type", content = "value", rename_all = "camelCase")]
pub enum UnfrozenContainer<T> {
  Local(T),
  Reference(UnfrozenReference),
}

impl<T> UnfrozenContainer<T> {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      UnfrozenContainer::Local(_) => {},
      UnfrozenContainer::Reference(reference) => {
        set.insert(reference);
      }
    }
  }
}

impl<T> UnfrozenContainer<T> where
  T: BlobDependencies
{
  pub fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    match self {
      UnfrozenContainer::Local(value) => {
        value.blob_dependencies(set);
      },
      UnfrozenContainer::Reference(_) => {}
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Container", tag = "type", content = "value", rename_all = "camelCase")]
pub enum FrozenContainer<T> {
  Local(T),
  Reference(FrozenReference),
}

impl<T> FrozenContainer<T> {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      FrozenContainer::Local(_) => {},
      FrozenContainer::Reference(reference) => {
        set.insert(reference);
      }
    }
  }
}