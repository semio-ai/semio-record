use std::cmp::Ordering;

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::Animation;
use super::KeySelectorAt;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct SetKeysAt {
  pub ats: Vec<KeySelectorAt>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetKeysAtError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<SetKeysAt> for Animation {
  type Error = SetKeysAtError;

  fn apply(&mut self, action: &SetKeysAt) -> Result<(), Self::Error> {
    // Validate all key selectors before mutating
    for selector in &action.ats {
      if let Some(control) = self.controls.get(&selector.selector.control_id) {
        if let Some(_) = control.keys.get(&selector.selector.key_id) {
          // do nothing
        } else {
          return Err(SetKeysAtError::KeyDoesNotExist);
        }
      } else {
        return Err(SetKeysAtError::ControlDoesNotExist);
      }
    }

    for at in &action.ats {
      if let Some(control) = self.controls.get_mut(&at.selector.control_id) {
        if let Some(key) = control.keys.get_mut(&at.selector.key_id) {
          key.at = at.at;
          // sort key ordering
          control.key_ordering.sort_by(|a, b| {
            let a = control.keys.get(a).unwrap();
            let b = control.keys.get(b).unwrap();
            a.at.partial_cmp(&b.at)
              .unwrap_or(Ordering::Equal)
          });
        }
      }
    }

    Ok(())
  }
}