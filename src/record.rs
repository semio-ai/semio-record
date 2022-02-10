


use std::{collections::{HashSet, HashMap}, error::Error};


use async_trait::async_trait;
use chrono::Duration;
use juniper::{GraphQLObject, GraphQLValue, ExecutionResult, marker::IsOutputType, ScalarValue, GraphQLType, DefaultScalarValue};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use derive_more::From;

use crate::blob::BlobDependencies;

pub const TYPE_USER: i16 = 0x0000;
pub const TYPE_ORGANIZATION: i16 = 0x0001;

pub const TYPE_FOLDER: i16 = 0x0010;

pub const TYPE_MODULE: i16 = 0x0020;
pub const TYPE_STRUCTURE: i16 = 0x0021;
pub const TYPE_ENUMERATION: i16 = 0x0022;

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Version(pub semver::Version);

#[juniper::graphql_scalar]
impl<S> GraphQLScalar for Version
where
  S: juniper::ScalarValue
{
  fn resolve(&self) -> juniper::Value {
    juniper::Value::scalar(self.0.to_string())
  }

  // NOTE: The error type should implement `IntoFieldError<S>`.
  fn from_input_value(value: &juniper::InputValue) -> Option<Version> {
    match value.as_string_value() {
      Some(s) => Some(Version(s.parse().ok()?)),
      None => None
    }
  }
  
  fn from_str<'a>(value: juniper::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
    <String as juniper::ParseScalarValue<S>>::from_str(value)
  }
}

#[derive(Debug, From, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionReq(pub Option<semver::VersionReq>);

#[juniper::graphql_scalar]
impl<S> GraphQLScalar for VersionReq
where
  S: juniper::ScalarValue
{
  fn resolve(&self) -> juniper::Value {
    match &self.0 {
      None => juniper::Value::null(),
      Some(s) => juniper::Value::scalar(s.to_string())
    }
  }

  fn from_input_value(value: &juniper::InputValue) -> Option<VersionReq> {
    if value.is_null() {
      return Some(VersionReq(None));
    }

    match value.as_string_value() {
      Some(s) => Some(VersionReq(Some(s.parse().ok()?))),
      None => None
    }
  }
  
  fn from_str<'a>(value: juniper::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
    <String as juniper::ParseScalarValue<S>>::from_str(value)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Path {
  pub components: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, PartialOrd, Ord, GraphQLObject)]
pub struct FrozenReference {
  pub id: Uuid,
  pub version: Version,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, GraphQLObject)]
pub struct UnfrozenReference {
  pub id: Uuid,
  pub version_req: VersionReq,
}

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

pub trait RecordDefn {
  const TYPE: i16;
  const SCHEMA_VERSION: i16;

  type Action;
  type Unfrozen: Unfrozen<Self::Action>;
  type Frozen: Frozen;
  type Public: From<Self::Unfrozen> + GraphQLValue + IsOutputType<DefaultScalarValue>;
  type Private: From<Self::Unfrozen> + GraphQLValue + IsOutputType<DefaultScalarValue>;
}

pub enum RecordContent<R: RecordDefn> {
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
  };
}

pub(crate) use impl_record;