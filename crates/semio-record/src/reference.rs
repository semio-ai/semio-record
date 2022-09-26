use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::record::VersionReq;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Reference<T> {
  Local(T),
  Remote(Uuid),
  RemoteVersion(VersionReq)
}