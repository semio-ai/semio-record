use crate::{record::Apply, action::{SetName, SetNameError}, acl::action::{Action as AclAction, ActionError as AclActionError}};

use super::unfrozen::Organization;
use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "organization_v0_Action", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Action {
  SetName(SetName),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "organization_v0_ActionError", tag = "type", rename_all = "camelCase", content = "value")]
pub enum ActionError {
  SetName(SetNameError),
  Acl(AclActionError),
}

impl Apply<Action> for Organization {
  type Error = ActionError;
  
  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }
    
    Ok(())
  }
}