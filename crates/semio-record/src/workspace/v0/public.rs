use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::acl::Acl;

use super::unfrozen::{Context, Workspace};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "workspace_V0_Public", rename_all = "camelCase")]
pub struct Public {
  pub parent: Uuid,
  pub name: String,

  pub active_context: Option<Uuid>,
  pub context_ordering: Vec<Uuid>,
  pub contexts: HashMap<Uuid, Context>,
}

impl From<Workspace> for Public {
  fn from(workspace: Workspace) -> Self {
    Self {
      parent: workspace.parent,
      name: workspace.name,
      active_context: workspace.active_context,
      context_ordering: workspace.context_ordering,
      contexts: workspace.contexts
    }
  }
}

impl From<Public> for Workspace {
  fn from(public: Public) -> Self {
    Self {
      parent: public.parent,
      acl: Default::default(),
      name: public.name,
      active_context: public.active_context,
      context_ordering: public.context_ordering,
      contexts: public.contexts,
    }
  }
}