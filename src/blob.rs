use std::collections::HashSet;

use uuid::Uuid;

pub trait BlobDependencies {
  fn blob_dependencies<'a>(&'a self, set: &mut HashSet<&'a Uuid>);
}
