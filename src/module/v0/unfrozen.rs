use std::{collections::{HashMap, HashSet}, error::Error};

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{UnfrozenReference, Freeze, Freezer, View, Unfrozen, Apply}, blob::BlobDependencies, unfrozen::impl_unfrozen};

use super::{frozen, action::Action};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
  pub name: String,
  pub type_ref: UnfrozenReference,
  pub mutable: bool,
}

impl Parameter {
  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    set.insert(&self.type_ref);
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Parameter {
  type Frozen = frozen::Parameter;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    Ok(Self::Frozen {
      name: self.name.clone(),
      type_ref: freezer.freeze(&self.type_ref).await?,
      mutable: self.mutable,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_type_ref: UnfrozenReference,
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

  pub fn parameter_mut(&mut self, id: &Uuid) -> Option<&mut Parameter> {
    self.parameters.get_mut(id)
  }

  pub fn parameter_named(&self, name: &str) -> Option<&Parameter> {
    self.parameter(self.parameter_id(name)?)
  }

  pub fn parameter_named_mut(&mut self, name: &str) -> Option<&mut Parameter> {
    self.parameter_mut(&self.parameter_id(name)?.clone())
  }

  pub fn remove_parameter(&mut self, id: &Uuid) -> Option<Parameter> {
    self.parameter_ordering.retain(|&parameter_id| parameter_id != *id);
    self.parameters.remove(id)
  }

  pub fn remove_parameter_named(&mut self, name: &str) -> Option<Parameter> {
    self.remove_parameter(&self.parameter_id(name)?.clone())
  }

  pub fn append_parameter(&mut self, parameter: Parameter) -> Uuid {
    let id = Uuid::new_v4();
    self.parameters.insert(id.clone(), parameter);
    self.parameter_ordering.push(id.clone());
    id
  }

  pub fn insert_parameter(&mut self, index: usize, parameter: Parameter) -> Uuid {
    let id = Uuid::new_v4();
    self.parameters.insert(id.clone(), parameter);
    self.parameter_ordering.insert(index, id.clone());
    id
  }

  pub fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for (_, parameter) in &self.parameters {
      parameter.dependencies(set);
    }
    set.insert(&self.return_type_ref);
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Function {
  type Frozen = frozen::Function;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut parameters = HashMap::with_capacity(self.parameters.len());
    for (id, parameter) in &self.parameters {
      parameters.insert(id.clone(), parameter.freeze(freezer).await?);
    }

    Ok(Self::Frozen {
      parameters,
      parameter_ordering: self.parameter_ordering.clone(),
      return_type_ref: freezer.freeze(&self.return_type_ref).await?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ExportKind {
  Function(Function),
}

impl ExportKind {
  pub fn as_function(&self) -> Option<&Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn as_function_mut(&mut self) -> Option<&mut Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn to_function(self) -> Option<Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    match self {
      Self::Function(function) => function.dependencies(set),
    }
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for ExportKind {
  type Frozen = frozen::ExportKind;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    match self {
      Self::Function(function) => Ok(Self::Frozen::Function(function.freeze(freezer).await?)),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Export {
  pub name: String,
  pub kind: ExportKind,
}

impl Export {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    self.kind.dependencies(set)
  }
}

#[async_trait]
impl<F: Freezer> Freeze<F> for Export {
  type Frozen = frozen::Export;

  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let kind = self.kind.freeze(freezer).await?;
    Ok(Self::Frozen {
      name: self.name.clone(),
      kind,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
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

impl_unfrozen!(Module, Action);

impl Default for Module {
  fn default() -> Self {
    Self {
      parent: Uuid::nil(),
      name: String::new(),
      exports: HashMap::new(),
      executable: None,
    }
  }
}

impl Unfrozen<Action> for Module {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a UnfrozenReference>) {
    for (_, export) in &self.exports {
      export.dependencies(set);
    }
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

#[async_trait]
impl<F: Freezer> Freeze<F> for Module {
  type Frozen = frozen::Module;
  
  async fn freeze(&self, freezer: &F) -> Result<Self::Frozen, F::Error> {
    let mut exports = HashMap::with_capacity(self.exports.len());
    for (id, export) in &self.exports {
      exports.insert(id.clone(), export.freeze(freezer).await?);
    }

    Ok(Self::Frozen {
      parent: self.parent,
      name: self.name.clone(),
      exports,
      executable: self.executable.clone(),
    })
  }
}