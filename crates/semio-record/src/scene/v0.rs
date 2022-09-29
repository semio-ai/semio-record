pub mod action;
pub mod unfrozen;
pub mod public;

use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_SCENE}};

pub struct Scene;

impl RecordDefn for Scene {
  const TYPE: i16 = TYPE_SCENE;
  const SCHEMA_VERSION: i16 = 0;

  type Action = action::Action;
  type Unfrozen = unfrozen::Scene;
  type Frozen = ();

  type Public = public::Public;
  type Private = unfrozen::Scene;
}

impl_schema_version!(Scene);