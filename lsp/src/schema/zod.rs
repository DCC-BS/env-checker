use crate::types::{EnvType, EnvVar, ParsedSchema, SchemaSource};
use crate::schema::map_type_string;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;

pub fn parse_zod_schema(file_path: &str) -> Result<Option<ParsedSchema>> {
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read TypeScript file: {}", file_path))?;

    let mut variables = Vec::new();
    find_zod_object_definitions(&content, &mut variables);

    if variables.is_empty() {
        return Ok(None);
    }

    Ok(Some(ParsedSchema {
        source: SchemaSource::Zod {
            file_path: file_path.to_string(),
        },
        variables,
    }))
}

fn find_zod_object_definitions(content: &str, variables: &mut Vec<EnvVar>) {
    let object_regex = Regex::new(r"z\.object\s*\(\s*\{([^}]+)\}\s*\)").unwrap();
    
    for captures in object_regex.captures_iter(content) {
        if let Some(properties) = captures.get(1) {
            parse_object_properties(properties.as_str(), variables);
        }
    }

    let object_alt_regex = Regex::new(r"\.object\s*\(\s*\{([^}]+)\}\s*\)").unwrap();
    for captures in object_alt_regex.captures_iter(content) {
        if let Some(properties) = captures.get(1) {
            parse_object_properties(properties.as_str(), variables);
        }
    }
}

fn parse_object_properties(properties: &str, variables: &mut Vec<EnvVar>) {
    let property_regex = Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,\n]+(?:,[^,\n]*)*)").unwrap();
    
    for captures in property_regex.captures_iter(properties) {
        if let (Some(name), Some(value)) = (captures.get(1), captures.get(2)) {
            parse_property_chain(name.as_str(), value.as_str(), variables);
        }
    }
}

fn parse_property_chain(name: &str, value: &str, variables: &mut Vec<EnvVar>) {
    let mut var_type = EnvType::String;
    let mut description = None;
    let mut default = None;
    let mut optional = false;
    let mut group = None;

    let value = value.trim();

    parse_type_chain(value, &mut var_type, &mut description, &mut default, &mut optional, &mut group);

    variables.push(EnvVar {
        name: name.to_string(),
        var_type,
        description,
        default,
        optional,
        group,
    });
}

fn parse_type_chain(
    value: &str,
    var_type: &mut EnvType,
    description: &mut Option<String>,
    default: &mut Option<String>,
    optional: &mut bool,
    group: &mut Option<String>,
) {
    let value = value.trim();

    if value.contains(".optional(") || value.contains(".nullable(") {
        *optional = true;
    }

    if let Some(type_name) = extract_type_name(value) {
        *var_type = map_type_string(type_name);
    }

    if let Some(desc) = extract_description(value) {
        *description = Some(desc);
    }

    if let Some(def) = extract_default(value) {
        *default = Some(def);
    }

    if let Some(grp) = extract_group(value) {
        *group = Some(grp);
    }
}

fn extract_type_name(value: &str) -> Option<&str> {
    let types = ["z.string", "z.number", "z.boolean", "z.int", "z.float"];
    
    for type_name in &types {
        if value.contains(type_name) {
            return Some(&type_name[2..]);
        }
    }

    if value.contains("z.coerce.number") {
        return Some("number");
    }
    if value.contains("z.coerce.boolean") {
        return Some("boolean");
    }
    if value.contains("z.coerce.string") {
        return Some("string");
    }

    None
}

fn extract_description(value: &str) -> Option<String> {
    let describe_regex = Regex::new(r#"\.describe\s*\(\s*['"]([^'"]*)['"]\s*\)"#).unwrap();
    
    if let Some(captures) = describe_regex.captures(value) {
        if let Some(desc) = captures.get(1) {
            return Some(desc.as_str().to_string());
        }
    }

    None
}

fn extract_default(value: &str) -> Option<String> {
    let default_regex = Regex::new(r"\.default\s*\(\s*([^)]+)\s*\)").unwrap();
    
    if let Some(captures) = default_regex.captures(value) {
        if let Some(def) = captures.get(1) {
            let def = def.as_str().trim();
            if def.starts_with('"') && def.ends_with('"') {
                return Some(def[1..def.len()-1].to_string());
            } else if def == "false" {
                return Some("false".to_string());
            } else if def == "true" {
                return Some("true".to_string());
            } else {
                return Some(def.to_string());
            }
        }
    }

    None
}

fn extract_group(value: &str) -> Option<String> {
    let register_regex = Regex::new(r#"\.register\s*\([^,]+,\s*\{[^}]*group\s*:\s*['"]([^'"]*)['"]"#).unwrap();
    
    if let Some(captures) = register_regex.captures(value) {
        if let Some(grp) = captures.get(1) {
            return Some(grp.as_str().to_string());
        }
    }

    None
}
