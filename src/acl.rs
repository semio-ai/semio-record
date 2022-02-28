use std::collections::HashMap;

use juniper::FieldError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;
use derive_more::Display;



/// An Access Control List (ACL) is a list of rules that specify which agents
/// can perform which actions, if any.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Acl {
  /// A map of agent IDs to their permissions.
  pub permissions: HashMap<Uuid, WithPermissions>,

  /// If no custom permissions are specified, these permissions should be used instead.
  pub default: WithPermissions,
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

  pub async fn resolve<P: PermissionResolver>(&self, resolver: &P, parent: Option<&Uuid>, agent: &Uuid) -> Result<Permissions, FieldError> {
    Ok(self
      .with_permissions(agent)
      .resolve(resolver, parent, agent)
      .await?
    )
  }
}

#[derive(Debug, Display, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, GraphQLEnum)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject)]
pub struct None {
  pub _dummy: i32,
}

const NONE: None = None { _dummy: 0 };

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject)]
pub struct Inherit {
  /// Inherit permissions from the following record with a ACL.
  /// If `Option::None`, the record's logical parent will be used.
  /// If the specified record does not have an ACL, that
  /// record's logical parent will be used recursively.
  /// If the recursive search doesn't find any ACL, this is equivalent to `acl::None`.
  pub from: Option<Uuid>
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject)]
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
  pub fn validate(&self, required: &Permissions) -> Result<(), FieldError> {
    if self.read < required.read {
      return Err(FieldError::new("Read permission for agent is too low", graphql_value!({ "error": "UNAUTHORIZED" })));
    }

    if self.write < required.write {
      return Err(FieldError::new("Write permission for agent is too low", graphql_value!({ "error": "UNAUTHORIZED" })));
    }

    Ok(())
  }
}

const NO_PERMISSIONS: Permissions = Permissions {
  read: PermissionLevel::None,
  write: PermissionLevel::None,
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLUnion)]
#[serde(tag = "type", rename_all = "lowercase")]
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
  pub async fn resolve<P: PermissionResolver>(&self, resolver: &P, parent: Option<&Uuid>, agent: &Uuid) -> Result<Permissions, FieldError> {
    match self {
      WithPermissions::None(_) => Ok(NO_PERMISSIONS),
      WithPermissions::Inherit(inherit) => Ok(resolver.inherited_permissions(parent, &inherit, agent).await?),
      WithPermissions::Custom(permissions) => Ok(permissions.clone()),
    }
  }
}

#[async_trait]
pub trait PermissionResolver {
  async fn inherited_permissions(&self, parent: Option<&Uuid>, inherit: &Inherit, agent: &Uuid) -> Result<Permissions, FieldError>;
}

pub struct DummyPermissionResolver;

#[async_trait]
impl PermissionResolver for DummyPermissionResolver {
  async fn inherited_permissions(&self, _parent: Option<&Uuid>, _inherit: &Inherit, _agent: &Uuid) -> Result<Permissions, FieldError> {
    Err(FieldError::new("Inheriting permissions is not possible", graphql_value!({ "error": "UNAUTHORIZED" })))
  }
}

