pub mod v0;

pub use v0 as latest;

use crate::record::impl_record;

impl_record!(
  0 => v0
);

#[cfg(test)]
mod tests {
  use indexmap::IndexMap;
  use uuid::Uuid;

  use crate::{
    acl::Acl,
    ty::{FrozenTy, Primitive, PrimitiveKind, UnfrozenTy},
  };

  use super::v0::{
    frozen::{Structure as StructureFrozen, StructureField as StructureFieldFrozen},
    unfrozen::{Structure, StructureField},
  };

  fn u64_field(name: &str) -> StructureField {
    StructureField {
      name: name.to_string(),
      ty: UnfrozenTy::Primitive(Primitive { kind: PrimitiveKind::U64 }),
    }
  }

  fn u64_field_frozen(name: &str) -> StructureFieldFrozen {
    StructureFieldFrozen {
      name: name.to_string(),
      ty: FrozenTy::Primitive(Primitive { kind: PrimitiveKind::U64 }),
    }
  }

  #[test]
  fn test_serde_yaml_unfrozen() {
    let ids = [Uuid::new_v4(), Uuid::new_v4()];
    let structure = Structure {
      name: "TickId".to_string(),
      parent: Uuid::new_v4(),
      fields: IndexMap::from_iter([
        (ids[0], u64_field("callable_id")),
        (ids[1], u64_field("sequence")),
      ]),
      acl: <Acl as Default>::default(),
    };

    let yaml = serde_yaml::to_string(&structure).unwrap();
    println!("YAML: {}", yaml);
    let deserialized: Structure = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(structure, deserialized);
  }

  #[test]
  fn test_serde_yaml_frozen() {
    let ids = [Uuid::new_v4(), Uuid::new_v4()];
    let structure = StructureFrozen {
      name: "TickId".to_string(),
      parent: Uuid::new_v4(),
      fields: IndexMap::from_iter([
        (ids[0], u64_field_frozen("callable_id")),
        (ids[1], u64_field_frozen("sequence")),
      ]),
    };

    let yaml = serde_yaml::to_string(&structure).unwrap();
    println!("YAML: {}", yaml);
    let deserialized: StructureFrozen = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(structure, deserialized);
  }

  /// Fields must appear in insertion order in the serialized YAML so that
  /// generated record files don't produce spurious diffs across rebuilds.
  #[test]
  fn test_yaml_field_order_matches_insertion_order() {
    let callable_id = Uuid::new_v4();
    let sequence_id = Uuid::new_v4();

    let structure = Structure {
      name: "TickId".to_string(),
      parent: Uuid::new_v4(),
      fields: IndexMap::from_iter([
        (callable_id, u64_field("callable_id")),
        (sequence_id, u64_field("sequence")),
      ]),
      acl: <Acl as Default>::default(),
    };

    let yaml = serde_yaml::to_string(&structure).unwrap();
    let callable_pos = yaml.find("callable_id").expect("callable_id not in YAML");
    let sequence_pos = yaml.find("sequence").expect("sequence not in YAML");
    assert!(callable_pos < sequence_pos, "callable_id should appear before sequence");

    // Serialization must also be deterministic across multiple calls.
    let yaml2 = serde_yaml::to_string(&structure).unwrap();
    assert_eq!(yaml, yaml2, "Repeated serialization must produce identical output");
  }
}
