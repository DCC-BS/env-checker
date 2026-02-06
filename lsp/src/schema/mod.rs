use crate::types::{EnvType, EnvVar, ParsedSchema, SchemaSource};
use anyhow::Result;

pub mod zod;
pub mod pydantic;
pub mod yaml;

pub fn parse_schema(source: &SchemaSource) -> Result<Option<ParsedSchema>> {
    match source {
        SchemaSource::Zod { file_path } => zod::parse_zod_schema(file_path),
        SchemaSource::Pydantic { file_path } => pydantic::parse_pydantic_schema(file_path),
        SchemaSource::Yaml { file_path } => yaml::parse_yaml_schema(file_path),
    }
}

fn map_type_string(type_str: &str) -> EnvType {
    let type_str = type_str.to_lowercase();
    
    if type_str.contains("bool") {
        EnvType::Boolean
    } else if type_str.contains("int") || type_str.contains("number") {
        if type_str.contains("int") {
            EnvType::Integer
        } else {
            EnvType::Number
        }
    } else {
        EnvType::String
    }
}
