pub mod v0;

use std::error::Error;

pub use v0 as latest;

use crate::record::Freezer;

pub async fn freeze<F: 'static + Freezer>(freezer: &mut F, schema_version: i16, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  match schema_version {
    0 => v0::freeze(freezer, data).await,
    _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
  }
}
pub fn apply_raw(schema_version: i16, module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  match schema_version {
    0 => v0::apply_raw(module, action),
    _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
  }
}

pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(schema_version: i16, module: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn Error>> {
  match schema_version {
    0 => v0::apply_raw_iter(module, actions),
    _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
  }
}