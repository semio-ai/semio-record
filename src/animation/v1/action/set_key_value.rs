use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::{Animation, Value};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
pub struct SetKeyValue {
  pub id: Uuid,
  pub key_id: Uuid,
  pub value: Value,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetKeyValueError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
}

impl Apply<SetKeyValue> for Animation {
  type Error = SetKeyValueError;

  fn apply(&mut self, action: &SetKeyValue) -> Result<(), Self::Error> {
    if let Some(control) = self.controls.get_mut(&action.id) {
      if let Some(key) = control.keys.get_mut(&action.key_id) {
        key.value = action.value.clone();
      } else {
        return Err(SetKeyValueError::KeyDoesNotExist);
      }
    } else {
      return Err(SetKeyValueError::ControlDoesNotExist);
    }

    Ok(())
  }
}