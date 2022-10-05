use serde::{Serialize, Deserialize};

use crate::acl::action::{Action as AclAction, ActionError as AclActionError};

use crate::action::{SetNameError, SetName, SetParent, SetParentError};
use crate::record::Apply;

use super::unfrozen::Workspace;

use derive_more::{Display, From, Error};

#[derive(Debug, Clone, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Action", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Action {
  SetParent(SetParent),
  SetName(SetName),
  Acl(AclAction),
}

#[derive(Display, Clone, Debug, Error, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_ActionError", tag = "type", rename_all = "camelCase")]
pub enum ActionError
{
  SetParent(SetParentError),
  SetName(SetNameError),
  Acl(AclActionError),
}

impl Apply<Action> for Workspace
{
  type Error = ActionError;

  fn mutates_name(action: &Action) -> bool {
    match action {
      Action::SetName(_) => true,
      _ => false,
    }
  }

  fn mutates_parent(_action: &Action) -> bool {
    match _action {
      Action::SetParent(_) => true,
      _ => false,
    }
  }

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetParent(action) => self.apply(action)?,
      Action::SetName(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
