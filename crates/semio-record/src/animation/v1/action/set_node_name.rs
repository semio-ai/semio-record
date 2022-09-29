use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;



use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetNodeName", rename_all = "camelCase")]
pub struct SetNodeName {
  pub node_id: Uuid,
  pub name: Option<String>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetNodeNameError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeNameError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeName> for Animation {
  type Error = SetNodeNameError;

  fn apply(&mut self, action: &SetNodeName) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.node_id) {
      node.name = action.name.clone();
    } else {
      return Err(SetNodeNameError::NodeDoesNotExist);
    }

    Ok(())
  }
}