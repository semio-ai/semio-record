use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  blob::BlobDependencies,
  record::{Frozen, FrozenReference, View},
  ty::FrozenTy,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_V0_Frozen_Parameter", rename_all = "camelCase")]
pub struct Parameter {
  pub name: String,
  #[serde(rename = "type")]
  pub ty: FrozenTy,
  pub mutable: bool,
}

impl Parameter {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    self.ty.dependencies(set);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_V0_Frozen_Function", rename_all = "camelCase")]
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

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, parameter) in &self.parameters {
      parameter.dependencies(set);
    }
    self.return_ty.dependencies(set);
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_V0_Frozen_Export_Kind", tag = "type", rename_all = "camelCase", content = "value")]
pub enum ExportKind {
  // `function` is a reserved keyword in JS/TS.
  #[serde(rename = "func")]
  Function(Function),
}

impl ExportKind {
  pub fn as_function(&self) -> Option<&Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::Function(function) => function.dependencies(set),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_V0_Frozen_Export", rename_all = "camelCase")]
pub struct Export {
  pub name: String,
  pub kind: ExportKind,
}

impl Export {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    self.kind.dependencies(set)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_V0_Frozen", rename_all = "camelCase")]
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
