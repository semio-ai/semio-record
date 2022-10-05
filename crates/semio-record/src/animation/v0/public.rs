
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::unfrozen::{Animation, Node};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
  pub root_id: Uuid,
  pub nodes: HashMap<Uuid, Node>,
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

impl From<Public> for Animation {
  fn from(public: Public) -> Self {
    Self {
      name: public.name,
      parent: public.parent,
      root_id: public.root_id,
      nodes: public.nodes,
      ..Default::default()
    }
  }
}