use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::{Animation, Transition};

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct SetKeyTransition {
  pub control_id: Uuid,
  pub key_id: Uuid,
  pub transition: Transition,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetKeyTransitionError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<SetKeyTransition> for Animation {
  type Error = SetKeyTransitionError;

  fn apply(&mut self, action: &SetKeyTransition) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
      if let Some(key) = control.keys.get_mut(&action.key_id) {
        key.transition = action.transition.clone();
      } else {
        return Err(SetKeyTransitionError::KeyDoesNotExist);
      }
    } else {
      return Err(SetKeyTransitionError::ControlDoesNotExist);
    }

    Ok(())
  }
}