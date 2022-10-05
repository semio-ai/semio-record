
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::unfrozen::Folder;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "folder_V0_Public")]
pub struct Public {
  pub name: String,
  pub parent: Uuid,
}

impl From<Folder> for Public {
  fn from(folder: Folder) -> Self {
    Self {
      name: folder.name,
      parent: folder.parent,
    }
  }
}

impl From<Public> for Folder {
  fn from(public: Public) -> Self {
    Self {
      name: public.name,
      parent: public.parent,
      ..Default::default()
    }
  }
}