use std::{collections::{HashSet, HashMap}, error::Error};

use async_trait::async_trait;
use chrono::Duration;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::blob::BlobDependencies;

pub const TYPE_USER: i16 = 0x0000;
pub const TYPE_ORGANIZATION: i16 = 0x0001;

pub const TYPE_FOLDER: i16 = 0x0010;

pub const TYPE_MODULE: i16 = 0x0020;
pub const TYPE_STRUCTURE: i16 = 0x0021;
pub const TYPE_ENUMERATION: i16 = 0x0022;

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
  pub components: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Reference<V> {
  pub id: Uuid,
  pub version: V,
}

pub type FrozenReference = Reference<semver::Version>;
pub type UnfrozenReference = Reference<Option<semver::VersionReq>>;

pub trait Apply<T> {
  type Result;

  fn apply(&mut self, action: &T) -> Self::Result;
}

#[async_trait]
pub trait Freeze<F: Freezer> {
  type Frozen;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error>;
}

#[async_trait]
pub trait Freezer: Send + Sync {
  type Error: std::error::Error;

  async fn freeze(&self, reference: &UnfrozenReference) -> Result<FrozenReference, Self::Error>;
}

pub trait View {
  fn name<'a>(&'a self) -> Option<&'a str> { None }
  fn parent<'a>(&'a self) -> Option<&'a Uuid> { None }
}

pub trait Unfrozen<T, F: Freezer>: View + BlobDependencies + Apply<T> + Freeze<F> + Serialize + DeserializeOwned {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}
pub trait Frozen: View + BlobDependencies + Serialize + DeserializeOwned {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a FrozenReference>) {}
}



pub trait Record<F: Freezer> {
  const TYPE: i16;
  const SCHEMA_VERSION: i16;

  type Action;
  type Unfrozen: Unfrozen<Self::Action, F>;
  type Frozen: Frozen;
}