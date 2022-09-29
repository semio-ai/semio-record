use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;


use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_SetControlLocked", rename_all = "camelCase")]
pub struct SetControlLocked {
  pub control_id: Uuid,
  pub locked: bool,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_SetControlLockedError", tag = "type", rename_all = "camelCase")]
pub enum SetControlLockedError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
}

impl Apply<SetControlLocked> for Animation {
  type Error = SetControlLockedError;

  fn apply(&mut self, action: &SetControlLocked) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
      control.locked = action.locked;
    } else {
      return Err(SetControlLockedError::ControlDoesNotExist);
    }

    Ok(())
  }
}