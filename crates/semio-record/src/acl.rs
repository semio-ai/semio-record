pub mod action;

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::Display;
use juniper::{marker::IsInputType, FieldError, FromInputValue, InputValue, ScalarValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct IdWithPermissions {
  pub id: Uuid,
  pub with_permissions: WithPermissions,
}

/// An Access Control List (ACL) is a list of rules that specify which agents
/// can perform which actions, if any.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
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

#[graphql_object]
impl Acl {
  pub fn default(&self) -> &WithPermissions {
    &self.default
  }

  pub fn permissions(&self) -> Vec<IdWithPermissions> {
    self
      .permissions
      .iter()
      .map(|(id, with_permissions)| IdWithPermissions {
        id: id.clone(),
        with_permissions: with_permissions.clone(),
      })
      .collect()
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
  ) -> Result<Permissions, FieldError> {
    Ok(
      self
        .with_permissions(agent)
        .resolve(resolver, parent, agent)
        .await?,
    )
  }
}

#[derive(
  Debug, Display, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord, GraphQLEnum, JsonSchema
)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject, JsonSchema)]
#[serde(rename = "Acl_WithPermissions_None")]
pub struct None {
  pub _dummy: i32,
}

const NONE: None = None { _dummy: 0 };

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject, JsonSchema)]
#[serde(rename = "Acl_WithPermissions_Inherit")]
pub struct Inherit {
  /// Inherit permissions from the following record with a ACL.
  /// If `Option::None`, the record's logical parent will be used.
  /// If the specified record does not have an ACL, that
  /// record's logical parent will be used recursively.
  /// If the recursive search doesn't find any ACL, this is equivalent to `acl::None`.
  pub from: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLObject, JsonSchema)]
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
  pub fn validate(&self, required: &Permissions) -> Result<(), FieldError> {
    if self.read < required.read {
      return Err(FieldError::new(
        "Read permission for agent is too low",
        graphql_value!({ "error": "UNAUTHORIZED" }),
      ));
    }

    if self.write < required.write {
      return Err(FieldError::new(
        "Write permission for agent is too low",
        graphql_value!({ "error": "UNAUTHORIZED" }),
      ));
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, GraphQLUnion, JsonSchema)]
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

impl<S: ScalarValue> FromInputValue<S> for WithPermissions {
  fn from_input_value(v: &InputValue<S>) -> Option<Self> {
    let s = v.as_string_value()?;
    let mut iter = s.split(':');
    match iter.next()? {
      "none" => Some(WithPermissions::None(None { _dummy: 0 })),
      "inherit" => Some(WithPermissions::Inherit(Inherit {
        from: iter.next().map(|s| Uuid::parse_str(s).unwrap()),
      })),
      "custom" => Some(WithPermissions::Custom(Permissions {
        read: PermissionLevel::from_input_value(&InputValue::<S>::Enum(iter.next()?.to_string()))?,
        write: PermissionLevel::from_input_value(&InputValue::<S>::Enum(iter.next()?.to_string()))?,
      })),
      _ => None,
    }
  }
}

impl<S: ScalarValue> IsInputType<S> for WithPermissions {}

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
  ) -> Result<Permissions, FieldError> {
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
  ) -> Result<Permissions, FieldError>;
}

pub struct DummyPermissionResolver;

#[async_trait]
impl PermissionResolver for DummyPermissionResolver {
  async fn inherited_permissions(
    &self,
    _parent: Option<&Uuid>,
    _inherit: &Inherit,
    _agent: &Uuid,
  ) -> Result<Permissions, FieldError> {
    Err(FieldError::new(
      "Inheriting permissions is not possible",
      graphql_value!({ "error": "UNAUTHORIZED" }),
    ))
  }
}
