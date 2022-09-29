use std::{
  collections::{BTreeMap, HashMap, HashSet, LinkedList},
  path::PathBuf,
};

use schemars::{schema::{RootSchema, Schema, SchemaObject, SingleOrVec, InstanceType}, _serde_json::Value};
use semio_record::{record::*, schema, schema_version::Schema as VersionSchema};
use serde::{Deserialize, Serialize};

use std::future::Future;
use std::pin::Pin;
use tokio::fs::{create_dir, create_dir_all, remove_dir_all};

use clap::Parser;
use tokio::io::AsyncWriteExt;

use camino::Utf8PathBuf;

use crate::{imports::Imports, ast::{File, TypeExpr, TypeDecl, NamespaceDecl, InterfaceDecl, InterfaceField, Decl, ExportDefaultDecl, ToTypescript}};

mod imports;
mod merge;
mod schema;
mod ast;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scope {
  definitions: BTreeMap<String, Schema>,
  children: BTreeMap<String, Scope>,
}

fn lookup_scope<'a, 'b>(root: &'a mut Scope, parts: &'b [&str]) -> &'a mut Scope {
  if parts.len() == 0 {
    return root;
  }

  let (first, rest) = parts.split_first().unwrap();

  let child = root.children.entry(first.to_string()).or_insert(Scope {
    definitions: BTreeMap::new(),
    children: BTreeMap::new(),
  });

  lookup_scope(child, rest)
}

fn rescope(definitions: BTreeMap<String, Schema>) -> Scope {
  let mut root = Scope {
    definitions: BTreeMap::new(),
    children: BTreeMap::new(),
  };

  for (name, definition) in definitions {
    let parts: Vec<&str> = name.split('_').collect();
    let scope = lookup_scope(&mut root, &parts[..parts.len() - 1]);
    scope
      .definitions
      .insert(parts[parts.len() - 1].to_string(), definition);
  }

  root
}

fn name_to_path(name: &str) -> Utf8PathBuf {
  let mut path = Utf8PathBuf::new();
  path.push("/");
  for part in name.split('_') {
    path.push(part);
  }
  path
}

fn instance_type_to_ts(instance_type: &InstanceType) -> String {
  match instance_type {
    InstanceType::Null => "null".to_string(),
    InstanceType::Boolean => "boolean".to_string(),
    InstanceType::Integer => "number".to_string(),
    InstanceType::Number => "number".to_string(),
    InstanceType::String => "string".to_string(),
    InstanceType::Array => "Array".to_string(),
    InstanceType::Object => "object".to_string(),
  }
}

fn schema_object_to_type(schema_object: &SchemaObject, imports: &mut Imports) -> TypeExpr {
  if let Some(enum_values) = &schema_object.enum_values {
    TypeExpr::union(enum_values.iter().map(|v| match v {
      Value::String(s) => TypeExpr::string_literal(s),
      Value::Number(n) => TypeExpr::number_literal(n.as_i64().unwrap()),
      _ => TypeExpr::identifier("never"),
    }).collect())
  } else if let Some(reference) = &schema_object.reference {
    let reference = reference.trim_start_matches("#/definitions/");
    let path = name_to_path(reference);
    let name = path.file_name().unwrap().to_string();
    let import_name = imports.add(name, path);
    TypeExpr::Identifier(import_name)
  } else if let Some(instance_type) = &schema_object.instance_type {
    match instance_type {
      SingleOrVec::Single(single) => {
        TypeExpr::identifier(&instance_type_to_ts(single))
      },
      SingleOrVec::Vec(vec) => {
        TypeExpr::union(vec
          .iter()
          .map(|v| TypeExpr::identifier(&instance_type_to_ts(v)))
          .collect()
        )
      }
    }
  } else {
    TypeExpr::identifier("any")
  }
}

