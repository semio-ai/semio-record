pub mod action;
pub mod unfrozen;
pub mod public;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_FOLDER}};

pub struct Folder;

impl RecordDefn for Folder {
  const TYPE: i16 = TYPE_FOLDER;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Folder;
  type Frozen = ();

  type Public = public::Public;
  type Private = unfrozen::Folder;
}

impl_schema_version!(Folder, "folder_V0");