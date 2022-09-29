use std::collections::{HashSet, HashMap};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer},
  blob::BlobDependencies,
  unfrozen::impl_unfrozen,
  acl::{Acl, action::with_acl},
  action::{name, parent},
  color::{Color, Rgb},
  math::MultiBezier2, migrate::Migrate
};

use super::action::Action;



#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Value_F64", rename_all = "camelCase")]
pub struct F64 {
  pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Value", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Value {
  F64(F64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Transition_None")]
pub struct None {
  pub _dummy: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Transition_Linear")]
pub struct Linear {
  pub _dummy: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Transition_MultiBezier")]
pub struct MultiBezier {
  pub multi_bezier: MultiBezier2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Transition", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Transition {
  None(None),
  Linear(Linear),
  MultiBezier(MultiBezier),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Key")]
pub struct Key {
  pub at: f64,
  pub value: Value,
  pub transition: Transition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Group")]
pub struct Group {
  pub name: String,
  pub locked: bool,
  pub collapsed: bool,
  pub color: Color,
  // Children nodes (of any type)
  pub children_ids: HashSet<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Track")]
pub struct Track {
  pub name: String,
  pub locked: bool,
  pub collapsed: bool,
  pub color: Color,

  pub keys: HashMap<Uuid, Key>,
  pub key_ordering: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct IdKey {
  pub id: Uuid,
  pub key: Key,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Node", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Node {
  Group(Group),
  Track(Track),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_v0_Private")]
pub struct Animation {
  pub name: String,
  pub parent: Uuid,
  pub root_id: Uuid,
  pub nodes: HashMap<Uuid, Node>,
  pub acl: Acl,
}

impl Default for Animation {
  fn default() -> Self {
    let mut nodes = HashMap::new();

    let root_id = Uuid::new_v4();

    nodes.insert(root_id.clone(), Node::Group(Group {
      name: "Root".to_string(),
      children_ids: HashSet::new(),
      collapsed: false,
      color: Color::Rgb(Rgb {
        r: 1.0,
        g: 0.0,
        b: 0.0
      }),
      locked: false,
    }));

    Self {
      name: "".to_string(),
      parent: Uuid::nil(),
      root_id,
      nodes,
      acl: Default::default(),
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
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}