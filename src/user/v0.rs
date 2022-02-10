pub mod action;
pub mod unfrozen;
pub mod public;
pub mod private;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_USER}};

struct User;

impl RecordDefn for User {
  const TYPE: i16 = TYPE_USER;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::User;
  type Frozen = ();

  type Public = public::Public;
  type Private = private::Private;
}

impl_schema_version!(User);