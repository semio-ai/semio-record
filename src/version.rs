pub enum ValidationResult {
  Invalid,
  Valid,
  MajorIncrementRequired,
  MinorIncrementRequired,
  PatchIncrementRequired,
}

pub trait ValidatedSemanticVersion {
  fn validate(&self, next: &Self) -> ValidationResult;
}
