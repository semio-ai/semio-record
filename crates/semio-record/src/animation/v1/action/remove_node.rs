use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::{Animation, NodeKind};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_RemoveNode", rename_all = "camelCase")]
pub struct RemoveNode {
  pub node_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Action_RemoveNodeError", tag = "type", rename_all = "camelCase")]
pub enum RemoveNodeError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<RemoveNode> for Animation {
  type Error = RemoveNodeError;

  fn apply(&mut self, action: &RemoveNode) -> Result<(), Self::Error> {
    if let None = self.nodes.remove(&action.node_id) {
      return Err(RemoveNodeError::NodeDoesNotExist);
    }

    self.nodes.remove(&action.node_id);

    // Remove id from other nodes
    for node in self.nodes.values_mut() {
      if let NodeKind::Group(g) = &mut node.kind {
        g.children_ids.retain(|id| id != &action.node_id);
      }
    }
    
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::collections::{HashMap, HashSet};
  use uuid::Uuid;
  use crate::{animation::v1::unfrozen::{Animation, Node, NodeKind, ControlNode, GroupNode}, record::Apply};
  use super::RemoveNode;

  #[test]
  fn simple() {
    let node_id = Uuid::new_v4();
    let mut nodes = HashMap::new();
    nodes.insert(node_id.clone(), Node {
      name: None,
      collapsed: false,
      kind: ControlNode {
        id: Uuid::new_v4(),
      }.into()
    });


    let mut animation = Animation {
      nodes,
      ..Default::default()
    };

    animation.apply(&RemoveNode {
      node_id,
    }).unwrap();

    assert_eq!(animation.nodes.len(), 0);
  }

  #[test]
  fn group() {
    let control_node_id = Uuid::new_v4();
    let group_node_id = Uuid::new_v4();
    
    let mut nodes = HashMap::new();
    nodes.insert(control_node_id.clone(), Node {
      collapsed: false,
      name: None,
      kind: ControlNode {
        id: Uuid::new_v4(),
      }.into()
    });

    nodes.insert(group_node_id, Node {
      collapsed: false,
      name: None,
      kind: GroupNode {
        children_ids: {
          let mut ret = HashSet::new();
          ret.insert(control_node_id.clone());
          ret
        },
      }.into()
    });


    let mut animation = Animation {
      nodes,
      ..Default::default()
    };

    animation.apply(&RemoveNode {
      node_id: control_node_id,
    }).unwrap();

    assert_eq!(animation.nodes.len(), 1);

    let group_node = animation.nodes.get(&group_node_id).unwrap();

    if let NodeKind::Group(g) = &group_node.kind {
      assert_eq!(g.children_ids.len(), 0);
    } else {
      panic!("Expected group node");
    }
  }
}