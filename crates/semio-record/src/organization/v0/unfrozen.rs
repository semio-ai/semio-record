use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer}, blob::BlobDependencies, unfrozen::impl_unfrozen, acl::{Acl, action::with_acl}, action::name, migrate::Migrate};

use super::action::Action;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone, JsonSchema)]
pub struct Organization {
  pub name: String,
  pub acl: Acl,
}

impl_unfrozen!(Organization, Action);

impl Unfrozen<Action> for Organization {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Organization {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

with_acl!(Organization);

impl View for Organization {
  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    None
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

name!(Organization);

#[async_trait]
impl<F: Freezer> Freeze<F> for Organization {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for Organization {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}