use crate::record::Apply;

use super::unfrozen::{User};
use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Display, From)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {

}

#[derive(Display, Debug, Error, From)]
pub enum ActionError {

}

impl Apply<Action> for User {
  type Error = ActionError;
  
  fn apply(&mut self, _action: &Action) -> Result<(), Self::Error> {
    Ok(())
  }
}