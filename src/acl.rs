use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;



/// An Access Control List (ACL) is a list of rules that specify which agents
/// can perform which actions, if any.
#[derive(Debug, Serialize, Deserialize)]
pub struct Acl {
  /// A map of agent IDs to their permissions.
  pub permissions: HashMap<Uuid, WithPermissions>,

  /// If no custom permissions are specified, these permissions should be used instead.
  pub default: WithPermissions,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum PermissionLevel {
  None,
  Public,
  Private,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Permissions {
  pub read: PermissionLevel,
  pub write: PermissionLevel,
}

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