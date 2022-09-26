use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{record::Apply, animation::v1::unfrozen::NodeKind};

use super::super::unfrozen::{Animation, Node};

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct AddNode {
  pub parent_id: Option<Uuid>,
  pub node_id: Uuid,
  pub node: Node,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum AddNodeError {
  #[display(fmt = "Node already exists")]
  NodeExists,
  #[display(fmt = "Parent node does not exist")]
  ParentDoesNotExist,
  #[display(fmt = "Parent node is not a group")]
  ParentIsNotGroup,
}

impl Apply<AddNode> for Animation {
  type Error = AddNodeError;

  fn apply(&mut self, action: &AddNode) -> Result<(), Self::Error> {
    if self.nodes.contains_key(&action.node_id) {
      return Err(AddNodeError::NodeExists);
    }

    if let Some(parent_id) = action.parent_id {
      if !self.nodes.contains_key(&parent_id) {
        return Err(AddNodeError::ParentDoesNotExist);
      }

      if let NodeKind::Group(g) = &mut self.nodes.get_mut(&parent_id).unwrap().kind {
        g.children_ids.insert(action.node_id);
      } else {
        return Err(AddNodeError::ParentIsNotGroup);
      }
    }

    self.nodes.insert(action.node_id, action.node.clone());

    Ok(())
  }
}