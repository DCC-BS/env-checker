use crate::types::EnvFileEntry;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn parse_env_file(file_path: &Path) -> Result<Vec<EnvFileEntry>> {
    let content = fs::read_to_string(file_path)
        .context(format!("Failed to read env file: {}", file_path.display()))?;

    let mut entries = Vec::new();
    let comment_regex = Regex::new(r"^\s*#.*$")?;
    let empty_regex = Regex::new(r"^\s*$")?;
    let export_regex = Regex::new(r"^\s*export\s+")?;
    let var_regex = Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$")?;

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();

        if comment_regex.is_match(line) || empty_regex.is_match(line) {
            continue;
        }

        let line = export_regex.replace(line, "");

        if let Some(captures) = var_regex.captures(&*line) {
            let name = captures.get(1).unwrap().as_str().to_string();
            let raw_value = captures.get(2).unwrap().as_str().trim();

            let value = if raw_value.is_empty() || raw_value == "''" || raw_value == "\"\"" {
                None
            } else {
                Some(parse_env_value(raw_value))
            };

            entries.push(EnvFileEntry {
                name,
                value,
                line: line_num,
                file_path: file_path.to_string_lossy().to_string(),
            });
        }
    }

    Ok(entries)
}

fn parse_env_value(value: &str) -> String {
    let value = value.trim();

    if (value.starts_with('"') && value.ends_with('"')) 
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        let stripped = &value[1..value.len() - 1];
        stripped.replace(r"\n", "\n").replace(r"\t", "\t").replace(r"\\", "\\")
    } else if value.starts_with('"') && !value.ends_with('"') {
        let mut result = String::new();
        let mut chars = value[1..].chars().peekable();
        let mut in_escape = false;

        while let Some(c) = chars.next() {
            if in_escape {
                match c {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    _ => result.push(c),
                }
                in_escape = false;
            } else if c == '\\' {
                in_escape = true;
            } else {
                result.push(c);
            }
        }

        result
    } else {
        value.to_string()
    }
}

pub fn merge_env_files(entries_list: Vec<Vec<EnvFileEntry>>) -> Vec<EnvFileEntry> {
    let mut merged = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for entries in entries_list {
        for entry in entries {
            if seen.insert(entry.name.clone()) {
                merged.push(entry);
            }
        }
    }

    merged
}
