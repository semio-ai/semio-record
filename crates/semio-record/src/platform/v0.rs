// pub mod action;
// pub mod unfrozen;
// pub mod public;

// use crate::{schema_version::impl_schema_version, record::{RecordDefn, TYPE_PLATFORM}};

// pub struct Platform;

// impl RecordDefn for Platform {
//   const TYPE: i16 = TYPE_PLATFORM;
//   const SCHEMA_VERSION: i16 = 0;

//   type Action = action::Action;
//   type Unfrozen = unfrozen::Animation;

//   type Public = public::Public;
//   type Private = unfrozen::Animation;
// }

// impl_schema_version!(Platform);