use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer}, blob::BlobDependencies, unfrozen::impl_unfrozen};

use super::action::Action;

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
  pub name: String,
}

impl_unfrozen!(Organization, Action);

impl Unfrozen<Action> for Organization {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Organization {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

impl View for Organization {
  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    None
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Organization {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}