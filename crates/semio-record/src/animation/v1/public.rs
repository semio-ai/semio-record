
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::unfrozen::{Animation, Node, Control};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v1_Public", rename_all = "camelCase")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub controls: HashMap<Uuid, Control>,
  pub nodes: HashMap<Uuid, Node>,
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