use crate::types::{EnvType, EnvVar, ParsedSchema, SchemaSource};
use crate::schema::map_type_string;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct YamlSchema {
    variables: Option<YamlVariables>,
}

#[derive(Debug, Deserialize)]
struct YamlVariables {
    #[serde(flatten)]
    vars: std::collections::HashMap<String, YamlVar>,
}

#[derive(Debug, Deserialize)]
struct YamlVar {
    #[serde(rename = "type")]
    var_type: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    default: Option<String>,
    #[serde(default = "default_false")]
    required: bool,
    #[serde(default)]
    group: Option<String>,
}

fn default_false() -> bool {
    false
}

pub fn parse_yaml_schema(file_path: &str) -> Result<Option<ParsedSchema>> {
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read YAML file: {}", file_path))?;

    let yaml: YamlSchema = serde_yaml::from_str(&content)
        .context("Failed to parse YAML schema")?;

    let variables = if let Some(vars_map) = yaml.variables {
        vars_map.vars
            .into_iter()
            .map(|(name, var)| EnvVar {
                name: name.to_uppercase(),
                var_type: map_type_string(&var.var_type),
                description: var.description,
                default: var.default,
                optional: !var.required,
                group: var.group,
            })
            .collect()
    } else {
        Vec::new()
    };

    if variables.is_empty() {
        return Ok(None);
    }

    Ok(Some(ParsedSchema {
        source: SchemaSource::Yaml {
            file_path: file_path.to_string(),
        },
        variables,
    }))
}
