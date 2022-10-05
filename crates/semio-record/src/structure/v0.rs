
pub mod action;
pub mod public;
pub mod frozen;
pub mod unfrozen;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_STRUCTURE}};

pub struct Structure;

impl RecordDefn for Structure {
  const TYPE: i16 = TYPE_STRUCTURE;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Structure;
  type Frozen = frozen::Structure;

  type Public = public::Public;
  type Private = unfrozen::Structure;
}

impl_schema_version!(Structure, "structure_V0");