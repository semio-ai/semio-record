use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
  container::UnfrozenContainer,
  scene::v0::unfrozen::Scene,
  action::{name, parent},
  animation::v1::unfrozen::Animation, acl::{Acl, action::with_acl}, record::{Freezer, Freeze, View, Unfrozen, UnfrozenReference}, migrate::Migrate, unfrozen::impl_unfrozen, blob::BlobDependencies
};

use super::action::Action;

use serde::{Serialize, Deserialize};

use derive_more::From;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Binding_Target", tag = "type", content = "value", rename_all = "camelCase")]
pub enum AnimationBindingTarget {
  Scene,
  Node(Uuid)
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Selection_Node", rename_all = "camelCase")]
pub struct AnimationSelectionNode {
  pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Selection_Control", rename_all = "camelCase")]
pub struct AnimationSelectionControl {
  pub control_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Selection_Key", rename_all = "camelCase")]
pub struct AnimationSelectionKey {
  pub control_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Selection", rename_all = "camelCase", tag = "type", content = "value")]
pub enum AnimationSelection {
  Node(AnimationSelectionNode),
  Control(AnimationSelectionControl),
  Key(AnimationSelectionKey)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Animation_Binding", rename_all = "camelCase")]
pub struct AnimationBinding {
  pub target: AnimationBindingTarget,
  pub animation: UnfrozenContainer<Animation>,
  pub selections: HashSet<AnimationSelection>,
}

impl AnimationBinding {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.animation.dependencies(set);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Scene_Selection_Node", rename_all = "camelCase")]
pub struct SceneSelectionNode {
  pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, From, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Scene_Selection", rename_all = "camelCase", tag = "type", content = "value")]
pub enum SceneSelection {
  Node(SceneSelectionNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Scene_Binding", rename_all = "camelCase")]
pub struct SceneBinding {
  pub scene: UnfrozenContainer<Scene>,
  pub animation_bindings: HashMap<Uuid, AnimationBinding>,
  pub selections: HashSet<SceneSelection>,
}

impl SceneBinding {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.scene.dependencies(set);
    for binding in self.animation_bindings.values() {
      binding.animation.dependencies(set);
    }
  }

  pub fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    self.scene.blob_dependencies(set);
    for binding in self.animation_bindings.values() {
      binding.animation.blob_dependencies(set);
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Context_Kind", tag = "type", content = "value", rename_all = "camelCase")]
pub enum ContextKind {
  Scene(SceneBinding),
}

impl ContextKind {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      ContextKind::Scene(scene) => {
        scene.dependencies(set);
      }
    }
  }

  pub fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    match self {
      ContextKind::Scene(scene) => {
        scene.blob_dependencies(set);
      }
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Context", rename_all = "camelCase")]
pub struct Context {
  pub kind: ContextKind,
}

impl Context {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.kind.dependencies(set);
  }

  pub fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    self.kind.blob_dependencies(set);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Private", rename_all = "camelCase")]
pub struct Workspace {
  pub parent: Uuid,
  pub acl: Acl,
  pub name: String,

  pub active_context: Option<Uuid>,
  pub context_ordering: Vec<Uuid>,
  pub contexts: HashMap<Uuid, Context>,
}

impl_unfrozen!(Workspace, Action);

impl Unfrozen<Action> for Workspace {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for context in self.contexts.values() {
      context.dependencies(set);
    }
  }
}

impl BlobDependencies for Workspace {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    for context in self.contexts.values() {
      context.blob_dependencies(set);
    }
  }
}

with_acl!(Workspace);
parent!(Workspace);
name!(Workspace);

impl View for Workspace {
  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    Some(&self.parent)
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Workspace {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for Workspace {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}