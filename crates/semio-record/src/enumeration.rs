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
    frozen::{Enumeration as EnumerationFrozen, EnumerationVariant as EnumerationVariantFrozen},
    unfrozen::{Enumeration, EnumerationVariant},
  };

  fn unit_variant(name: &str) -> EnumerationVariant {
    EnumerationVariant {
      name: name.to_string(),
      ty: UnfrozenTy::Primitive(Primitive { kind: PrimitiveKind::Unit }),
    }
  }

  fn unit_variant_frozen(name: &str) -> EnumerationVariantFrozen {
    EnumerationVariantFrozen {
      name: name.to_string(),
      ty: FrozenTy::Primitive(Primitive { kind: PrimitiveKind::Unit }),
    }
  }

  #[test]
  fn test_serde_yaml_unfrozen() {
    let ids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let enumeration_meta = Enumeration {
      name: "Status".to_string(),
      parent: Uuid::new_v4(),
      variants: IndexMap::from_iter([
        (ids[0], unit_variant("Success")),
        (ids[1], unit_variant("Failure")),
        (ids[2], unit_variant("Running")),
      ]),
      acl: <Acl as Default>::default(),
    };

    let yaml = serde_yaml::to_string(&enumeration_meta).unwrap();
    println!("YAML: {}", yaml);
    let deserialized: Enumeration = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(enumeration_meta, deserialized);
  }

  #[test]
  fn test_serde_yaml_frozen() {
    let ids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let enumeration_meta = EnumerationFrozen {
      name: "Status".to_string(),
      parent: Uuid::new_v4(),
      variants: IndexMap::from_iter([
        (ids[0], unit_variant_frozen("Success")),
        (ids[1], unit_variant_frozen("Failure")),
        (ids[2], unit_variant_frozen("Running")),
      ]),
    };

    let yaml = serde_yaml::to_string(&enumeration_meta).unwrap();
    println!("YAML: {}", yaml);
    let deserialized: EnumerationFrozen = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(enumeration_meta, deserialized);
  }

  /// Variants must appear in insertion order in the serialized YAML so that
  /// generated record files don't produce spurious diffs across rebuilds.
  #[test]
  fn test_yaml_variant_order_matches_insertion_order() {
    let success_id = Uuid::new_v4();
    let failure_id = Uuid::new_v4();
    let running_id = Uuid::new_v4();

    let enumeration = Enumeration {
      name: "Status".to_string(),
      parent: Uuid::new_v4(),
      variants: IndexMap::from_iter([
        (success_id, unit_variant("Success")),
        (failure_id, unit_variant("Failure")),
        (running_id, unit_variant("Running")),
      ]),
      acl: <Acl as Default>::default(),
    };

    let yaml = serde_yaml::to_string(&enumeration).unwrap();
    let success_pos = yaml.find("Success").expect("Success not in YAML");
    let failure_pos = yaml.find("Failure").expect("Failure not in YAML");
    let running_pos = yaml.find("Running").expect("Running not in YAML");
    assert!(success_pos < failure_pos, "Success should appear before Failure");
    assert!(failure_pos < running_pos, "Failure should appear before Running");

    // Serialization must also be deterministic across multiple calls.
    let yaml2 = serde_yaml::to_string(&enumeration).unwrap();
    assert_eq!(yaml, yaml2, "Repeated serialization must produce identical output");
  }
}
