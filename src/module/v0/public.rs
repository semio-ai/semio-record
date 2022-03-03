use std::{collections::{HashMap, HashSet}};

use async_trait::async_trait;
use juniper::{GraphQLUnion, GraphQLObject, FromInputValue, ScalarValue, InputValue, marker::IsInputType};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{UnfrozenReference, Freeze, Freezer, View, Unfrozen}, blob::BlobDependencies, unfrozen::impl_unfrozen, action::{name, parent}, ty::UnfrozenTy, acl::{Acl, action::with_acl}};

use super::{frozen, action::Action, unfrozen::{Module, Export}};

use derive_more::From;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Public {
  pub parent: Uuid,
  pub name: String,
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
}

impl Public {
  pub fn export_id(&self, name: &str) -> Option<&Uuid> {
    self
      .exports
      .iter()
      .find(|(_, export)| export.name == *name)
      .map(|(id, _)| id)
  }
  
  pub fn has_export(&self, id: &Uuid) -> bool {
    self.exports.contains_key(id)
  }

  pub fn has_export_named(&self, name: &str) -> bool {
    self.export_id(name).is_some()
  }

  pub fn export(&self, id: &Uuid) -> Option<&Export> {
    self.exports.get(id)
  }

  pub fn export_named(&self, name: &str) -> Option<&Export> {
    self.exports.get(&self.export_id(name)?.clone())
  }

  pub fn export_mut(&mut self, id: &Uuid) -> Option<&mut Export> {
    self.exports.get_mut(id)
  }

  pub fn export_named_mut(&mut self, name: &str) -> Option<&mut Export> {
    self.exports.get_mut(&self.export_id(name)?.clone())
  }

  pub fn remove_export(&mut self, id: &Uuid) -> Option<Export> {
    self.exports.remove(id)
  }

  pub fn remove_export_named(&mut self, name: &str) -> Option<Export> {
    self.remove_export(&self.export_id(name)?.clone())
  }

  pub fn add_export(&mut self, export: Export) -> Uuid {
    let id = Uuid::new_v4();
    self.exports.insert(id.clone(), export);
    id
  }
}

#[derive(GraphQLObject)]
pub struct IdExport {
  pub id: Uuid,
  pub export: Export,
}

impl From<(Uuid, Export)> for IdExport {
  fn from(pair: (Uuid, Export)) -> Self {
    Self {
      id: pair.0,
      export: pair.1,
    }
  }
}

#[graphql_object(name = "ModulePublic")]
impl Public {
  #[graphql(name = "parent")]
  pub fn gql_parent(&self) -> Uuid {
    self.parent.clone()
  }

  #[graphql(name = "name")]
  pub fn gql_name(&self) -> String {
    self.name.clone()
  }

  #[graphql(name = "exports")]
  pub fn gql_exports(&self) -> Vec<IdExport> {
    self.exports.iter().map(|(k, v)| (k.clone(), v.clone()).into()).collect()
  }

  #[graphql(name = "executable")]
  pub fn gql_executable(&self) -> Option<Uuid> {
    self.executable.clone()
  }

  #[graphql(name = "exportId")]
  pub fn gql_export_id(&self, name: String) -> Option<Uuid> {
    self.export_id(&name).map(Clone::clone)
  }

  #[graphql(name = "hasExport")]
  pub fn gql_has_export(&self, id: Uuid) -> bool {
    self.has_export(&id)
  }

  #[graphql(name = "hasExportNamed")]
  pub fn gql_has_export_named(&self, name: String) -> bool {
    self.has_export_named(&name)
  }

  #[graphql(name = "export")]
  pub fn gql_export(&self, id: Uuid) -> Option<&Export> {
    self.export(&id)
  }

  #[graphql(name = "exportNamed")]
  pub fn gql_export_named(&self, name: String) -> Option<&Export> {
    self.export_named(&name)
  }
}

impl From<Module> for Public {
  fn from(module: Module) -> Self {
    Self {
      parent: module.parent,
      name: module.name,
      exports: module.exports.into_iter().map(|(k, v)| (k, v.into())).collect(),
      executable: module.executable,
    }
  }
}