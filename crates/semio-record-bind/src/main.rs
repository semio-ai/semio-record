use std::{
  collections::{BTreeMap, HashMap, LinkedList, HashSet},
  path::PathBuf,
};

use schemars::schema::{RootSchema, Schema, SchemaObject};
use semio_record::{record::*, schema, schema_version::Schema as VersionSchema};
use serde::{Deserialize, Serialize};

use std::future::Future;
use std::pin::Pin;
use tokio::fs::{create_dir, create_dir_all, remove_dir_all};

use clap::Parser;
use tokio::io::AsyncWriteExt;

use camino::Utf8PathBuf;

mod merge;
mod schema;

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

fn parse_schema_object(name: &str, mut logical_path: Utf8PathBuf, object: &SchemaObject) -> String {
  println!("{}: {:#?}", name, object);
  // Remove file name
  logical_path.pop();

  let mut imports = HashMap::<String, Utf8PathBuf>::new();

  let mut ret = String::new();

  if let Some(subschemas) = &object.subschemas {
    if let Some(one_of) = &subschemas.one_of {
      if one_of.len() == 0 {
        ret += &format!("type {name} = void;\n");
      }

      ret += &format!("namespace {name} {{\n");

      let mut members = HashSet::new();
      // Union
      for schema in one_of {
        let variant_name = schema::name(schema);
        if let Some(variant_name) = variant_name {
          members.insert(format!("{name}.{variant_name}"));
          ret += &format!("  export interface {variant_name} {{\n");
          if let Some(properties) = schema::object_properties(&schema) {
            for (name, property) in properties {
              if let Schema::Object(object) = property.schema {
                ret += &format!("    {name}");
                if !property.required {
                  ret += "?";
                }
                ret += ": ";
                if let Some(enum_values) = &object.enum_values {
                  for (i, enum_value) in enum_values.iter().enumerate() {
                    if i > 0 {
                      ret += " | ";
                    }
                    ret += &format!("{enum_value}");
                  }
                } else if let Some(reference) = &object.reference {
                  let reference = reference.trim_start_matches("#/definitions/");
                  let path = name_to_path(reference);
                  let name = path.file_name().unwrap().to_string();
                  ret += &format!("{name}");
                  imports.insert(name, path);
                }
                ret += ";\n";
              }
            }
          }
          ret += &format!("  }}\n");
        }
      }
      ret += &format!("}}\n");

      ret += &format!("type {name} = ");
      for (i, member) in members.iter().enumerate() {
        if i > 0 {
          ret += " | ";
        }
        ret += &format!("{member}");
      }
      ret += ";\n";
    }
  } else if let Some(object_validation) = &object.object {
    ret += &format!("interface {name} {{\n");
    for (name, property) in &object_validation.properties {
      if let Schema::Object(property_object) = &property {
        ret += &format!("  {name}");
        if !object_validation.required.contains(name) {
          ret += "?";
        }
        ret += ": ";
        if let Some(enum_values) = &property_object.enum_values {
          for (i, enum_value) in enum_values.iter().enumerate() {
            if i > 0 {
              ret += " | ";
            }
            ret += &format!("{enum_value}");
          }
        } else if let Some(reference) = &property_object.reference {
          let reference = reference.trim_start_matches("#/definitions/");
          let path = name_to_path(reference);
          let name = path.file_name().unwrap().to_string();
          ret += &format!("{name}");
          imports.insert(name, path);
        } else {
          ret += &format!("any");
        }
        ret += ";\n";
      }
    }
    ret += &format!("}}\n");
  } else {
    ret += &format!("{:#?}", object);
  }

  for (name, path) in imports {
    let relative_path = pathdiff::diff_utf8_paths(path, &logical_path).unwrap();
    ret = format!("import {name} from \"./{relative_path}\";\n") + ret.as_str();
  }

  ret += &format!("export default {name};");

  ret
}

fn parse_schema(name: &str, logical_path: Utf8PathBuf, schema: &Schema) -> String {
  match schema {
    Schema::Object(object) => parse_schema_object(name, logical_path, object),
    _ => "".to_string(),
  }
}

async fn write_file(path: PathBuf, name: &str, logical_path: Utf8PathBuf, schema: &Schema) -> tokio::io::Result<()> {
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
