use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct SetControlName {
  pub control_id: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetControlNameError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
}

impl Apply<SetControlName> for Animation {
  type Error = SetControlNameError;

  fn apply(&mut self, action: &SetControlName) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
      control.name = action.name.clone();
    } else {
      return Err(SetControlNameError::ControlDoesNotExist);
    }

    Ok(())
  }
}