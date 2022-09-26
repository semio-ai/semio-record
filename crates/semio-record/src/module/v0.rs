pub mod action;
pub mod public;
pub mod frozen;
pub mod unfrozen;



use crate::{
  record::{RecordDefn, TYPE_MODULE},
  version::{ValidatedSemanticVersion, ValidationResult}, schema_version::impl_schema_version,
};

pub struct Module;

impl RecordDefn for Module {
  const TYPE: i16 = TYPE_MODULE;
  const SCHEMA_VERSION: i16 = 0;

  type Frozen = frozen::Module;
  type Unfrozen = unfrozen::Module;
  type Action = action::Action;
  type Public = public::Public;
  type Private = unfrozen::Module;
}

impl ValidatedSemanticVersion for Module {
  fn validate(&self, _: &Self) -> ValidationResult {
    ValidationResult::Valid
  }
}

impl_schema_version!(Module);