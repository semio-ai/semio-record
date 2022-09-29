use std::collections::{HashSet, HashMap};

use async_trait::async_trait;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  record::{Unfrozen, UnfrozenReference, View, Freeze, Freezer},
  blob::BlobDependencies,
  unfrozen::impl_unfrozen,
  acl::{Acl, action::with_acl},
  action::name,
  unit_math::{Vector3, Vector2, ReferenceFrame},
  unit::{Distance, DistanceKind, Mass},
  math::{Vector3 as RawVector3, Vector2 as RawVector2}, migrate::Migrate,
};

use super::action::Action;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_Box", rename_all = "camelCase")]
pub struct BoxGeometry {
  size: Vector3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_Sphere", rename_all = "camelCase")]
pub struct Sphere {
  radius: Distance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_Cylinder", rename_all = "camelCase")]
pub struct Cylinder {
  radius: Distance,
  height: Distance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_Cone", rename_all = "camelCase")]
pub struct Cone {
  radius: Distance,
  height: Distance,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_Plane", rename_all = "camelCase")]
pub struct Plane {
  size: Vector2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_LocalMesh", rename_all = "camelCase")]
pub struct LocalMesh {
  distance_type: Option<DistanceKind>,
  vertices: Vec<RawVector3>,
  uvs: Option<Vec<RawVector2>>,
  indices: Option<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry_RemoteMesh", rename_all = "camelCase")]
pub struct RemoteMesh {
  blob_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Geometry", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Geometry {
  Box(BoxGeometry),
  Sphere(Sphere),
  Cylinder(Cylinder),
  Cone(Cone),
  Plane(Plane),
  LocalMesh(LocalMesh),
  RemoteMesh(RemoteMesh),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_ColliderKind", rename_all = "camelCase")]
pub enum ColliderKind {
  Box,
  Sphere,
  Cylinder,
  Mesh,
  None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Physics", rename_all = "camelCase")]
pub struct Physics {
  collider_id: Option<Uuid>,
  collider_kind: ColliderKind,
  fixed: Option<bool>,
  mass: Option<Mass>,
  friction: Option<f64>,
  restitution: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Node_Kind_Object", rename_all = "camelCase")]
pub struct Object {
  pub geometry_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Node_Kind_DirectionalLight", rename_all = "camelCase")]
pub struct DirectionalLight {
  radius: Option<Distance>,
  range: Option<Distance>,
  direction: Vector3,
  intensity: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Node_Kind", tag = "type", rename_all = "camelCase", content = "value")]
pub enum NodeKind {
  Empty,
  // `object` is a reserved keyword in JS/TS.
  #[serde(rename = "obj")]
  Object(Object),
  PointLight,
  SpotLight,
  DirectionalLight(DirectionalLight),
  FromTemplate
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Node", rename_all = "camelCase")]
pub struct Node {
  pub name: String,
  pub parent_id: Option<Uuid>,
  pub origin: Option<ReferenceFrame>,
  pub editable: Option<bool>,
  pub visible: Option<bool>,
  pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "scene_v0_Private", rename_all = "camelCase")]
pub struct Scene {
  pub name: String,
  pub description: String,
  pub acl: Acl,
  pub geometry: HashMap<Uuid, Geometry>,
  pub nodes: HashMap<Uuid, Node>
}

impl Default for Scene {
  fn default() -> Self {
    Self {
      name: String::new(),
      description: String::new(),
      acl: Default::default(),
      geometry: HashMap::new(),
      nodes: HashMap::new()
    }
  }
}

impl_unfrozen!(Scene, Action);

impl Unfrozen<Action> for Scene {
  fn dependencies<'a>(&'a self, _set: &mut HashSet<&'a UnfrozenReference>) {}
}

impl BlobDependencies for Scene {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a uuid::Uuid>) {
    for geometry in self.geometry.values() {
      match geometry {
        Geometry::RemoteMesh(mesh) => {
          set.insert(&mesh.blob_id);
        },
        _ => {}
      }
    }
  }
}

with_acl!(Scene);

impl View for Scene {
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

name!(Scene);

#[async_trait]
impl<F: Freezer> Freeze<F> for Scene {
  type Frozen = ();

  async fn freeze(&self, _: &F) -> Result<Self::Frozen, F::Error> {
    Ok(())
  }
}

impl Migrate for Scene {
  fn migrate(from_version: i16, _from: &[u8]) -> anyhow::Result<Self> {
    anyhow::bail!("Migration not implemented for version {}", from_version)
  }
}