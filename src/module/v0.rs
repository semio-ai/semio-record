pub mod action;
pub mod frozen;
pub mod unfrozen;

use std::{collections::{HashMap, HashSet}, error::Error};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  blob::BlobDependencies,
  record::{Record, Reference, TYPE_MODULE, Freezer, Freeze, Unfrozen},
  version::{ValidatedSemanticVersion, ValidationResult},
};

pub struct Module;

impl<F: Freezer> Record<F> for Module {
  const TYPE: i16 = TYPE_MODULE;
  const SCHEMA_VERSION: i16 = 0;

  type Frozen = frozen::Module;
  type Unfrozen = unfrozen::Module;
  type Action = action::Action;
}



impl ValidatedSemanticVersion for Module {
  fn validate(&self, _: &Self) -> ValidationResult {
    ValidationResult::Valid
  }
}

pub async fn freeze<F: 'static + Freezer>(freezer: &mut F, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  let unfrozen: unfrozen::Module = rmp_serde::from_slice(data)?;
  let frozen = unfrozen.freeze(freezer).await?;
  Ok(rmp_serde::to_vec(&frozen)?)
}

pub fn apply_raw(module: &[u8], action: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
  let mut module: unfrozen::Module = rmp_serde::from_slice(module)?;
  module.apply_raw(action)?;
  Ok(rmp_serde::to_vec(&module)?)
}

pub fn apply_raw_iter<B: AsRef<[u8]>, I: Iterator<Item = B>>(module: &[u8], actions: I) -> Result<Vec<u8>, Box<dyn Error>> {
  let mut module: unfrozen::Module = rmp_serde::from_slice(module)?;
  module.apply_raw_iter(actions)?;
  Ok(rmp_serde::to_vec(&module)?)
}