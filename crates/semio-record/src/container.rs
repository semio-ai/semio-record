use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{record::{VersionReq, UnfrozenReference, FrozenReference}};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "unfrozen_Container", tag = "type", content = "value", rename_all = "camelCase")]
pub enum UnfrozenContainer<T> {
  Local(T),
  Reference(UnfrozenReference),
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "frozen_Container", tag = "type", content = "value", rename_all = "camelCase")]
pub enum FrozenContainer<T> {
  Local(T),
  Reference(FrozenReference),
}