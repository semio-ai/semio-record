pub mod action;

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  blob::BlobDependencies,
  entity::{Entity, Reference},
  id::Id,
  version::{ValidatedSemanticVersion, ValidationResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
  pub name: String,
  pub type_ref: Reference,
  pub mutable: bool,
}

impl Parameter {
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    set.insert(&self.type_ref);
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
  pub parameters: HashMap<Uuid, Parameter>,
  pub parameter_ordering: Vec<Uuid>,
  pub return_type_ref: Reference,
}

impl Function {
  pub fn contains_parameter(&self, id: &Id) -> bool {
    match id {
      Id::Uuid(id) => self.parameters.contains_key(id),
      Id::Name(name) => self
        .parameters
        .iter()
        .find(|(_, export)| export.name == *name)
        .is_some(),
    }
  }

  pub fn get_parameter_id(&self, id: &Id) -> Option<Uuid> {
    match id {
      Id::Uuid(id) => {
        if self.parameters.contains_key(id) {
          Some(id.clone())
        } else {
          None
        }
      }
      Id::Name(name) => self
        .parameters
        .iter()
        .find(|(_, parameter)| parameter.name == *name)
        .map(|(id, _)| id.clone()),
    }
  }

  pub fn get_parameter(&self, id: &Id) -> Option<&Parameter> {
    self.parameters.get(&self.get_parameter_id(id)?)
  }

  pub fn get_parameter_mut(&mut self, id: &Id) -> Option<&mut Parameter> {
    self.parameters.get_mut(&self.get_parameter_id(id)?)
  }

  pub fn remove_parameter(&mut self, id: &Id) -> Option<Parameter> {
    let id = self.get_parameter_id(id)?;
    let ret = self.parameters.remove(&id);
    self.parameter_ordering.retain(|pid| *pid != id);
    ret
  }

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
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

  pub fn as_function_mut(&mut self) -> Option<&mut Function> {
    match self {
      ExportKind::Function(function) => Some(function),
    }
  }

  pub fn to_function(self) -> Function {
    match self {
      ExportKind::Function(function) => function,
    }
  }

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
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
  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    self.kind.dependencies(set)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
}

impl Module {
  pub fn contains_export(&self, id: &Id) -> bool {
    match id {
      Id::Uuid(id) => self.exports.contains_key(id),
      Id::Name(name) => self
        .exports
        .iter()
        .find(|(_, export)| export.name == *name)
        .is_some(),
    }
  }

  pub fn get_export_id(&self, id: &Id) -> Option<Uuid> {
    match id {
      Id::Uuid(id) => {
        if self.exports.contains_key(id) {
          Some(id.clone())
        } else {
          None
        }
      }
      Id::Name(name) => Some(
        self
          .exports
          .iter()
          .find(|(_, export)| export.name == *name)?
          .0
          .clone(),
      ),
    }
  }

  pub fn get_export(&self, id: &Id) -> Option<&Export> {
    self.exports.get(&self.get_export_id(id)?)
  }

  pub fn get_export_mut(&mut self, id: &Id) -> Option<&mut Export> {
    self.exports.get_mut(&self.get_export_id(id)?)
  }
}

impl Default for Module {
  fn default() -> Self {
    Self {
      exports: HashMap::new(),
      executable: None,
    }
  }
}

impl Entity for Module {
  const NAME: &'static str = "module";

  fn dependencies<'a>(&'a self, set: &mut HashSet<&'a Reference>) {
    for export in self.exports.values() {
      export.dependencies(set);
    }
  }
}

impl BlobDependencies for Module {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>) {
    if let Some(executable) = &self.executable {
      set.insert(&executable);
    }
  }
}

impl ValidatedSemanticVersion for Module {
  fn validate(&self, _: &Self) -> ValidationResult {
    ValidationResult::Valid
  }
}
