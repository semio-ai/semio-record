pub mod action;
pub mod frozen;
pub mod unfrozen;

use std::error::Error;

use uuid::Uuid;

use crate::{
  record::{RecordDefn, TYPE_MODULE, Freezer, Freeze, View},
  version::{ValidatedSemanticVersion, ValidationResult}, schema_version::impl_schema_version,
};

pub struct Module;

impl RecordDefn for Module {
  const TYPE: i16 = TYPE_MODULE;
  const SCHEMA_VERSION: i16 = 0;

  type Frozen = frozen::Module;
  type Unfrozen = unfrozen::Module;
  type Action = action::Action;
  type Public = unfrozen::Module;
  type Private = unfrozen::Module;
}

impl ValidatedSemanticVersion for Module {
  fn validate(&self, _: &Self) -> ValidationResult {
    ValidationResult::Valid
  }
}

impl_schema_version!(Module);