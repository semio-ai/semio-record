use std::collections::{HashSet, HashMap};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer}, blob::BlobDependencies, unfrozen::impl_unfrozen, acl::{Acl, action::with_acl}, action::{name, parent}, color::Color, math::MultiBezier2};

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct F64 {
  pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, Clone)]
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
pub struct Group {
  pub name: String,
  pub locked: bool,
  pub collapsed: bool,
  pub color: Color,
  // Children nodes (of any type)
  pub children_ids: HashSet<Uuid>,
}

#[graphql_object]
impl Group {
  fn name(&self) -> &str {
    &self.name
  }

  fn locked(&self) -> bool {
    self.locked
  }

  fn collapsed(&self) -> bool {
    self.collapsed
  }

  fn color(&self) -> &Color {
    &self.color
  }

  fn children_ids(&self) -> Vec<Uuid> {
    self.children_ids.iter().cloned().collect()
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
  pub name: String,
  pub locked: bool,
  pub collapsed: bool,
  pub color: Color,

  pub keys: HashMap<Uuid, Key>,
  pub key_ordering: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct IdKey {
  pub id: Uuid,
  pub key: Key,
}

#[graphql_object]
impl Track {
  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn locked(&self) -> bool {
    self.locked
  }

  pub fn collapsed(&self) -> bool {
    self.collapsed
  }

  pub fn color(&self) -> &Color {
    &self.color
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

#[derive(Debug, Serialize, Deserialize, GraphQLUnion, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Node {
  Group(Group),
  Track(Track),
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Animation {
  pub name: String,
  pub parent: Uuid,
  pub root_id: Uuid,
  pub nodes: HashMap<Uuid, Node>,
  pub acl: Acl,
}

impl Default for Animation {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      parent: Uuid::new_v4(),
      root_id: Uuid::new_v4(),
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

#[graphql_object]
impl Animation {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn parent(&self) -> &Uuid {
    &self.parent
  }

  pub fn root_id(&self) -> &Uuid {
    &self.root_id
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