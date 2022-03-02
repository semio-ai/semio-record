use std::{collections::{HashMap, HashSet}};

use async_trait::async_trait;
use juniper::{GraphQLUnion, GraphQLObject, FromInputValue, ScalarValue, InputValue, marker::IsInputType};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{UnfrozenReference, Freeze, Freezer, View, Unfrozen}, blob::BlobDependencies, unfrozen::impl_unfrozen, action::{name, parent}, ty::UnfrozenTy, acl::{Acl, action::with_acl}};

use super::{frozen, action::Action};

use derive_more::From;

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Parameter {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: UnfrozenTy,
  pub mutable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_ty: UnfrozenTy,
}

#[derive(GraphQLObject)]
pub struct IdParameter {
  pub id: Uuid,
  pub parameter: Parameter,
}

impl From<(Uuid, Parameter)> for IdParameter {
  fn from(pair: (Uuid, Parameter)) -> Self {
    Self {
      id: pair.0,
      parameter: pair.1,
    }
  }
}

#[graphql_object]
impl Function {
  #[graphql(name = "parameters")]
  pub fn gql_parameters(&self) -> Vec<IdParameter> {
    self.parameters.iter().map(|(k, v)| (k.clone(), v.clone()).into()).collect()
  }

  #[graphql(name = "parameterId")]
  pub fn gql_parameter_id(&self, id: String) -> Option<Uuid> {
    self.parameter_id(&id).map(Clone::clone)
  }

  #[graphql(name = "hasParameter")]
  pub fn gql_has_parameter(&self, id: Uuid) -> bool {
    self.has_parameter(&id)
  }

  #[graphql(name = "hasParameterNamed")]
  pub fn gql_has_parameter_named(&self, name: String) -> bool {
    self.has_parameter_named(&name)
  }

  #[graphql(name = "parameter")]
  pub fn gql_parameter(&self, id: Uuid) -> Option<&Parameter> {
    self.parameter(&id)
  }

  #[graphql(name = "parameterNamed")]
  pub fn gql_parameter_named(&self, name: String) -> Option<&Parameter> {
    self.parameter_named(&name)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLUnion, From)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ExportKind {
  Function(Function),
}

#[derive(Debug, Serialize, Deserialize, Clone, GraphQLObject)]
pub struct Export {
  pub name: String,
  pub kind: ExportKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Public {
  pub parent: Uuid,
  pub name: String,
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
}

impl Module {
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
impl Module {
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