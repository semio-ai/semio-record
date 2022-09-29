use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{record::Apply, animation::v1::unfrozen::NodeKind};



use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetNodeParent", rename_all = "camelCase")]
pub struct SetNodeParent {
  pub id: Uuid,
  pub parent_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_SetNodeParentError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeParentError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Parent does not exist")]
  ParentDoesNotExist,
  #[display(fmt = "Parent is not a group")]
  ParentIsNotGroup,
}

impl Apply<SetNodeParent> for Animation {
  type Error = SetNodeParentError;

  fn apply(&mut self, action: &SetNodeParent) -> Result<(), Self::Error> {
    // Check if node exists
    if let None = self.nodes.get(&action.id) {
      return Err(SetNodeParentError::NodeDoesNotExist);
    }

    // Remove node from old parent
    for node in self.nodes.values_mut() {
      if let NodeKind::Group(g) = &mut node.kind {
        g.children_ids.remove(&action.id);
      }
    }

    // Add node to new parent
    if let Some(parent_node) = self.nodes.get_mut(&action.parent_id) {
      match &mut parent_node.kind {
        NodeKind::Group(g) => {
          g.children_ids.insert(action.id);
        }
        _ => return Err(SetNodeParentError::ParentIsNotGroup),
      }
    } else {
      return Err(SetNodeParentError::ParentDoesNotExist);
    }


    Ok(())
  }
}