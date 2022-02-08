use std::error::Error;

use record::Freezer;

pub mod blob;
pub mod record;
pub mod module;
pub mod patch;
pub mod ty;
pub mod structure;
pub mod enumeration;
pub mod folder;
pub mod version;
pub mod serial;

pub async fn freeze<F: Freezer + 'static>(freezer: &mut F, ty: i16, schema_version: i16, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>
where
  F: Freezer,
{
  match ty {
    record::TYPE_MODULE => module::freeze(freezer, schema_version, data).await,
    _ => Err(format!("Unsupported record type: {}", ty).into()),
  }
}

pub fn apply_raw(ty: i16, schema_version: i16, module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  match ty {
    record::TYPE_MODULE => module::apply_raw(schema_version, module, action),
    _ => Err(format!("Unsupported record type: {}", ty).into()),
  }
}

pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(ty: i16, schema_version: i16, module: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn Error>> {
  match ty {
    record::TYPE_MODULE => module::apply_raw_iter(schema_version, module, actions),
    _ => Err(format!("Unsupported record type: {}", ty).into()),
  }
}