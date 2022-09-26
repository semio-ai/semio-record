
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use schemars::JsonSchema;

use super::unfrozen::{Animation, Node, IdNode};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub root_id: Uuid,
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

  fn root_id(&self) -> &Uuid {
    &self.root_id
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
      root_id: animation.root_id,
      nodes: animation.nodes,
    }
  }
}