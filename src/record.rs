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
  type Error;

  fn apply(&mut self, action: &T) -> Result<(), Self::Error>;
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

pub trait Unfrozen<T>: View + BlobDependencies + Apply<T> + Serialize + DeserializeOwned {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}
pub trait Frozen: View + BlobDependencies + Serialize + DeserializeOwned {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a FrozenReference>) {}
}

impl Frozen for () {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a FrozenReference>) {}
}

impl BlobDependencies for () {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a Uuid>) {}
}

impl View for () {
  fn name<'a>(&'a self) -> Option<&'a str> { None }
  fn parent<'a>(&'a self) -> Option<&'a Uuid> { None }
}

pub trait Record {
  const TYPE: i16;
  const SCHEMA_VERSION: i16;

  type Action;
  type Unfrozen: Unfrozen<Self::Action>;
  type Frozen: Frozen;
}

pub enum RecordContent<R: Record> {
  Action(R::Action),
  Unfrozen(R::Unfrozen),
  Frozen(R::Frozen),
}

macro_rules! impl_record {
  ($($version:expr => $module: ident),+) => {
    pub async fn freeze<F: 'static + crate::record::Freezer>(freezer: &mut F, schema_version: i16, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::freeze(freezer, data).await,
        )+
        _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
      }
    }
    pub fn apply_raw(schema_version: i16, module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::apply_raw(module, action),
        )+
        _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
      }
    }
    
    pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(schema_version: i16, module: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::apply_raw_iter(module, actions),
        )+
        _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
      }
    }
    
    pub fn name(schema_version: i16, module: &[u8]) -> Option<String> {
      match schema_version {
        $(
          $version => $module::name(module),
        )+
        _ => None,
      }
    }
    
    pub fn parent(schema_version: i16, module: &[u8]) -> Option<uuid::Uuid> {
      match schema_version {
        $(
          $version => $module::parent(module),
        )+
        _ => None,
      }
    }
  };
}

pub(crate) use impl_record;