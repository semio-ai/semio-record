use std::collections::HashSet;

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::{Animation, NodeKind};

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveControl")]
pub struct RemoveControl {
  pub control_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_RemoveControlError", tag = "type", rename_all = "camelCase")]
pub enum RemoveControlError {
  #[display(fmt = "Control does not exist")]
  ControlDoesNotExist,
}

impl Apply<RemoveControl> for Animation {
  type Error = RemoveControlError;

  fn apply(&mut self, action: &RemoveControl) -> Result<(), Self::Error> {
    if let None = self.controls.remove(&action.control_id) {
      return Err(RemoveControlError::ControlDoesNotExist);
    }

    let mut remove_ids = HashSet::new();
    for (id, node) in self.nodes.iter() {
      if let NodeKind::Control(c) = &node.kind {
        if c.id == action.control_id {
          remove_ids.insert(id.clone());
        }
      }
    }

    for id in remove_ids.iter() {
      self.nodes.remove(id);
    }

    // Remove children_ids
    for node in self.nodes.values_mut() {
      if let NodeKind::Group(g) = &mut node.kind {
        g.children_ids.retain(|id| !remove_ids.contains(id));
      }
    }
    
    Ok(())
  }
}