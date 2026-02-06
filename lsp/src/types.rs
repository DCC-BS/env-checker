use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EnvType {
    String,
    Boolean,
    Number,
    Integer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub var_type: EnvType,
    pub description: Option<String>,
    pub default: Option<String>,
    pub optional: bool,
    pub group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaSource {
    Zod { file_path: String },
    Pydantic { file_path: String },
    Yaml { file_path: String },
}

#[derive(Debug, Clone)]
pub struct ParsedSchema {
    pub source: SchemaSource,
    pub variables: Vec<EnvVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub schema_files: Vec<String>,
    #[serde(default)]
    pub env_files: Vec<String>,
    #[serde(default = "default_true")]
    pub auto_discover: bool,
    #[serde(default)]
    pub groups: HashMap<String, String>,
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema_files: Vec::new(),
            env_files: vec![".env".to_string()],
            auto_discover: true,
            groups: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvFileEntry {
    pub name: String,
    pub value: Option<String>,
    pub line: usize,
    pub file_path: String,
}
