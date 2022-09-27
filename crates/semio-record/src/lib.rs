#![feature(associated_type_defaults)]

#[macro_use] extern crate juniper;

pub mod blob;
pub mod expr;
pub mod record;
pub mod module;
pub mod workspace;
pub mod ty;
pub mod structure;
pub mod enumeration;
pub mod organization;
pub mod folder;
pub mod version;
pub mod serial;
pub mod scene;
pub mod user;
pub mod schema_version;
pub mod unfrozen;
pub mod acl;
pub mod action;
pub mod animation;
pub mod color;
pub mod math;
pub mod migrate;
pub mod reference;
pub mod i18n;
pub mod unit;
pub mod unit_math;

macro_rules! impl_lib {
  ($($ty:tt => $module:tt),+) => {
    pub async fn freeze<F>(freezer: &F, ty: i16, schema_version: i16, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
      F: 'static + crate::record::Freezer,
    {
      match ty {
        $(
          crate::record::$ty => $module::freeze(freezer, schema_version, data).await,
        )+
        _ => Err(format!("Unsupported record type: {}", ty).into()),
      }
    }

    pub fn apply_raw(ty: i16, schema_version: i16, data: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match ty {
        $(
          crate::record::$ty => $module::apply_raw(schema_version, data, action),
        )+
        _ => Err(format!("Unsupported record type: {}", ty).into()),
      }
    }

    pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(ty: i16, schema_version: i16, data: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match ty {
        $(
          crate::record::$ty => $module::apply_raw_iter(schema_version, data, actions),
        )+
        _ => Err(format!("Unsupported record type: {}", ty).into()),
      }
    }

    pub async fn apply_raw_stream<S: futures::Stream<Item = Vec<u8>> + Unpin>(ty: i16, schema_version: i16, data: &[u8], actions: S) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
      match ty {
        $(
          crate::record::$ty => $module::apply_raw_stream(schema_version, data, actions).await,
        )+
        _ => Err(format!("Unsupported record type: {}", ty).into()),
      }
    }

    pub fn name(ty: i16, schema_version: i16, data: &[u8]) -> Option<String> {
      match ty {
        $(
          crate::record::$ty => $module::name(schema_version, data),
        )+
        _ => None,
      }
    }

    pub fn parent(ty: i16, schema_version: i16, data: &[u8]) -> Option<uuid::Uuid> {
      match ty {
        $(
          crate::record::$ty => $module::parent(schema_version, data),
        )+
        _ => None,
      }
    }

    pub fn acl(ty: i16, schema_version: i16, data: &[u8]) -> Option<acl::Acl> {
      match ty {
        $(
          crate::record::$ty => $module::acl(schema_version, data),
        )+
        _ => None,
      }
    }

    pub fn schema(ty: i16, schema_version: i16) -> Result<crate::schema_version::Schema, Box<dyn std::error::Error>> {
      match ty {
        $(
          crate::record::$ty => $module::schema(schema_version),
        )+
        _ => Err(format!("Unsupported record type: {}", ty).into()),
      }
    }
  }
}

impl_lib!(
  TYPE_USER => user,
  TYPE_ORGANIZATION => organization,
  TYPE_MODULE => module,
  TYPE_STRUCTURE => structure,
  TYPE_ENUMERATION => enumeration,
  TYPE_FOLDER => folder,
  TYPE_ANIMATION => animation
);

#[derive(Debug, Serialize, Deserialize, GraphQLUnion)]
pub enum FrozenRecord {
  Module(module::latest::frozen::Module),

}


pub use serde_json::to_vec as serialize;
pub use serde_json::from_slice as deserialize;
use serde::Deserialize;
use serde::Serialize;