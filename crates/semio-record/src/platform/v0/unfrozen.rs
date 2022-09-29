use crate::i18n::LocalizedString;

pub struct Number<T> {
  pub default_value: Option<T>,

  pub min_position: Option<T>,
  pub max_position: Option<T>,

  pub min_velocity: Option<T>,
  pub max_velocity: Option<T>,

  pub min_acceleration: Option<T>,
  pub max_acceleration: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum ControlKind {
  Real(Number<f64>),
  Integer(Number<i32>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Control {
  pub name: LocalizedString,
  pub kind: ControlKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlGroup {
  pub name: LocalizedString,
  pub parent_id: Option<Uuid>,
  pub control_ids: HashSet<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Controllable {
  pub name: LocalizedString,
  pub parent_id: Option<Uuid>,
  pub origin: Option<ReferenceFrame>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Platform {
  pub controls: HashMap<Uuid, Control>,
  pub control_groups: HashMap<Uuid, ControlGroup>,
}