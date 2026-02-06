use crate::types::{Config, SchemaSource};
use anyhow::{Context, Result};
use glob::glob;
use std::fs;
use std::path::Path;

pub fn load_config(workspace_root: &Path) -> Result<Option<Config>> {
    let config_path = workspace_root.join(".envchecker.json");
    
    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .context("Failed to read .envchecker.json")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse .envchecker.json")?;
    
    Ok(Some(config))
}

pub fn discover_schemas(workspace_root: &Path, config: &Config) -> Result<Vec<SchemaSource>> {
    let mut schemas = Vec::new();

    if config.auto_discover {
        schemas.extend(discover_zod_schemas(workspace_root)?);
        schemas.extend(discover_pydantic_schemas(workspace_root)?);
        schemas.extend(discover_yaml_schemas(workspace_root)?);
    }

    for schema_path in &config.schema_files {
        let full_path = workspace_root.join(schema_path);
        if !full_path.exists() {
            continue;
        }

        let source = match full_path.extension().and_then(|e| e.to_str()) {
            Some("ts") | Some("js") | Some("tsx") | Some("jsx") => SchemaSource::Zod {
                file_path: full_path.to_string_lossy().to_string(),
            },
            Some("py") => SchemaSource::Pydantic {
                file_path: full_path.to_string_lossy().to_string(),
            },
            Some("yml") | Some("yaml") => SchemaSource::Yaml {
                file_path: full_path.to_string_lossy().to_string(),
            },
            _ => continue,
        };

        schemas.push(source);
    }

    Ok(schemas)
}

fn discover_zod_schemas(workspace_root: &Path) -> Result<Vec<SchemaSource>> {
    let mut schemas = Vec::new();
    let patterns = vec![
        "**/schema.ts",
        "**/schema.js",
        "**/config.ts",
        "**/config.js",
        "**/env.ts",
        "**/env.js",
        "**/*config.ts",
        "**/*config.js",
    ];

    for pattern in patterns {
        let full_pattern = workspace_root.join(pattern).to_string_lossy().to_string();
        if let Ok(entries) = glob(&full_pattern) {
            for entry in entries.flatten() {
                if entry.is_file() {
                    schemas.push(SchemaSource::Zod {
                        file_path: entry.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    Ok(schemas)
}

fn discover_pydantic_schemas(workspace_root: &Path) -> Result<Vec<SchemaSource>> {
    let mut schemas = Vec::new();
    let patterns = vec![
        "**/config.py",
        "**/settings.py",
        "**/env.py",
        "**/configuration.py",
        "**/app_config.py",
    ];

    for pattern in patterns {
        let full_pattern = workspace_root.join(pattern).to_string_lossy().to_string();
        if let Ok(entries) = glob(&full_pattern) {
            for entry in entries.flatten() {
                if entry.is_file() {
                    schemas.push(SchemaSource::Pydantic {
                        file_path: entry.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    Ok(schemas)
}

fn discover_yaml_schemas(workspace_root: &Path) -> Result<Vec<SchemaSource>> {
    let mut schemas = Vec::new();
    let patterns = vec![
        "**/env.schema.yml",
        "**/env.schema.yaml",
        "**/.env.schema.yml",
        "**/.env.schema.yaml",
    ];

    for pattern in patterns {
        let full_pattern = workspace_root.join(pattern).to_string_lossy().to_string();
        if let Ok(entries) = glob(&full_pattern) {
            for entry in entries.flatten() {
                if entry.is_file() {
                    schemas.push(SchemaSource::Yaml {
                        file_path: entry.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    Ok(schemas)
}

pub fn get_env_file_paths(workspace_root: &Path, config: &Config) -> Result<Vec<String>> {
    let mut env_files = Vec::new();

    for pattern in &config.env_files {
        let full_path = workspace_root.join(pattern);
        
        if full_path.exists() && full_path.is_file() {
            env_files.push(full_path.to_string_lossy().to_string());
        } else {
            let full_pattern = workspace_root.join(pattern).to_string_lossy().to_string();
            if let Ok(entries) = glob(&full_pattern) {
                for entry in entries.flatten() {
                    if entry.is_file() {
                        env_files.push(entry.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    if env_files.is_empty() {
        let default_path = workspace_root.join(".env");
        if default_path.exists() {
            env_files.push(default_path.to_string_lossy().to_string());
        }
    }

    Ok(env_files)
}
