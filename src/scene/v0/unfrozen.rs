use std::collections::HashSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer}, blob::BlobDependencies, unfrozen::impl_unfrozen, acl::{Acl, action::with_acl}, action::name};

use super::action::Action;

pub struct Box {
  pub _dummy: i32
}

pub struct Geometry {
  pub geometry_id: Uuid,
}

pub enum Collider {
  Box(Box),
  Geometry(Geometry),
}

pub struct Physics {
  pub collider: Option<Collider>,
  pub fixed: Option<bool>,
  pub mass: 
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Empty {
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Object {
  geometry_id: Uuid,
  physics: Option<>
}



#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub enum NodeKind {
  Empty(Empty),
  Object,
  PointLight,
  SpotLight,
  DirectionalLight,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Node {
  pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Scene {
  pub name: String,
  pub description: String,
  pub acl: Acl,
  pub nodes: HashMap<Uuid, Node>
}

impl_unfrozen!(Organization, Action);

impl Unfrozen<Action> for Organization {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Organization {
  fn blob_dependencies<'a>(&'a self, _set: &mut HashSet<&'a uuid::Uuid>) {}
}

with_acl!(Organization);

impl View for Organization {
  fn name(&self) -> Option<&str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a uuid::Uuid> {
    None
  }

  fn acl<'a>(&'a self) -> Option<&'a Acl> {
    Some(&self.acl)
  }
}

name!(Organization);

#[async_trait]
impl<F: Freezer> Freeze<F> for Organization {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}