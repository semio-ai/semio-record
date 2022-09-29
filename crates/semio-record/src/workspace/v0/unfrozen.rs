use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{
  reference::Reference,
  scene::v0::unfrozen::Scene,
  animation::v1::unfrozen::Animation, acl::Acl
};

use serde::{Serialize, Deserialize};

use derive_more::From;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum AnimationBindingTarget {
  Scene,
  Node(Uuid)
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub struct AnimationSelectionNode {
  pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub struct AnimationSelectionControl {
  pub control_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub struct AnimationSelectionKey {
  pub control_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub enum AnimationSelection {
  Node(AnimationSelectionNode),
  Control(AnimationSelectionControl),
  Key(AnimationSelectionKey)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationBinding {
  pub target: AnimationBindingTarget,
  pub animation: Reference<Animation>,
  pub selections: HashSet<AnimationSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub struct SceneSelectionNode {
  pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
pub enum SceneSelection {
  Node(SceneSelectionNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBinding {
  pub scene: Reference<Scene>,
  pub animation_bindings: HashMap<Uuid, AnimationBinding>,
  pub selections: HashSet<SceneSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum ContextKind {
  Scene(SceneBinding),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
  pub kind: ContextKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
  pub parent: Uuid,
  pub acl: Acl,

  pub active_context: Option<Uuid>,
  pub context_ordering: Vec<Uuid>,
  pub contexts: HashMap<Uuid, Context>,
}