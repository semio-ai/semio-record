use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, From, Clone, Hash, PartialEq, Eq, JsonSchema)]
pub struct KeySelector {
  pub control_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
pub struct KeySelectorAt {
  pub selector: KeySelector,
  pub at: f64,
}

pub mod add_control;
pub mod add_key;
pub mod add_node;
pub mod remove_control;
pub mod remove_key;
pub mod remove_keys;
pub mod remove_node;
pub mod set_control_locked;
pub mod set_control_name;
pub mod set_key_at;
pub mod set_key_transition;
pub mod set_key_value;
pub mod set_keys_at;
pub mod set_node_collapsed;
pub mod set_node_name;
pub mod set_node_parent;

use crate::{
  acl::action::{Action as AclAction, ActionError as AclActionError},
  action::{SetName, SetNameError, SetParent, SetParentError},
  record::Apply
};

use super::unfrozen::Animation;


#[derive(Debug, Serialize, Deserialize, From, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum Action {
  SetName(SetName),
  SetParent(SetParent),
  AddControl(add_control::AddControl),
  AddNode(add_node::AddNode),
  RemoveControl(remove_control::RemoveControl),
  RemoveNode(remove_node::RemoveNode),
  AddKey(add_key::AddKey),
  RemoveKey(remove_key::RemoveKey),
  RemoveKeys(remove_keys::RemoveKeys),
  SetControlName(set_control_name::SetControlName),
  SetNodeName(set_node_name::SetNodeName),
  SetNodeParent(set_node_parent::SetNodeParent),
  SetControlLocked(set_control_locked::SetControlLocked),
  SetNodeCollapsed(set_node_collapsed::SetNodeCollapsed),
  SetKeyAt(set_key_at::SetKeyAt),
  SetKeysAt(set_keys_at::SetKeysAt),
  SetKeyValue(set_key_value::SetKeyValue),
  SetKeyTransition(set_key_transition::SetKeyTransition),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", content = "value")]
pub enum ActionError {
  SetName(SetNameError),
  SetParent(SetParentError),
  AddControl(add_control::AddControlError),
  AddNode(add_node::AddNodeError),
  RemoveControl(remove_control::RemoveControlError),
  RemoveNode(remove_node::RemoveNodeError),
  AddKey(add_key::AddKeyError),
  RemoveKey(remove_key::RemoveKeyError),
  RemoveKeys(remove_keys::RemoveKeysError),
  SetControlName(set_control_name::SetControlNameError),
  SetNodeName(set_node_name::SetNodeNameError),
  SetNodeParent(set_node_parent::SetNodeParentError),
  SetControlLocked(set_control_locked::SetControlLockedError),
  SetNodeCollapsed(set_node_collapsed::SetNodeCollapsedError),
  SetKeyAt(set_key_at::SetKeyAtError),
  SetKeysAt(set_keys_at::SetKeysAtError),
  SetKeyValue(set_key_value::SetKeyValueError),
  SetKeyTransition(set_key_transition::SetKeyTransitionError),
  Acl(AclActionError),
}

impl Apply<Action> for Animation {
  type Error = ActionError;

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::SetParent(action) => self.apply(action)?,
      Action::AddControl(action) => self.apply(action)?,
      Action::AddNode(action) => self.apply(action)?,
      Action::RemoveControl(action) => self.apply(action)?,
      Action::RemoveNode(action) => self.apply(action)?,
      Action::AddKey(action) => self.apply(action)?,
      Action::RemoveKey(action) => self.apply(action)?,
      Action::RemoveKeys(action) => self.apply(action)?,
      Action::SetControlName(action) => self.apply(action)?,
      Action::SetNodeName(action) => self.apply(action)?,
      Action::SetNodeParent(action) => self.apply(action)?,
      Action::SetControlLocked(action) => self.apply(action)?,
      Action::SetNodeCollapsed(action) => self.apply(action)?,
      Action::SetKeyAt(action) => self.apply(action)?,
      Action::SetKeysAt(action) => self.apply(action)?,
      Action::SetKeyValue(action) => self.apply(action)?,
      Action::SetKeyTransition(action) => self.apply(action)?,
      Action::Acl(action) => self.apply(action)?,
    }

    Ok(())
  }
}
