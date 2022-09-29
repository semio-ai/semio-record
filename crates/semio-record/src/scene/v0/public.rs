use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashMap;

use super::unfrozen::{Scene, Geometry, Node};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Public", rename_all = "camelCase")]
pub struct Public {
  pub name: String,
  pub description: String,
  pub geometry: HashMap<Uuid, Geometry>,
  pub nodes: HashMap<Uuid, Node>
}

impl From<Scene> for Public {
  fn from(scene: Scene) -> Self {
    Self {
      name: scene.name,
      description: scene.description,
      geometry: scene.geometry,
      nodes: scene.nodes
    }
  }
}
