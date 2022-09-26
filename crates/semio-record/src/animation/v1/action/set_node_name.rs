use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use schemars::JsonSchema;

use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct SetNodeName {
  pub node_id: Uuid,
  pub name: Option<String>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
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