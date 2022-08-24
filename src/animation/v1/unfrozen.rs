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

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct F64 {
  pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, GraphQLEnum, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ValueKind {
  F64,
}

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum Value {
  F64(F64),
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct None {
  pub _dummy: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Linear {
  pub _dummy: i32,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct MultiBezier {
  pub multi_bezier: MultiBezier2,
}

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum Transition {
  None(None),
  Linear(Linear),
  MultiBezier(MultiBezier),
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Key {
  pub at: f64,
  pub value: Value,
  pub transition: Transition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Control {
  pub value_kind: ValueKind,
  pub name: String,
  pub locked: bool,
  pub keys: HashMap<Uuid, Key>,
  pub key_ordering: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct IdKey {
  pub id: Uuid,
  pub key: Key,
}

#[graphql_object]
impl Control {
  pub fn value_kind(&self) -> ValueKind {
    self.value_kind
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn locked(&self) -> bool {
    self.locked
  }

  pub fn keys(&self) -> Vec<IdKey> {
    self.key_ordering
      .iter()
      .map(|id| IdKey {
        id: *id,
        key: self.keys.get(id).unwrap().clone(),
      })
      .collect()
  }

  pub fn key_ordering(&self) -> &Vec<Uuid> {
    &self.key_ordering
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupNode {
  pub name: String,
  pub collapsed: bool,
  // Children nodes (of any type)
  pub children_ids: HashSet<Uuid>,
}

#[graphql_object]
impl GroupNode {
  fn name(&self) -> &str {
    &self.name
  }

  fn collapsed(&self) -> bool {
    self.collapsed
  }

  fn children_ids(&self) -> Vec<Uuid> {
    self.children_ids.iter().cloned().collect()
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct ControlNode {
  pub collapsed: bool,
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, Clone, From)]
#[serde(tag = "type", rename_all = "lowercase", content = "value")]
pub enum Node {
  Group(GroupNode),
  Control(ControlNode),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct IdNode {
  pub id: Uuid,
  pub node: Node,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct IdControl {
  pub id: Uuid,
  pub control: Control,
}

#[graphql_object]
impl Animation {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn parent(&self) -> &Uuid {
    &self.parent
  }

  pub fn nodes(&self) -> Vec<IdNode> {
    self.nodes
      .iter()
      .map(|(id, node)| IdNode {
        id: *id,
        node: node.clone(),
      })
      .collect()
  }

  pub fn controls(&self) -> Vec<IdControl> {
    self.controls
      .iter()
      .map(|(id, control)| IdControl {
        id: *id,
        control: control.clone(),
      })
      .collect()
  }

  pub fn acl(&self) -> &Acl {
    &self.acl
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
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}