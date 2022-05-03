use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

use crate::{
  acl::action::{Action as AclAction, ActionError as AclActionError},
  action::{SetName, SetNameError, SetParent, SetParentError},
  record::Apply,
};

use super::unfrozen::Folder;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
  SetName(SetName),
  SetParent(SetParent),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Serialize, Deserialize, Clone)]
pub enum ActionError {
  SetName(SetNameError),
  SetParent(SetParentError),
  Acl(AclActionError),
}

impl Apply<Action> for Folder {
  type Error = ActionError;

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::SetParent(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
