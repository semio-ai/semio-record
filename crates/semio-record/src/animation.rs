pub mod v0;
pub mod v1;

pub use v1 as latest;

use crate::record::impl_record;

impl_record!(
  0 => v0,
  1 => v1
);