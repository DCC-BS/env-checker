use crate::types::{EnvVar, EnvFileEntry};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub var_name: String,
    pub missing_from_all: bool,
    pub schema_var: EnvVar,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub all_env_vars: HashMap<String, EnvFileEntry>,
    pub all_schema_vars: Vec<EnvVar>,
}

pub fn validate(schema_vars: &[EnvVar], env_entries: &[EnvFileEntry]) -> ValidationResult {
    let env_map: HashMap<String, EnvFileEntry> = env_entries
        .iter()
        .map(|e| (e.name.clone(), e.clone()))
        .collect();

    let mut errors = Vec::new();

    for schema_var in schema_vars {
        if schema_var.optional {
            continue;
        }

        if !env_map.contains_key(&schema_var.name) {
            errors.push(ValidationError {
                var_name: schema_var.name.clone(),
                missing_from_all: true,
                schema_var: schema_var.clone(),
            });
        }
    }

    ValidationResult {
        errors,
        all_env_vars: env_map,
        all_schema_vars: schema_vars.to_vec(),
    }
}

pub fn group_variables(vars: &[EnvVar]) -> Vec<(String, Vec<EnvVar>)> {
    let mut groups: HashMap<String, Vec<EnvVar>> = HashMap::new();
    let mut ungrouped = Vec::new();

    for var in vars {
        if let Some(group) = &var.group {
            groups.entry(group.clone()).or_default().push(var.clone());
        } else {
            ungrouped.push(var.clone());
        }
    }

    let mut result: Vec<(String, Vec<EnvVar>)> = groups
        .into_iter()
        .map(|(k, mut v)| {
            v.sort_by(|a, b| a.name.cmp(&b.name));
            (k, v)
        })
        .collect();

    if !ungrouped.is_empty() {
        ungrouped.sort_by(|a, b| a.name.cmp(&b.name));
        result.push(("Other".to_string(), ungrouped));
    }

    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}
