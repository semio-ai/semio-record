use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::{Animation, Node};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
pub struct AddNode {
  pub node_id: Uuid,
  pub node: Node,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum AddNodeError {
  #[display(fmt = "Node already exists")]
  NodeExists,
}

impl Apply<AddNode> for Animation {
  type Error = AddNodeError;

  fn apply(&mut self, action: &AddNode) -> Result<(), Self::Error> {
    if self.nodes.contains_key(&action.node_id) {
      return Err(AddNodeError::NodeExists);
    }

    self.nodes.insert(action.node_id, action.node.clone());

    Ok(())
  }
}