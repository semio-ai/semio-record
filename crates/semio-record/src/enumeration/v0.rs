pub mod action;
pub mod frozen;
pub mod unfrozen;
pub mod public;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_ENUMERATION}};

pub struct Enumeration;

impl RecordDefn for Enumeration {
  const TYPE: i16 = TYPE_ENUMERATION;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Enumeration;
  type Frozen = frozen::Enumeration;

  type Public = public::Public;
  type Private = unfrozen::Enumeration;
}

impl_schema_version!(Enumeration, "enumeration_V0");