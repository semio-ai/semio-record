pub mod action;
pub mod unfrozen;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_ORGANIZATION}};

struct Organization;

impl RecordDefn for Organization {
  const TYPE: i16 = TYPE_ORGANIZATION;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Organization;
  type Frozen = ();

  type Public = unfrozen::Organization;
  type Private = unfrozen::Organization;
}

impl_schema_version!(Organization);