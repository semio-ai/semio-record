
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use schemars::JsonSchema;

use super::unfrozen::Folder;

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "folder_v0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
}

#[graphql_object(name = "FolderPublic")]
impl Public {
  fn name(&self) -> &str {
    &self.name
  }

  fn parent(&self) -> &Uuid {
    &self.parent
  }
}

impl From<Folder> for Public {
  fn from(folder: Folder) -> Self {
    Self {
      name: folder.name,
      parent: folder.parent,
    }
  }
}