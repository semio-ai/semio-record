use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::{Animation, Key};

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_AddKey")]
pub struct AddKey {
  pub control_id: Uuid,
  pub key_id: Uuid,
  pub key: Key,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_AddKeyError", tag = "type", rename_all = "camelCase")]
pub enum AddKeyError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key already exists")]
  KeyExists,
}

impl Apply<AddKey> for Animation {
  type Error = AddKeyError;

  fn apply(&mut self, action: &AddKey) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.control_id) {
      if control.keys.contains_key(&action.key_id) {
        return Err(AddKeyError::KeyExists);
      }

      control.keys.insert(action.key_id, action.key.clone());

      control.key_ordering.push(action.key_id);
      control.key_ordering.sort_by(|a, b| {
        let a = control.keys.get(a).unwrap();
        let b = control.keys.get(b).unwrap();
        a.at.partial_cmp(&b.at)
          .unwrap_or(std::cmp::Ordering::Equal)
      });
    } else {
      return Err(AddKeyError::ControlDoesNotExist);
    }

    Ok(())
  }
}