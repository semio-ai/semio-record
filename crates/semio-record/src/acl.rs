pub mod action;

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdWithPermissions {
  pub id: Uuid,
  pub with_permissions: WithPermissions,
}

/// An Access Control List (ACL) is a list of rules that specify which agents
/// can perform which actions, if any.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl")]
pub struct Acl {
  /// A map of agent IDs to their permissions.
  pub permissions: HashMap<Uuid, WithPermissions>,

  /// If no custom permissions are specified, these permissions should be used instead.
  pub default: WithPermissions,
}

impl Acl {
  pub fn new(owner: Uuid, with_permissions: WithPermissions) -> Self {
    let mut permissions = HashMap::new();
    permissions.insert(owner, with_permissions);
    Self {
      permissions,
      default: Default::default(),
    }
  }

  pub fn default(&self) -> &WithPermissions {
    &self.default
  }

  pub fn default_mut(&mut self) -> &mut WithPermissions {
    &mut self.default
  }

  pub fn permissions(&self) -> &HashMap<Uuid, WithPermissions> {
    &self.permissions
  }

  pub fn permissions_mut(&mut self) -> &mut HashMap<Uuid, WithPermissions> {
    &mut self.permissions
  }
}

impl Default for Acl {
  fn default() -> Self {
    Self {
      permissions: HashMap::new(),
      default: Default::default(),
    }
  }
}

impl Acl {
  pub fn with_permissions(&self, agent: &Uuid) -> &WithPermissions {
    self.permissions.get(agent).unwrap_or(&self.default)
  }

  pub async fn resolve<P: PermissionResolver>(
    &self,
    resolver: &P,
    parent: Option<&Uuid>,
    agent: &Uuid,
  ) -> Result<Permissions, Box<dyn std::error::Error>> {
    Ok(
      self
        .with_permissions(agent)
        .resolve(resolver, parent, agent)
        .await?,
    )
  }
}

#[derive(
  Debug, Display, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_PermissionLevel", rename_all = "camelCase")]
pub enum PermissionLevel {
  None,
  Public,
  Private,
}

impl Default for PermissionLevel {
  fn default() -> Self {
    PermissionLevel::None
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_WithPermissions_None")]
pub struct None {
  pub _dummy: i32,
}

const NONE: None = None { _dummy: 0 };

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_WithPermissions_Inherit")]
pub struct Inherit {
  /// Inherit permissions from the following record with a ACL.
  /// If `Option::None`, the record's logical parent will be used.
  /// If the specified record does not have an ACL, that
  /// record's logical parent will be used recursively.
  /// If the recursive search doesn't find any ACL, this is equivalent to `acl::None`.
  pub from: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Permissions")]
pub struct Permissions {
  pub read: PermissionLevel,
  pub write: PermissionLevel,
}

impl Default for Permissions {
  fn default() -> Self {
    Permissions {
      read: Default::default(),
      write: Default::default(),
    }
  }
}

impl Permissions {
  pub fn validate(&self, required: &Permissions) -> Result<(), Box<dyn std::error::Error>> {
    if self.read < required.read {
      return Err(anyhow::anyhow!(
        "Read permission for agent is too low",
      ).into());
    }

    if self.write < required.write {
      return Err(anyhow::anyhow!(
        "Write permission for agent is too low",
      ).into());
    }

    Ok(())
  }
}

pub const NO_PERMISSIONS: Permissions = Permissions {
  read: PermissionLevel::None,
  write: PermissionLevel::None,
};

pub const PRIVATE_WRITE: Permissions = Permissions {
  read: PermissionLevel::None,
  write: PermissionLevel::Private,
};

pub const PRIVATE_READ: Permissions = Permissions {
  read: PermissionLevel::Private,
  write: PermissionLevel::None,
};

pub const PUBLIC_READ: Permissions = Permissions {
  read: PermissionLevel::Public,
  write: PermissionLevel::None,
};

pub const PUBLIC_WRITE: Permissions = Permissions {
  read: PermissionLevel::None,
  write: PermissionLevel::Public,
};

pub const PUBLIC_READ_WRITE: Permissions = Permissions {
  read: PermissionLevel::Public,
  write: PermissionLevel::Public,
};

pub const PRIVATE_READ_WRITE: Permissions = Permissions {
  read: PermissionLevel::Private,
  write: PermissionLevel::Private,
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_WithPermissions", tag = "type", rename_all = "camelCase", content = "value")]
pub enum WithPermissions {
  /// No permissions
  None(None),

  /// Inherit permissions from the another record's ACL. If the parent has no ACL,
  /// then the parent's parent will be checked, and so on. If no ACL is
  /// found, then this is equivalent to `None`.
  Inherit(Inherit),

  /// Custom permissions
  Custom(Permissions),
}


impl Default for WithPermissions {
  fn default() -> Self {
    WithPermissions::None(NONE)
  }
}

impl WithPermissions {
  pub async fn resolve<P: PermissionResolver>(
    &self,
    resolver: &P,
    parent: Option<&Uuid>,
    agent: &Uuid,
  ) -> Result<Permissions, Box<dyn std::error::Error>> {
    match self {
      WithPermissions::None(_) => Ok(NO_PERMISSIONS),
      WithPermissions::Inherit(inherit) => Ok(
        resolver
          .inherited_permissions(parent, &inherit, agent)
          .await?,
      ),
      WithPermissions::Custom(permissions) => Ok(permissions.clone()),
    }
  }
}

#[async_trait]
pub trait PermissionResolver {
  async fn inherited_permissions(
    &self,
    parent: Option<&Uuid>,
    inherit: &Inherit,
    agent: &Uuid,
  ) -> Result<Permissions, Box<dyn std::error::Error>>;
}

pub struct DummyPermissionResolver;

#[async_trait]
impl PermissionResolver for DummyPermissionResolver {
  async fn inherited_permissions(
    &self,
    _parent: Option<&Uuid>,
    _inherit: &Inherit,
    _agent: &Uuid,
  ) -> Result<Permissions, Box<dyn std::error::Error>> {
    Err(anyhow::anyhow!(
      "Inheriting permissions is not possible",
    ).into())
  }
}
