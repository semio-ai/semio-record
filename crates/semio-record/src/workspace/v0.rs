// pub mod action;
// pub mod unfrozen;
// pub mod public;

// use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_WORKSPACE}};

// pub struct Workspace;

// impl RecordDefn for Workspace {
//   const TYPE: i16 = TYPE_WORKSPACE;
//   const SCHEMA_VERSION: i16 = 0;

//   type Action = action::Action;
//   type Unfrozen = unfrozen::Animation;

//   type Public = public::Public;
//   type Private = unfrozen::Animation;
// }

// impl_schema_version!(Animation);