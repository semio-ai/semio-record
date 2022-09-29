use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::{Animation, Control};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_AddControl", rename_all = "camelCase")]
pub struct AddControl {
  pub control_id: Uuid,
  pub control: Control,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_AddControlError", tag = "type", rename_all = "camelCase")]
pub enum AddControlError {
  #[display(fmt = "Control already exists")]
  ControlExists,
}

impl Apply<AddControl> for Animation {
  type Error = AddControlError;

  fn apply(&mut self, action: &AddControl) -> Result<(), Self::Error> {
    if self.controls.contains_key(&action.control_id) {
      return Err(AddControlError::ControlExists);
    }

    self.controls.insert(action.control_id, action.control.clone());

    Ok(())
  }
}