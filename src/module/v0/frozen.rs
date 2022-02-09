use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{record::{FrozenReference, Frozen, View}, blob::BlobDependencies};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
  pub name: String,
  pub type_ref: FrozenReference,
  pub mutable: bool,
}

impl Parameter {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    set.insert(&self.type_ref);
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_type_ref: FrozenReference,
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

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    for (_, parameter) in &self.parameters {
      parameter.dependencies(set);
    }
    set.insert(&self.return_type_ref);
  }
}

#[derive(Debug, Serialize, Deserialize)]
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

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a FrozenReference>) {
    match self {
      Self::Function(function) => function.dependencies(set),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
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
  }
}