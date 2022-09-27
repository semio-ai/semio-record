use std::collections::BTreeMap;

use schemars::schema::{Schema, ObjectValidation};

pub fn object_validation(schema: &Schema) -> Option<&Box<ObjectValidation>> {
  if let Schema::Object(object) = &schema {
    object.object.as_ref()
  } else {
    None
  }
}

pub struct ObjectProperty<'a> {
  pub schema: &'a Schema,
  pub required: bool
}

pub fn object_properties<'a>(schema: &'a Schema) -> Option<BTreeMap<String, ObjectProperty<'a>>> {
  if let Some(object_validation) = object_validation(schema) {
    let mut properties = BTreeMap::new();
    for (name, property) in object_validation.properties.iter() {
      let required = object_validation.required.contains(name);
      let property = ObjectProperty {
        schema: property,
        required,
      };
      properties.insert(name.to_string(), property);
    }
    Some(properties)
  } else {
    None
  }
}

pub fn name(schema: &Schema) -> Option<&str> {
  if let Schema::Object(object) = schema {
    
    // Try to find given name
    if let Some(metadata) = &object.metadata {
      if let Some(title) = &metadata.title {
        return Some(&title);
      }
    }

    // Use `type` property
    if let Some(properties) = object_properties(schema) {
      if let Some(property) = properties.get("type") {
        if let Schema::Object(object) = property.schema {
          if let Some(enum_values) = &object.enum_values {
            if enum_values.len() == 1 {
              if let serde_json::Value::String(string) = &enum_values[0] {
                return Some(&string);
              }
            }
          }
        }
      }
    }
  }

  None
}