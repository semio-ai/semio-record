use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer}, blob::BlobDependencies, unfrozen::impl_unfrozen, action::Name, migrate::Migrate};

use schemars::JsonSchema;

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct User {
  pub user_name: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password_hash: String,
  pub email_verified: bool,
  pub token_secret: String,
}

impl_unfrozen!(User, Action);

impl Unfrozen<Action> for User {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for User {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

impl View for User {
  fn name(&self) -> Option<&str> {
    Some(&self.user_name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    None
  }
}

impl Name for User {
  fn name(&self) -> &str {
    &self.user_name
  }
  
  fn set_name(&mut self, name: String) {
    self.user_name = name;
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for User {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for User {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}