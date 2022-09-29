use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

use crate::record::Apply;

use schemars::JsonSchema;

use super::KeySelector;
use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveKeys", rename_all = "camelCase")]
pub struct RemoveKeys {
  pub selectors: Vec<KeySelector>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveKeysError", tag = "type", rename_all = "camelCase")]
pub enum RemoveKeysError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<RemoveKeys> for Animation {
  type Error = RemoveKeysError;

  fn apply(&mut self, action: &RemoveKeys) -> Result<(), Self::Error> {
    // Check for errors first

    for selector in &action.selectors {
      if let Some(control) = self.controls.get(&selector.control_id) {
        if !control.keys.contains_key(&selector.key_id) {
          return Err(RemoveKeysError::KeyDoesNotExist);
        }
      } else {
        return Err(RemoveKeysError::ControlDoesNotExist);
      }
    }

    // Apply
    for selector in &action.selectors {
      if let Some(control) = self.controls.get_mut(&selector.control_id) {
        control.keys.remove(&selector.key_id);
        control.key_ordering.retain(|k| k != &selector.key_id);
      }
    }

    Ok(())
  }
}