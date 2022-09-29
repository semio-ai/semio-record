use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::Animation;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveKey", rename_all = "camelCase")]
pub struct RemoveKey {
  pub control_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveKeyError", tag = "type", rename_all = "camelCase")]
pub enum RemoveKeyError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<RemoveKey> for Animation {
  type Error = RemoveKeyError;

  fn apply(&mut self, action: &RemoveKey) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
      if !control.keys.contains_key(&action.key_id) {
        return Err(RemoveKeyError::KeyDoesNotExist);
      }

      control.keys.remove(&action.key_id);
      control.key_ordering.retain(|k| k != &action.key_id);
    } else {
      return Err(RemoveKeyError::ControlDoesNotExist);
    }

    Ok(())
  }
}