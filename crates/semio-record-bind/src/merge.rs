use std::collections::BTreeMap;

use schemars::schema::{Schema, RootSchema};

use semio_record::{schema_version::Schema as VersionSchema};

pub(crate) fn into<I: Iterator<Item = (String, Schema)>>(definitions: I, res: &mut BTreeMap<String, Schema>) {
  for (name, schema) in definitions {
    if res.contains_key(&name) {
      continue;
    }
    res.insert(name, schema);
  }
}

pub(crate) fn root_schema(root_schema: RootSchema, res: &mut BTreeMap<String, Schema>) {
  if let Some(metadata) = &root_schema.schema.metadata {
    if let Some(title) = &metadata.title {
      res.insert(title.clone(), Schema::Object(root_schema.schema));
    }
  }
  
  into(root_schema.definitions.into_iter(), res);
}

pub(crate) fn version(version_schema: VersionSchema, res: &mut BTreeMap<String, Schema>) {
  root_schema(version_schema.public, res);
  root_schema(version_schema.private, res);
  root_schema(version_schema.action, res);
  root_schema(version_schema.frozen, res);
}