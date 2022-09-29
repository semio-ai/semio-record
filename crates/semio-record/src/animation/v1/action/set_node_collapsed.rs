use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_SetNodeCollapsed", rename_all = "camelCase")]
pub struct SetNodeCollapsed {
  pub node_id: Uuid,
  pub collapsed: bool,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(rename = "animation_v1_Action_SetNodeCollapsedError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeCollapsedError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeCollapsed> for Animation {
  type Error = SetNodeCollapsedError;

  fn apply(&mut self, action: &SetNodeCollapsed) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.node_id) {
      node.collapsed = action.collapsed
    } else {
      return Err(SetNodeCollapsedError::NodeDoesNotExist);
    }

    Ok(())
  }
}