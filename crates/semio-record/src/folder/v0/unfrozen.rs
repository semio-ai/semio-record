use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use schemars::JsonSchema;

use crate::{
  record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer},
  blob::BlobDependencies,
  unfrozen::impl_unfrozen,
  acl::{Acl, action::with_acl},
  action::{name, parent},
  migrate::Migrate
};

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone, JsonSchema)]
#[serde(rename = "folder_v0_Private")]
pub struct Folder {
  pub name: String,
  pub parent: Uuid,
  pub acl: Acl,
}

impl_unfrozen!(Folder, Action);
name!(Folder);
parent!(Folder);
with_acl!(Folder);

impl Unfrozen<Action> for Folder {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Folder {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

impl View for Folder {
  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    Some(&self.parent)
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Folder {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for Folder {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}