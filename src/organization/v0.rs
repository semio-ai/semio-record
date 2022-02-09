pub mod action;
pub mod unfrozen;

use crate::{schema_version::impl_schema_version, record::{Record, TYPE_ORGANIZATION}};

struct Organization;

impl Record for Organization {
  const TYPE: i16 = TYPE_ORGANIZATION;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Organization;
  type Frozen = ();
}

impl_schema_version!(Organization);