pub mod action;
pub mod unfrozen;

use crate::{schema_version::impl_schema_version, record::{Record, TYPE_USER}};

struct User;

impl Record for User {
  const TYPE: i16 = TYPE_USER;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::User;
  type Frozen = ();
}

impl_schema_version!(User);