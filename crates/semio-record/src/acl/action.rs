use serde::{Serialize, Deserialize};
use uuid::Uuid;

use derive_more::{Display, Error, From};

macro_rules! with_acl {
  ($t: tt) => {
    impl crate::acl::action::WithAcl for $t {
      fn acl<'a>(&'a self) -> &'a crate::acl::Acl {
        &self.acl
      }

      fn acl_mut<'a>(&'a mut self) -> &'a mut crate::acl::Acl {
        &mut self.acl
      }
    }
  }
}

pub(crate) use with_acl;

pub trait WithAcl {
  fn acl<'a>(&'a self) -> &'a Acl;
  fn acl_mut<'a>(&'a mut self) -> &'a mut Acl;
}

use crate::record::Apply;

use super::{WithPermissions, Acl};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_SetDefault")]
pub struct SetDefault {
  pub with_permissions: WithPermissions,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_SetDefaultError", tag = "type", rename_all = "camelCase")]
pub enum SetDefaultError {
  #[display(fmt = "Invalid permissions")]
  InvalidPermissions,
}

impl<T: WithAcl> Apply<SetDefault> for T {
  type Error = SetDefaultError;

  fn apply(&mut self, action: &SetDefault) -> Result<(), Self::Error> {
    let acl = self.acl_mut();
    acl.default = action.with_permissions.clone();

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_AddPermissions")]
pub struct AddPermissions {
  pub agent: Uuid,
  pub with_permissions: WithPermissions,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_AddPermissionsError", tag = "type", rename_all = "camelCase")]
pub enum AddPermissionsError {
  #[display(fmt = "Invalid permissions")]
  InvalidPermissions,
  #[display(fmt = "Agent already exists")]
  AgentAlreadyExists,
}

impl<T: WithAcl> Apply<AddPermissions> for T {
  type Error = AddPermissionsError;

  fn apply(&mut self, action: &AddPermissions) -> Result<(), Self::Error> {
    let acl = self.acl_mut();
    let permissions = acl.permissions_mut();
    if let Some(_) = permissions.get(&action.agent) {
      return Err(AddPermissionsError::AgentAlreadyExists);
    }

    permissions.insert(action.agent.clone(), action.with_permissions.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_RemovePermissions")]
pub struct RemovePermissions {
  pub agent: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_RemovePermissionsError", tag = "type", rename_all = "camelCase")]
pub enum RemovePermissionsError {
  #[display(fmt = "Agent does not exists")]
  AgentDoesNotExists,
}

impl<T: WithAcl> Apply<RemovePermissions> for T {
  type Error = RemovePermissionsError;

  fn apply(&mut self, action: &RemovePermissions) -> Result<(), Self::Error> {
    let acl = self.acl_mut();
    let permissions = acl.permissions_mut();
    
    if let None = permissions.remove(&action.agent) {
      return Err(RemovePermissionsError::AgentDoesNotExists);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_SetPermissions")]
pub struct SetPermissions {
  pub agent: Uuid,
  pub with_permissions: WithPermissions,
}

#[derive(Debug, Serialize, Deserialize, Display, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action_SetPermissionsError", tag = "type", rename_all = "camelCase")]
pub enum SetPermissionsError {
  #[display(fmt = "Invalid permissions")]
  InvalidPermissions,
  #[display(fmt = "Agent does not exists")]
  AgentDoesNotExists,
}

impl<T: WithAcl> Apply<SetPermissions> for T {
  type Error = SetPermissionsError;

  fn apply(&mut self, action: &SetPermissions) -> Result<(), Self::Error> {
    let acl = self.acl_mut();
    let permissions = acl.permissions_mut();
    
    if let Some(with_permissions) = permissions.get_mut(&action.agent) {
      *with_permissions = action.with_permissions.clone();
    } else {
      return Err(SetPermissionsError::AgentDoesNotExists);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_Action", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Action {
  SetDefault(SetDefault),
  AddPermissions(AddPermissions),
  RemovePermissions(RemovePermissions),
  SetPermissions(SetPermissions),
}


#[derive(Debug, Serialize, Deserialize, Display, Error, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "Acl_ActionError", tag = "type", rename_all = "camelCase", content = "value")]
pub enum ActionError {
  SetDefault(SetDefaultError),
  AddPermissions(AddPermissionsError),
  RemovePermissions(RemovePermissionsError),
  SetPermissions(SetPermissionsError),
}

impl<T: WithAcl> Apply<Action> for T {
  type Error = ActionError;

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetDefault(action) => self.apply(action)?,
      Action::AddPermissions(action) => self.apply(action)?,
      Action::RemovePermissions(action) => self.apply(action)?,
      Action::SetPermissions(action) => self.apply(action)?,
    }

    Ok(())
  }
}