use serde::{Deserialize, Serialize};

use derive_more::{Display, From, Error};

use crate::action::{SetParent, SetName, SetParentError, SetNameError};

use crate::acl::action::{Action as AclAction, ActionError as AclActionError};
use crate::record::Apply;

use super::unfrozen::Scene;

#[derive(Debug, Clone, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Action", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Action {
  SetName(SetName),
  Acl(AclAction),
}

#[derive(Display, Clone, Debug, Error, Serialize, Deserialize, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_ActionError", tag = "type", rename_all = "camelCase")]
pub enum ActionError
{
  SetName(SetNameError),
  Acl(AclActionError),
}

impl Apply<Action> for Scene
{
  type Error = ActionError;

  fn mutates_name(action: &Action) -> bool {
    match action {
      Action::SetName(_) => true,
      _ => false,
    }
  }

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
