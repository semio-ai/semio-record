use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::UnfrozenReference;

use super::unfrozen::{Export, Module};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "module_v0_Public", rename_all = "camelCase")]
pub struct Public {
  pub parent: Uuid,
  pub name: String,
  pub exports: HashMap<Uuid, Export>,
  pub executable: Option<Uuid>,
  pub dependencies: Vec<UnfrozenReference>,
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
}

impl From<Module> for Public {
  fn from(module: Module) -> Self {
    Self {
      parent: module.parent,
      name: module.name,
      exports: module
        .exports
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect(),
      executable: module.executable,
      dependencies: module.dependencies,
    }
  }
}

impl From<Public> for Module {
  fn from(public: Public) -> Self {
    Self {
      parent: public.parent,
      name: public.name,
      exports: public
        .exports
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect(),
      executable: public.executable,
      dependencies: public.dependencies,
      ..Default::default()
    }
  }
}