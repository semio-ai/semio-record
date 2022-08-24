use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::record::Apply;

use super::super::unfrozen::Animation;

#[derive(Debug, Serialize, Deserialize, From, Clone)]
pub struct SetNodeCollapsed {
  pub id: Uuid,
  pub collapsed: bool,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum SetNodeCollapsedError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeCollapsed> for Animation {
  type Error = SetNodeCollapsedError;

  fn apply(&mut self, action: &SetNodeCollapsed) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      node.collapsed = action.collapsed
    } else {
      return Err(SetNodeCollapsedError::NodeDoesNotExist);
    }

    Ok(())
  }
}