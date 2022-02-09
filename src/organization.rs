pub mod v0;

pub use v0 as latest;

use crate::record::impl_record;

impl_record!(
  0 => v0
);