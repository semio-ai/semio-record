use std::cmp::Ordering;

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetKeyAt", rename_all = "camelCase")]
pub struct SetKeyAt {
  pub control_id: Uuid,
  pub key_id: Uuid,
  pub at: f64,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetKeyAtError", tag = "type", rename_all = "camelCase")]
pub enum SetKeyAtError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<SetKeyAt> for Animation {
  type Error = SetKeyAtError;

  fn apply(&mut self, action: &SetKeyAt) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
        if let Some(key) = control.keys.get_mut(&action.key_id) {
          key.at = action.at;
          // sort key ordering
          control.key_ordering.sort_by(|a, b| {
            let a = control.keys.get(a).unwrap();
            let b = control.keys.get(b).unwrap();
            a.at.partial_cmp(&b.at)
              .unwrap_or(Ordering::Equal)
          });
        } else {
          return Err(SetKeyAtError::KeyDoesNotExist);
        }
    } else {
      return Err(SetKeyAtError::ControlDoesNotExist);
    }

    Ok(())
  }
}