fn parse_schema_object(name: &str, mut logical_path: Utf8PathBuf, object: &SchemaObject) -> String {
  // Remove file name
  logical_path.pop();

  let mut imports = Imports::new(name, logical_path);

  let mut file = File::new();

  if let Some(subschemas) = &object.subschemas {
    if let Some(one_of) = &subschemas.one_of {
      if one_of.len() == 0 {
        file.append(TypeDecl::new(name, TypeExpr::identifier("void")));
      }

      let mut namespace = NamespaceDecl::new(name);
      // Union
      for schema in one_of {
        let variant_name = schema::name(schema);
        if let Some(variant_name) = variant_name {
          let mut interface = InterfaceDecl::export(variant_name);
          if let Some(properties) = schema::object_properties(&schema) {
            for (name, property) in properties {
              if let Schema::Object(object) = property.schema {
                interface.add_field(InterfaceField {
                  name: name.to_string(),
                  optional: Some(!property.required),
                  description: None,
                  ty: schema_object_to_type(object, &mut imports)
                });
              }
            }
          }
          namespace.append(interface);
        }
      }

      let union = namespace.decls.iter().filter_map(|decl| {
        if let Decl::Interface(interface) = decl {
          Some(TypeExpr::identifier(format!("{}.{}", &name, &interface.name)))
        } else {
          None
        }
      }).collect();

      file.append(namespace);
      file.append(TypeDecl::new(name, TypeExpr::union(union)));
    }
  } else if let Some(object_validation) = &object.object {
    let mut interface = InterfaceDecl::new(name);
    for (name, property) in &object_validation.properties {
      if let Schema::Object(property_object) = &property {
        interface.add_field(InterfaceField {
          name: name.to_string(),
          optional: Some(!object_validation.required.contains(name)),
          description: None,
          ty: schema_object_to_type(property_object, &mut imports)
        });
      }
    }
    file.append(interface);
  } else if let Some(enum_values) = &object.enum_values {
    file.append(TypeDecl::new(name, TypeExpr::union(enum_values.iter().map(|v| match v {
      Value::String(s) => TypeExpr::string_literal(s),
      Value::Number(n) => TypeExpr::number_literal(n.as_i64().unwrap()),
      _ => TypeExpr::identifier("never"),
    }).collect())));
  } else if let Some(instance_type) = &object.instance_type {
    match instance_type {
      SingleOrVec::Single(single) => {
        file.append(TypeDecl::new(name, TypeExpr::identifier(&instance_type_to_ts(single))))
      },
      SingleOrVec::Vec(vec) => {
        file.append(TypeDecl::new(name, TypeExpr::union(vec.iter().map(|v| TypeExpr::identifier(&instance_type_to_ts(v))).collect())))
      }
    }

  }

  file.prepend_all(imports.decls().into_iter());
  file.append(Decl::ExportDefault(ExportDefaultDecl {
    identifier: name.to_string(),
  }));

  file.to_typescript().to_string(0)
}

fn parse_schema(name: &str, logical_path: Utf8PathBuf, schema: &Schema) -> String {
  match schema {
    Schema::Object(object) => parse_schema_object(name, logical_path, object),
    _ => "".to_string(),
  }
}

async fn write_file(
  path: PathBuf,
  name: &str,
  logical_path: Utf8PathBuf,
  schema: &Schema,
) -> tokio::io::Result<()> {
  let mut file = tokio::fs::File::create(path).await?;
  file
    .write_all(parse_schema(name, logical_path, schema).as_bytes())
    .await?;
  Ok(())
}

fn write_scope<'a>(
  path: PathBuf,
  logical_path: Utf8PathBuf,
  scope: &'a Scope,
) -> Pin<Box<dyn Future<Output = tokio::io::Result<()>> + 'a>> {
  Box::pin(async move {
    create_dir(&path).await?;

    for (name, definition) in &scope.definitions {
      let mut definition_path = path.clone();
      definition_path.push(format!("{}.ts", name));
      let mut child_logical_path = logical_path.clone();
      child_logical_path.push(name);
      write_file(definition_path, &name, child_logical_path, &definition).await?;
    }

    for (name, scope) in scope.children.iter() {
      let mut child_path = path.clone();
      child_path.push(name);
      let mut child_logical_path = logical_path.clone();
      child_logical_path.push(name);
      write_scope(child_path, child_logical_path, &scope).await?;
    }

    Ok(())
  })
}

#[derive(Parser, Debug)]
struct Args {
  #[clap(short, long)]
  outdir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  let mut definitions = BTreeMap::new();

  merge::version(schema(TYPE_FOLDER, 0)?, &mut definitions);
  merge::version(schema(TYPE_USER, 0)?, &mut definitions);
  merge::version(schema(TYPE_ORGANIZATION, 0)?, &mut definitions);
  merge::version(schema(TYPE_STRUCTURE, 0)?, &mut definitions);
  merge::version(schema(TYPE_ENUMERATION, 0)?, &mut definitions);
  merge::version(schema(TYPE_MODULE, 0)?, &mut definitions);
  merge::version(schema(TYPE_ANIMATION, 0)?, &mut definitions);
  merge::version(schema(TYPE_ANIMATION, 1)?, &mut definitions);

  let root = rescope(definitions);

  let mut marker_path = args.outdir.clone();
  marker_path.push(".marker");

  if args.outdir.exists() {
    // Check for marker file
    if !marker_path.exists() {
      return Err("Output directory is not empty".into());
    }

    remove_dir_all(&args.outdir).await?;
  }

  let mut logical_root_path = Utf8PathBuf::new();
  logical_root_path.push("/");
  write_scope(args.outdir, logical_root_path, &root).await?;
  // Write out blank marker file
  tokio::fs::File::create(marker_path).await?;

  Ok(())
}
