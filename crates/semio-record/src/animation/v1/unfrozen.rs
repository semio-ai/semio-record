use std::collections::{HashSet, HashMap};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use derive_more::From;

use crate::{
  record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer},
  blob::BlobDependencies,
  unfrozen::impl_unfrozen,
  acl::{Acl, action::with_acl},
  action::{name, parent},
  math::MultiBezier2, migrate::Migrate
};

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Value_F64", rename_all = "camelCase")]
pub struct F64 {
  pub value: f64,
}

impl From<crate::animation::v0::unfrozen::F64> for F64 {
  fn from(value: crate::animation::v0::unfrozen::F64) -> Self {
    Self {
      value: value.value,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Value_Kind", rename_all = "camelCase")]
pub enum ValueKind {
  F64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Value", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Value {
  F64(F64),
}

impl From<crate::animation::v0::unfrozen::Value> for Value {
  fn from(value: crate::animation::v0::unfrozen::Value) -> Self {
    match value {
      crate::animation::v0::unfrozen::Value::F64(value) => Self::F64(value.into())
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Transition_MultiBezier", rename_all = "camelCase")]
pub struct MultiBezier {
  pub multi_bezier: MultiBezier2,
}

impl From<crate::animation::v0::unfrozen::MultiBezier> for MultiBezier {
  fn from(value: crate::animation::v0::unfrozen::MultiBezier) -> Self {
    Self {
      multi_bezier: value.multi_bezier,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Transition", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Transition {
  None,
  Linear,
  MultiBezier(MultiBezier),
}

impl From<crate::animation::v0::unfrozen::Transition> for Transition {
  fn from(value: crate::animation::v0::unfrozen::Transition) -> Self {
    match value {
      crate::animation::v0::unfrozen::Transition::None(value) => Self::None,
      crate::animation::v0::unfrozen::Transition::Linear(value) => Self::Linear,
      crate::animation::v0::unfrozen::Transition::MultiBezier(value) => Self::MultiBezier(value.into()),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Key", rename_all = "camelCase")]
pub struct Key {
  pub at: f64,
  pub value: Value,
  pub transition: Transition,
}

impl From<crate::animation::v0::unfrozen::Key> for Key {
  fn from(key: crate::animation::v0::unfrozen::Key) -> Self {
    Key {
      at: key.at,
      value: key.value.into(),
      transition: key.transition.into()
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Control", rename_all = "camelCase")]
pub struct Control {
  pub value_kind: ValueKind,
  pub name: String,
  pub locked: bool,
  pub keys: HashMap<Uuid, Key>,
  pub key_ordering: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Node_Kind_Group", rename_all = "camelCase")]
pub struct GroupNode {
  // Children nodes (of any type)
  pub children_ids: HashSet<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Node_Kind_Control", rename_all = "camelCase")]
pub struct ControlNode {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, From)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Node_Kind", tag = "type", rename_all = "camelCase", content = "value")]
pub enum NodeKind {
  Group(GroupNode),
  Control(ControlNode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Node", rename_all = "camelCase")]
pub struct Node {
  pub name: Option<String>,
  pub collapsed: bool,
  pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V1_Private", rename_all = "camelCase")]
pub struct Animation {
  /// Human-readable name of the animation
  pub name: String,

  /// The parent record ID
  pub parent: Uuid,

  /// The controls
  pub controls: HashMap<Uuid, Control>,
  
  /// The organization of controls into a forest
  pub nodes: HashMap<Uuid, Node>,

  pub acl: Acl,
}

impl Default for Animation {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      parent: Uuid::nil(),
      controls: HashMap::new(),
      nodes: HashMap::new(),
      acl: Default::default(),
    }
  }
}

impl From<crate::animation::v0::unfrozen::Animation> for Animation {
  fn from(v0: crate::animation::v0::unfrozen::Animation) -> Self {
    use crate::animation::v0::unfrozen as unfrozen;
    
    let mut controls = HashMap::new();
    let mut nodes = HashMap::new();

    for (id, node) in v0.nodes.into_iter() {
      match node {
        unfrozen::Node::Track(track) => {
          let control = Control {
            locked: track.locked,
            name: track.name.clone(),
            value_kind: match track.keys.get(&track.key_ordering[0]).unwrap().value {
              unfrozen::Value::F64(_) => ValueKind::F64,
            },
            keys: track.keys
              .into_iter()
              .map(|(id, key)| (id, key.into()))
              .collect(),
            key_ordering: track.key_ordering,
          };

          let control_id = Uuid::new_v4();
          controls.insert(control_id, control);
          nodes.insert(id, Node {
            name: None,
            collapsed: track.collapsed,
            kind: NodeKind::Control(ControlNode { id: control_id }),
          });
        }
        unfrozen::Node::Group(group) => {
          nodes.insert(id, Node {
            name: Some(group.name),
            collapsed: group.collapsed,
            kind: NodeKind::Group(GroupNode { children_ids: group.children_ids }),
          });
        }
      }
    }
    
    Self {
      name: v0.name,
      parent: v0.parent,
      controls,
      nodes,
      acl: v0.acl,
    }
  }
}

impl_unfrozen!(Animation, Action);
name!(Animation);
parent!(Animation);
with_acl!(Animation);

impl Unfrozen<Action> for Animation {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Animation {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

impl View for Animation {
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
impl<F: Freezer> Freeze<F> for Animation {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for Animation {
  fn migrate(from_version: i16, from: &[u8]) -> anyhow::Result<Self> {
    
    match from_version {
      0 => {
        let v0: crate::animation::v0::unfrozen::Animation = crate::deserialize(from)?;
        Ok(v0.into())
      }
      _ => {
        anyhow::bail!("Unsupported version: {}", from_version)
      }
    }
  }
}