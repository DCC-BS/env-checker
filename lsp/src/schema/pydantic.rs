use crate::types::{EnvType, EnvVar, ParsedSchema, SchemaSource};
use crate::schema::map_type_string;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;

pub fn parse_pydantic_schema(file_path: &str) -> Result<Option<ParsedSchema>> {
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read Python file: {}", file_path))?;

    let mut variables = Vec::new();
    find_class_definitions(&content, &mut variables);

    if variables.is_empty() {
        return Ok(None);
    }

    Ok(Some(ParsedSchema {
        source: SchemaSource::Pydantic {
            file_path: file_path.to_string(),
        },
        variables,
    }))
}

fn find_class_definitions(content: &str, variables: &mut Vec<EnvVar>) {
    let class_regex = Regex::new(r"class\s+(\w+)\s*(?:\([^)]*\))?:\s*\n((?:\s{4}[^\n]+\n)*)").unwrap();
    
    for captures in class_regex.captures_iter(content) {
        if let Some(body) = captures.get(2) {
            parse_class_body(body.as_str(), variables);
        }
    }
}

fn parse_class_body(body: &str, variables: &mut Vec<EnvVar>) {
    let assignment_regex = Regex::new(r"(\w+)\s*:\s*([^\n=]+)(?:\s*=\s*([^\n]+))?").unwrap();
    
    for line in body.lines() {
        if let Some(captures) = assignment_regex.captures(line) {
            if let (Some(name), Some(type_hint)) = (captures.get(1), captures.get(2)) {
                let value = captures.get(3).map_or("", |v| v.as_str());
                parse_field_assignment(
                    name.as_str(),
                    type_hint.as_str(),
                    value,
                    variables,
                );
            }
        }
    }
}

fn parse_field_assignment(name: &str, type_hint: &str, value: &str, variables: &mut Vec<EnvVar>) {
    let var_type = map_type_string(type_hint);
    let mut description = None;
    let mut default = None;
    let mut optional = false;

    if type_hint.contains("Optional[") || type_hint.contains(" | None") || type_hint.contains("NoneType") {
        optional = true;
    }

    if value.contains("Field(") {
        parse_field_args(value, &mut description, &mut default, &mut optional);
    }

    if default.is_some() {
        optional = true;
    }

    variables.push(EnvVar {
        name: name.to_uppercase(),
        var_type,
        description,
        default,
        optional,
        group: None,
    });
}

fn parse_field_args(value: &str, description: &mut Option<String>, default: &mut Option<String>, _optional: &mut bool) {
    let desc_regex = Regex::new(r#"description\s*=\s*["']([^"']+)["']"#).unwrap();
    if let Some(captures) = desc_regex.captures(value) {
        if let Some(desc) = captures.get(1) {
            *description = Some(desc.as_str().to_string());
        }
    }

    let default_regex = Regex::new(r"default\s*=\s*([^\),]+)").unwrap();
    if let Some(captures) = default_regex.captures(value) {
        if let Some(def) = captures.get(1) {
            let def = def.as_str().trim();
            *default = Some(sanitize_python_value(def));
        }
    }
}

fn sanitize_python_value(value: &str) -> String {
    let value = value.trim();
    
    if value.starts_with('"') && value.ends_with('"') {
        value[1..value.len()-1].to_string()
    } else if value.starts_with('\'') && value.ends_with('\'') {
        value[1..value.len()-1].to_string()
    } else if value == "True" {
        "true".to_string()
    } else if value == "False" {
        "false".to_string()
    } else if value == "None" {
        String::new()
    } else {
        value.to_string()
    }
}
