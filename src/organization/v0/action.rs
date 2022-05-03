use crate::{record::Apply, action::{SetName, SetNameError}, acl::action::{Action as AclAction, ActionError as AclActionError}};

use super::unfrozen::Organization;
use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  SetName(SetName),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Clone, Serialize, Deserialize)]
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