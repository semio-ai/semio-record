use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;



/// An Access Control List (ACL) is a list of rules that specify which agents
/// can perform which actions, if any.
#[derive(Debug, Serialize, Deserialize)]
pub struct Acl {
  /// A map of agent IDs to their permissions.
  pub permissions: HashMap<Uuid, WithPermissions>,

  /// If no custom permissions are specified, these permissions should be used instead.
  pub default: WithPermissions,
}

impl Acl {
  pub fn with_permissions(&self, agent: &Uuid) -> &WithPermissions {
    self.permissions.get(agent).unwrap_or(&self.default)
  }

  pub async fn resolve<P: PermissionResolver>(&self, resolver: &P, id: &Uuid, agent: &Uuid) -> anyhow::Result<Permissions> {
    Ok(self
      .with_permissions(agent)
      .resolve(resolver, id, agent)
      .await?
    )
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum PermissionLevel {
  None,
  Public,
  Private,
}

impl PermissionLevel {
  pub fn valid(&self) -> bool {
    match self {
      PermissionLevel::None => false,
      PermissionLevel::Public => true,
      PermissionLevel::Private => true,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Permissions {
  pub read: PermissionLevel,
  pub write: PermissionLevel,
}

const NO_PERMISSIONS: Permissions = Permissions {
  read: PermissionLevel::None,
  write: PermissionLevel::None,
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum WithPermissions {
  /// No permissions
  None,

  /// Inherit permissions from the parent's ACL. If the parent has no ACL,
  /// then the parent's parent will be checked, and so on. If no ACL is
  /// found, then this is equivalent to `None`.
  Inherit,

  /// Custom permissions
  Custom(Permissions),
}

impl WithPermissions {
  pub async fn resolve<P: PermissionResolver>(&self, resolver: &P, id: &Uuid, agent: &Uuid) -> anyhow::Result<Permissions> {
    match self {
      WithPermissions::None => Ok(NO_PERMISSIONS),
      WithPermissions::Inherit => Ok(resolver.inherited_permissions(id, agent).await?),
      WithPermissions::Custom(permissions) => Ok(permissions.clone()),
    }
  }
}

#[async_trait]
pub trait PermissionResolver {
  async fn inherited_permissions(&self, id: &Uuid, agent: &Uuid) -> anyhow::Result<Permissions>;
}

pub struct DummyPermissionResolver;

#[async_trait]
impl PermissionResolver for DummyPermissionResolver {
  async fn inherited_permissions(&self, _id: &Uuid, _agent: &Uuid) -> anyhow::Result<Permissions> {
    anyhow::bail!("DummyPermissionResolver does not support inherited permissions")
  }
}

