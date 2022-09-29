use std::{collections::HashSet, fmt::Display};

use async_trait::async_trait;
use derive_more::From;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{acl::Acl, blob::BlobDependencies, migrate::Migrate};

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

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Version(#[cfg_attr(feature = "schemars", schemars(with = "String"))] pub semver::Version);

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Version {
  pub fn parse(s: &str) -> Result<Self, semver::Error> {
    Ok(Version(s.parse()?))
  }
}

#[derive(Debug, From, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct VersionReq(#[cfg_attr(feature = "schemars", schemars(with = "Option::<String>"))] pub Option<semver::VersionReq>);

impl VersionReq {
  pub fn parse(s: &str) -> Result<Self, semver::Error> {
    Ok(Self(Some(s.parse()?)))
  }
}

impl std::fmt::Display for VersionReq {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match &self.0 {
      Some(req) => write!(f, "{}", req),
      None => write!(f, "*"),
    }
  }
}

impl Into<semver::VersionReq> for VersionReq {
  fn into(self) -> semver::VersionReq {
    self.0.unwrap_or_default()
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Path {
  pub components: Vec<String>,
}

#[derive(
  Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, PartialOrd, Ord
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Reference", rename_all = "camelCase")]
pub struct FrozenReference {
  pub id: Uuid,
  pub version: Version,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unfrozen_Reference", rename_all = "camelCase")]
pub struct UnfrozenReference {
  pub id: Uuid,
  pub version_req: VersionReq,
}

impl ToString for UnfrozenReference {
  fn to_string(&self) -> String {
    if let Some(version_req) = &self.version_req.0 {
      format!("{}@{}", self.id, version_req)
    } else {
      self.id.to_string()
    }
  }
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
  type Public: From<Self::Unfrozen>;
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
