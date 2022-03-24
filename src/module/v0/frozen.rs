use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  blob::BlobDependencies,
  record::{Frozen, FrozenReference, View},
  ty::FrozenTy,
};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "FrozenParameter")]
pub struct Parameter {
  pub name: String,
  #[graphql(name = "type")]
  pub ty: FrozenTy,
  pub mutable: bool,
}

impl Parameter {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    self.ty.dependencies(set);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_ty: FrozenTy,
}

impl Function {
  pub fn parameter_id(&self, name: &str) -> Option<&Uuid> {
    self
      .parameters
      .iter()
      .find(|(_, parameter)| parameter.name == *name)
      .map(|(id, _)| id)
  }

  pub fn has_parameter(&self, id: &Uuid) -> bool {
    self.parameters.contains_key(id)
  }

  pub fn has_parameter_named(&self, name: &str) -> bool {
    self.parameter_id(name).is_some()
  }

  pub fn parameter(&self, id: &Uuid) -> Option<&Parameter> {
    self.parameters.get(id)
  }

  pub fn parameter_named(&self, name: &str) -> Option<&Parameter> {
    self.parameter(self.parameter_id(name)?)
  }

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, parameter) in &self.parameters {
      parameter.dependencies(set);
    }
    self.return_ty.dependencies(set);
  }
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(name = "FrozenIdParameter")]
pub struct IdParameter {
  pub id: Uuid,
  pub parameter: Parameter,
}

#[graphql_object(name = "FrozenFunction")]
impl Function {
  #[graphql(name = "parameterId")]
  pub fn gql_parameter_id(&self, name: String) -> Option<Uuid> {
    self.parameter_id(&name).cloned()
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
  pub fn gql_parameter(&self, id: Uuid) -> Option<Parameter> {
    self.parameter(&id).cloned()
  }

  #[graphql(name = "parameterNamed")]
  pub fn gql_parameter_named(&self, name: String) -> Option<Parameter> {
    self.parameter_named(&name).cloned()
  }

  #[graphql(name = "parameters")]
  pub fn gql_parameters(&self) -> Vec<IdParameter> {
    self
      .parameter_ordering
      .iter()
      .filter_map(|id| {
        if let Some(parameter) = self.parameters.get(id) {
          Some(IdParameter {
            id: id.clone(),
            parameter: parameter.clone(),
          })
        } else {
          None
        }
      })
      .collect()
  }

  #[graphql(name = "returnType")]
  pub fn gql_return_type(&self) -> FrozenTy {
    self.return_ty.clone()
  }
}

#[derive(Debug, GraphQLUnion, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
#[graphql(name = "FrozenExportKind")]
pub enum ExportKind {
  Function(Function),
}

impl ExportKind {
  pub fn as_function(&self) -> Option<&Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::Function(function) => function.dependencies(set),
    }
  }
}

#[derive(Debug, Clone, GraphQLObject, Serialize, Deserialize)]
#[graphql(name = "FrozenExport")]
pub struct Export {
  pub name: String,
  pub kind: ExportKind,
}

impl Export {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    self.kind.dependencies(set)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
  pub parent: Uuid,
  pub name: String,
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
  pub dependencies: Vec<FrozenReference>,
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
    self.exports.get(self.export_id(name)?)
  }
}

#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
#[graphql(name = "FrozenIdExport")]
pub struct IdExport {
  pub id: Uuid,
  pub export: Export,
}

#[graphql_object(name = "FrozenModule")]
impl Module {
  #[graphql(name = "exportId")]
  pub fn gql_export_id(&self, name: String) -> Option<Uuid> {
    self.export_id(&name).cloned()
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
  pub fn gql_export(&self, id: Uuid) -> Option<Export> {
    self.export(&id).cloned()
  }

  #[graphql(name = "exportNamed")]
  pub fn gql_export_named(&self, name: String) -> Option<Export> {
    self.export_named(&name).cloned()
  }

  #[graphql(name = "parent")]
  pub fn gql_parent(&self) -> Option<Uuid> {
    Some(self.parent)
  }

  #[graphql(name = "name")]
  pub fn gql_name(&self) -> String {
    self.name.clone()
  }

  #[graphql(name = "exports")]
  pub fn gql_exports(&self) -> Vec<IdExport> {
    self
      .exports
      .iter()
      .map(|(id, export)| IdExport {
        id: id.clone(),
        export: export.clone(),
      })
      .collect()
  }

  #[graphql(name = "executable")]
  pub fn gql_executable(&self) -> Option<Uuid> {
    self.executable.clone()
  }

  #[graphql(name = "dependencies")]
  pub fn gql_dependencies(&self) -> &Vec<FrozenReference> {
    &self.dependencies
  }
}

impl View for Module {
  fn name<'a>(&'a self) -> Option<&'a str> {
    Some(&self.name)
  }

  fn parent<'a>(&'a self) -> Option<&'a Uuid> {
    Some(&self.parent)
  }
}

impl BlobDependencies for Module {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    if let Some(executable) = &self.executable {
      set.insert(&executable);
    }
  }
}

impl Frozen for Module {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, export) in &self.exports {
      export.dependencies(set);
    }

    for dependency in &self.dependencies {
      set.insert(dependency);
    }
  }
}
