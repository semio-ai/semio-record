use std::{cmp::Ordering};

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  acl::action::{Action as AclAction, ActionError as AclActionError},
  action::{SetName, SetNameError, SetParent, SetParentError},
  record::Apply, color::Color,
};

use super::unfrozen::{Animation, Node, Key, Value, Transition};

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_AddNode")]
pub struct AddNode {
  pub parent_id: Uuid,
  pub id: Uuid,
  pub node: Node,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_AddNodeError", tag = "type", rename_all = "camelCase")]
pub enum AddNodeError {
  #[display(fmt = "Node already exists")]
  NodeExists,
  #[display(fmt = "Parent node does not exist")]
  ParentNodeDoesNotExist,
  #[display(fmt = "Parent node is not a group")]
  ParentNodeIsNotGroup,
}

impl Apply<AddNode> for Animation {
  type Error = AddNodeError;

  fn apply(&mut self, action: &AddNode) -> Result<(), Self::Error> {
    if self.nodes.contains_key(&action.id) {
      return Err(AddNodeError::NodeExists);
    }

    let parent_node = self
      .nodes
      .get_mut(&action.parent_id)
      .ok_or(AddNodeError::ParentNodeDoesNotExist)?;

    match parent_node {
      Node::Group(g) => {
        g.children_ids.insert(action.id);
      }
      _ => return Err(AddNodeError::ParentNodeIsNotGroup),
    }

    self.nodes.insert(action.id, action.node.clone());

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveNode")]
pub struct RemoveNode {
  pub id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveNodeError", tag = "type", rename_all = "camelCase")]
pub enum RemoveNodeError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<RemoveNode> for Animation {
  type Error = RemoveNodeError;

  fn apply(&mut self, action: &RemoveNode) -> Result<(), Self::Error> {
    if let None = self.nodes.remove(&action.id) {
      return Err(RemoveNodeError::NodeDoesNotExist);
    }

    for node in self.nodes.values_mut() {
      if let Node::Group(g) = node {
        g.children_ids.remove(&action.id);
      }
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_AddKey")]
pub struct AddKey {
  pub node_id: Uuid,
  pub key_id: Uuid,
  pub key: Key,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_AddKeyError", tag = "type", rename_all = "camelCase")]
pub enum AddKeyError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key already exists")]
  KeyExists,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<AddKey> for Animation {
  type Error = AddKeyError;

  fn apply(&mut self, action: &AddKey) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.node_id) {
      match node {
        Node::Track(t) => {
          if t.keys.contains_key(&action.key_id) {
            return Err(AddKeyError::KeyExists);
          }

          t.keys.insert(action.key_id, action.key.clone());

          t.key_ordering.push(action.key_id);
          t.key_ordering.sort_by(|a, b| {
            let a = t.keys.get(a).unwrap();
            let b = t.keys.get(b).unwrap();
            a.at.partial_cmp(&b.at)
              .unwrap_or(std::cmp::Ordering::Equal)
          });
        }
        _ => return Err(AddKeyError::NodeIsNotTrack),
      }
    } else {
      return Err(AddKeyError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveKey")]
pub struct RemoveKey {
  pub node_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveKeyError", tag = "type", rename_all = "camelCase")]
pub enum RemoveKeyError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<RemoveKey> for Animation {
  type Error = RemoveKeyError;

  fn apply(&mut self, action: &RemoveKey) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.node_id) {
      match node {
        Node::Track(t) => {
          if !t.keys.contains_key(&action.key_id) {
            return Err(RemoveKeyError::KeyDoesNotExist);
          }

          t.keys.remove(&action.key_id);
          t.key_ordering.retain(|k| k != &action.key_id);
        }
        _ => return Err(RemoveKeyError::NodeIsNotTrack),
      }
    } else {
      return Err(RemoveKeyError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveKeys")]
pub struct RemoveKeys {
  pub selectors: Vec<KeySelector>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_RemoveKeysError", tag = "type", rename_all = "camelCase")]
pub enum RemoveKeysError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<RemoveKeys> for Animation {
  type Error = RemoveKeysError;

  fn apply(&mut self, action: &RemoveKeys) -> Result<(), Self::Error> {
    // Check for errors first

    for selector in &action.selectors {
      if let Some(node) = self.nodes.get(&selector.node_id) {
        match node {
          Node::Track(t) => {
            if !t.keys.contains_key(&selector.key_id) {
              return Err(RemoveKeysError::KeyDoesNotExist);
            }
          }
          _ => return Err(RemoveKeysError::NodeIsNotTrack),
        }
      } else {
        return Err(RemoveKeysError::NodeDoesNotExist);
      }
    }

    // Apply
    for selector in &action.selectors {
      if let Some(node) = self.nodes.get_mut(&selector.node_id) {
        match node {
          Node::Track(t) => {
            t.keys.remove(&selector.key_id);
            t.key_ordering.retain(|k| k != &selector.key_id);
          },
          _ => unreachable!(),
        }
      }
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeName")]
pub struct SetNodeName {
  pub id: Uuid,
  pub name: String,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeNameError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeNameError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeName> for Animation {
  type Error = SetNodeNameError;

  fn apply(&mut self, action: &SetNodeName) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(g) => g.name = action.name.clone(),
        Node::Track(t) => t.name = action.name.clone(),
      }
    } else {
      return Err(SetNodeNameError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeParent")]
pub struct SetNodeParent {
  pub id: Uuid,
  pub parent_id: Uuid,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeParentError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeParentError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Parent node does not exist")]
  ParentNodeDoesNotExist,
  #[display(fmt = "Parent node is not a group")]
  ParentNodeIsNotGroup,
}

impl Apply<SetNodeParent> for Animation {
  type Error = SetNodeParentError;

  fn apply(&mut self, action: &SetNodeParent) -> Result<(), Self::Error> {
    // Check if node exists
    if let None = self.nodes.get(&action.id) {
      return Err(SetNodeParentError::NodeDoesNotExist);
    }

    // Remove node from old parent
    for node in self.nodes.values_mut() {
      if let Node::Group(g) = node {
        g.children_ids.remove(&action.id);
      }
    }

    // Add node to new parent
    if let Some(parent_node) = self.nodes.get_mut(&action.parent_id) {
      match parent_node {
        Node::Group(g) => {
          g.children_ids.insert(action.id);
        }
        _ => return Err(SetNodeParentError::ParentNodeIsNotGroup),
      }
    } else {
      return Err(SetNodeParentError::ParentNodeDoesNotExist);
    }


    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeColor")]
pub struct SetNodeColor {
  pub id: Uuid,
  pub color: Color,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeColorError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeColorError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeColor> for Animation {
  type Error = SetNodeColorError;

  fn apply(&mut self, action: &SetNodeColor) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(g) => g.color = action.color.clone(),
        Node::Track(t) => t.color = action.color.clone(),
      }
    } else {
      return Err(SetNodeColorError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeLocked")]
pub struct SetNodeLocked {
  pub id: Uuid,
  pub locked: bool,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeLockedError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeLockedError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeLocked> for Animation {
  type Error = SetNodeLockedError;

  fn apply(&mut self, action: &SetNodeLocked) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(g) => g.locked = action.locked,
        Node::Track(t) => t.locked = action.locked,
      }
    } else {
      return Err(SetNodeLockedError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeCollapsed")]
pub struct SetNodeCollapsed {
  pub id: Uuid,
  pub collapsed: bool,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetNodeCollapsedError", tag = "type", rename_all = "camelCase")]
pub enum SetNodeCollapsedError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
}

impl Apply<SetNodeCollapsed> for Animation {
  type Error = SetNodeCollapsedError;

  fn apply(&mut self, action: &SetNodeCollapsed) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(g) => g.collapsed = action.collapsed,
        Node::Track(t) => t.collapsed = action.collapsed,
      }
    } else {
      return Err(SetNodeCollapsedError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyAt")]
pub struct SetKeyAt {
  pub id: Uuid,
  pub key_id: Uuid,
  pub at: f64,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyAtError", tag = "type", rename_all = "camelCase")]
pub enum SetKeyAtError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<SetKeyAt> for Animation {
  type Error = SetKeyAtError;

  fn apply(&mut self, action: &SetKeyAt) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(_) => {
          return Err(SetKeyAtError::NodeIsNotTrack);
        }
        Node::Track(t) => {
          if let Some(key) = t.keys.get_mut(&action.key_id) {
            key.at = action.at;
            // sort key ordering
            t.key_ordering.sort_by(|a, b| {
              let a = t.keys.get(a).unwrap();
              let b = t.keys.get(b).unwrap();
              a.at.partial_cmp(&b.at)
                .unwrap_or(Ordering::Equal)
            });
          } else {
            return Err(SetKeyAtError::KeyDoesNotExist);
          }
        }
      }
    } else {
      return Err(SetKeyAtError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_KeySelector", rename_all = "camelCase")]
pub struct KeySelector {
  pub node_id: Uuid,
  pub key_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_KeySelectorAt")]
pub struct KeySelectorAt {
  pub selector: KeySelector,
  pub at: f64,
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeysAt")]
pub struct SetKeysAt {
  pub ats: Vec<KeySelectorAt>,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeysAtError", tag = "type", rename_all = "camelCase")]
pub enum SetKeysAtError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<SetKeysAt> for Animation {
  type Error = SetKeysAtError;

  fn apply(&mut self, action: &SetKeysAt) -> Result<(), Self::Error> {
    // Validate all key selectors before mutating
    for selector in &action.ats {
      if let Some(node) = self.nodes.get(&selector.selector.node_id) {
        match node {
          Node::Group(_) => {
            return Err(SetKeysAtError::NodeIsNotTrack);
          }
          Node::Track(t) => {
            if let Some(key) = t.keys.get(&selector.selector.key_id) {
              // do nothing
            } else {
              return Err(SetKeysAtError::KeyDoesNotExist);
            }
          }
        }
      } else {
        return Err(SetKeysAtError::NodeDoesNotExist);
      }
    }

    for at in &action.ats {
      if let Some(node) = self.nodes.get_mut(&at.selector.node_id) {
        match node {
          Node::Group(_) => unreachable!(),
          Node::Track(t) => {
            if let Some(key) = t.keys.get_mut(&at.selector.key_id) {
              key.at = at.at;
              // sort key ordering
              t.key_ordering.sort_by(|a, b| {
                let a = t.keys.get(a).unwrap();
                let b = t.keys.get(b).unwrap();
                a.at.partial_cmp(&b.at)
                  .unwrap_or(Ordering::Equal)
              });
            }
          }
        }
      }
    }

    Ok(())
  }
}



#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyValue", rename_all = "camelCase")]
pub struct SetKeyValue {
  pub id: Uuid,
  pub key_id: Uuid,
  pub value: Value,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyValueError", tag = "type", rename_all = "camelCase")]
pub enum SetKeyValueError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<SetKeyValue> for Animation {
  type Error = SetKeyValueError;

  fn apply(&mut self, action: &SetKeyValue) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(_) => {
          return Err(SetKeyValueError::NodeIsNotTrack);
        }
        Node::Track(t) => {
          if let Some(key) = t.keys.get_mut(&action.key_id) {
            key.value = action.value.clone();
          } else {
            return Err(SetKeyValueError::KeyDoesNotExist);
          }
        }
      }
    } else {
      return Err(SetKeyValueError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyTransition", rename_all = "camelCase")]
pub struct SetKeyTransition {
  pub id: Uuid,
  pub key_id: Uuid,
  pub transition: Transition,
}

#[derive(Display, Debug, Serialize, Deserialize, Error, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action_SetKeyTransitionError", tag = "type", rename_all = "camelCase")]
pub enum SetKeyTransitionError {
  #[display(fmt = "Node does not exist")]
  NodeDoesNotExist,
  #[display(fmt = "Key does not exist")]
  KeyDoesNotExist,
  #[display(fmt = "Node is not a track")]
  NodeIsNotTrack,
}

impl Apply<SetKeyTransition> for Animation {
  type Error = SetKeyTransitionError;

  fn apply(&mut self, action: &SetKeyTransition) -> Result<(), Self::Error> {
    if let Some(node) = self.nodes.get_mut(&action.id) {
      match node {
        Node::Group(_) => {
          return Err(SetKeyTransitionError::NodeIsNotTrack);
        }
        Node::Track(t) => {
          if let Some(key) = t.keys.get_mut(&action.key_id) {
            key.transition = action.transition.clone();
          } else {
            return Err(SetKeyTransitionError::KeyDoesNotExist);
          }
        }
      }
    } else {
      return Err(SetKeyTransitionError::NodeDoesNotExist);
    }

    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, From, Clone)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_Action", tag = "type", rename_all = "camelCase", content = "value")]
pub enum Action {
  SetName(SetName),
  SetParent(SetParent),
  AddNode(AddNode),
  RemoveNode(RemoveNode),
  AddKey(AddKey),
  RemoveKey(RemoveKey),
  RemoveKeys(RemoveKeys),
  SetNodeName(SetNodeName),
  SetNodeParent(SetNodeParent),
  SetNodeColor(SetNodeColor),
  SetNodeLocked(SetNodeLocked),
  SetNodeCollapsed(SetNodeCollapsed),
  SetKeyAt(SetKeyAt),
  SetKeysAt(SetKeysAt),
  SetKeyValue(SetKeyValue),
  SetKeyTransition(SetKeyTransition),
  Acl(AclAction),
}

#[derive(Display, Debug, Error, From, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "animation_V0_ActionError", tag = "type", rename_all = "camelCase", content = "value")]
pub enum ActionError {
  SetName(SetNameError),
  SetParent(SetParentError),
  AddNode(AddNodeError),
  RemoveNode(RemoveNodeError),
  AddKey(AddKeyError),
  RemoveKey(RemoveKeyError),
  RemoveKeys(RemoveKeysError),
  SetNodeName(SetNodeNameError),
  SetNodeParent(SetNodeParentError),
  SetNodeColor(SetNodeColorError),
  SetNodeLocked(SetNodeLockedError),
  SetNodeCollapsed(SetNodeCollapsedError),
  SetKeyAt(SetKeyAtError),
  SetKeysAt(SetKeysAtError),
  SetKeyValue(SetKeyValueError),
  SetKeyTransition(SetKeyTransitionError),
  Acl(AclActionError),
}

impl Apply<Action> for Animation {
  type Error = ActionError;

  fn apply(&mut self, action: &Action) -> Result<(), Self::Error> {
    match action {
      Action::SetName(action) => self.apply(action)?,
      Action::SetParent(action) => self.apply(action)?,
      Action::AddNode(action) => self.apply(action)?,
      Action::RemoveNode(action) => self.apply(action)?,
      Action::AddKey(action) => self.apply(action)?,
      Action::RemoveKey(action) => self.apply(action)?,
      Action::RemoveKeys(action) => self.apply(action)?,
      Action::SetNodeName(action) => self.apply(action)?,
      Action::SetNodeParent(action) => self.apply(action)?,
      Action::SetNodeColor(action) => self.apply(action)?,
      Action::SetNodeLocked(action) => self.apply(action)?,
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
