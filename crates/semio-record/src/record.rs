use std::{collections::HashSet, fmt::Display};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{acl::Acl, blob::BlobDependencies, migrate::Migrate};

// The neutral versioning + freezing vocabulary lives in arora-types, which was
// built as a store-agnostic re-modeling of exactly these types (its serde
// shapes are byte-compatible with ours). Reusing it keeps a single definition
// of record references and the freeze mechanism; semio-record layers its record
// model — actions, ACL, views, projections, the i16 dispatch — on top.
// `Freezer` is our historical name for arora's `Resolver`; they are one trait.
pub use arora_types::record::freeze::{Freeze, Resolver as Freezer};
pub use arora_types::record::reference::{
  FrozenReference, UnfrozenReference, Version, VersionReq,
};

pub const TYPE_USER: i16 = 0x0000;
pub const TYPE_ORGANIZATION: i16 = 0x0001;

pub const TYPE_FOLDER: i16 = 0x0010;

pub const TYPE_MODULE: i16 = 0x0020;
pub const TYPE_STRUCTURE: i16 = 0x0021;
pub const TYPE_ENUMERATION: i16 = 0x0022;

pub const TYPE_ANIMATION: i16 = 0x0030;

pub const TYPE_SCENE: i16 = 0x0100;
pub const TYPE_PLATFORM: i16 = 0x0200;
pub const TYPE_WORKSPACE: i16 = 0x0300;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Path {
  pub components: Vec<String>,
}

pub trait Apply<T> {
  type Error: Display;

  fn mutates_name(_action: &T) -> bool {
    false
  }
  fn mutates_parent(_action: &T) -> bool {
    false
  }

  fn apply(&mut self, action: &T) -> Result<(), Self::Error>;
}

pub trait View {
  fn name<'a>(&'a self) -> Option<&'a str> {
    None
  }
  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    None
  }
  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    None
  }
}

pub trait Unfrozen<T>: View + BlobDependencies + Apply<T> + Migrate + Serialize + DeserializeOwned {
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

impl View for () {}

pub trait RecordDefn {
  const TYPE: i16;
  const SCHEMA_VERSION: i16;

  type Action;
  type Unfrozen: Unfrozen<Self::Action>;
  type Frozen: Frozen = ();
  #[cfg(feature = "schemars")]
  type Public: From<Self::Unfrozen> + schemars::JsonSchema;
  #[cfg(not(feature = "schemars"))]
  type Public: From<Self::Unfrozen>;
  
  #[cfg(feature = "schemars")]
  type Private: From<Self::Unfrozen> + schemars::JsonSchema;
  #[cfg(not(feature = "schemars"))]
  type Private: From<Self::Unfrozen>;
}

pub enum RecordContent<R: RecordDefn> {
  Action(R::Action),
  Unfrozen(R::Unfrozen),
  Frozen(R::Frozen),
}

macro_rules! impl_record {
  ($($version:expr => $module: ident),+) => {
    pub async fn freeze<F: 'static + crate::record::Freezer>(freezer: &F, schema_version: i16, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

    pub fn apply_public_raw(schema_version: i16, module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::apply_public_raw(module, action),
        )+
        _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
      }
    }

    pub fn apply_private_raw(schema_version: i16, module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::apply_private_raw(module, action),
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

    pub async fn apply_raw_stream<S: futures::Stream<Item = Vec<u8>> + Unpin>(schema_version: i16, module: &[u8], actions: S) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => $module::apply_raw_stream(module, actions).await,
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

    pub fn acl(schema_version: i16, module: &[u8]) -> Option<crate::acl::Acl> {
      match schema_version {
        $(
          $version => $module::acl(module),
        )+
        _ => None,
      }
    }

    #[cfg(feature = "schemars")]
    pub fn schema(schema_version: i16) -> Result<crate::schema_version::Schema, Box<dyn std::error::Error>> {
      match schema_version {
        $(
          $version => Ok($module::schema()),
        )+
        _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
      }
    }
  };
}

pub(crate) use impl_record;
