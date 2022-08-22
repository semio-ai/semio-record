pub mod action;
pub mod unfrozen;
pub mod public;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_ORGANIZATION}};

pub struct Organization;

impl RecordDefn for Organization {
  const TYPE: i16 = TYPE_ORGANIZATION;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Organization;
  type Frozen = ();

  type Public = public::Public;
  type Private = unfrozen::Organization;
}

impl_schema_version!(Organization);