
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::unfrozen::{Animation, Node, IdNode, IdControl, Control};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub controls: HashMap<Uuid, Control>,
  pub nodes: HashMap<Uuid, Node>,
}

#[graphql_object(name = "AnimationPublic")]
impl Public {
  fn name(&self) -> &str {
    &self.name
  }

  fn parent(&self) -> &Uuid {
    &self.parent
  }

  fn controls(&self) -> Vec<IdControl> {
    self.controls
      .iter()
      .map(|(id, control)| IdControl {
        id: *id,
        control: control.clone(),
      })
      .collect()
  }

  fn nodes(&self) -> Vec<IdNode> {
    self.nodes
      .iter()
      .map(|(id, node)| IdNode {
        id: *id,
        node: node.clone(),
      })
      .collect()
  }
}

impl From<Animation> for Public {
  fn from(animation: Animation) -> Self {
    Self {
      name: animation.name,
      parent: animation.parent,
      controls: animation.controls,
      nodes: animation.nodes,
    }
  }
}