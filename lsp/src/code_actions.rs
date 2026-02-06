use crate::types::EnvVar;
use crate::validation::group_variables;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Position, Range, 
    TextEdit, WorkspaceEdit, Url,
};
use std::collections::HashMap;

pub fn create_append_missing_action(
    missing_vars: &[EnvVar],
    file_uri: &str,
    last_line: usize,
    line_content: Option<String>,
) -> CodeActionOrCommand {
    let mut edits = Vec::new();

    let content = generate_env_content(missing_vars);

    let position = if let Some(line) = line_content {
        if line.trim().is_empty() {
            Position::new(last_line as u32, 0)
        } else {
            Position::new((last_line + 1) as u32, 0)
        }
    } else {
        Position::new(last_line as u32, 0)
    };

    let text_edit = TextEdit::new(
        Range::new(position, position),
        content,
    );

    edits.push(text_edit);

    let uri = Url::parse(file_uri).unwrap_or_else(|_| Url::from_file_path(file_uri).unwrap());
    let mut workspace_edits = HashMap::new();
    workspace_edits.insert(uri, edits);

    let workspace_edit = WorkspaceEdit {
        changes: Some(workspace_edits),
        document_changes: None,
        change_annotations: None,
    };

    let title = format!(
        "Append {} missing environment variable(s)",
        missing_vars.len()
    );

    CodeAction {
        title,
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: None,
        edit: Some(workspace_edit),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    }
    .into()
}

pub fn create_generate_example_action(
    all_vars: &[EnvVar],
    file_uri: &str,
) -> CodeActionOrCommand {
    let content = generate_example_content(all_vars);

    let text_edit = TextEdit::new(
        Range::new(Position::new(0, 0), Position::new(u32::MAX, u32::MAX)),
        content,
    );

    let uri = Url::parse(file_uri).unwrap_or_else(|_| Url::from_file_path(file_uri).unwrap());
    let mut workspace_edits = HashMap::new();
    workspace_edits.insert(uri, vec![text_edit]);

    let workspace_edit = WorkspaceEdit {
        changes: Some(workspace_edits),
        document_changes: None,
        change_annotations: None,
    };

    CodeAction {
        title: "Generate .env.example file".to_string(),
        kind: Some(CodeActionKind::SOURCE),
        diagnostics: None,
        edit: Some(workspace_edit),
        command: None,
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }
    .into()
}

pub fn create_create_example_action(
    all_vars: &[EnvVar],
    workspace_uri: &str,
) -> CodeActionOrCommand {
    let example_uri = format!("{}/.env.example", workspace_uri.trim_end_matches('/'));
    let uri = Url::parse(&example_uri).unwrap_or_else(|_| Url::from_file_path(&example_uri).unwrap());

    let mut workspace_edits = HashMap::new();
    workspace_edits.insert(uri, vec![]);

    let workspace_edit = WorkspaceEdit {
        changes: Some(workspace_edits),
        document_changes: None,
        change_annotations: None,
    };

    CodeAction {
        title: "Create .env.example file".to_string(),
        kind: Some(CodeActionKind::SOURCE),
        diagnostics: None,
        edit: Some(workspace_edit),
        command: None,
        is_preferred: Some(false),
        disabled: None,
        data: None,
    }
    .into()
}

fn generate_env_content(vars: &[EnvVar]) -> String {
    let grouped = group_variables(vars);
    let mut content = String::new();

    for (group, group_vars) in grouped {
        content.push_str(&format!("\n# {}\n", group));
        
        for var in &group_vars {
            content.push_str(&format_var_entry(var, false));
        }
    }

    if !content.is_empty() && content.starts_with('\n') {
        content.remove(0);
    }

    content
}

fn generate_example_content(vars: &[EnvVar]) -> String {
    let grouped = group_variables(vars);
    let mut content = String::new();

    for (group, group_vars) in grouped {
        content.push_str(&format!("# {}\n", group));
        
        for var in &group_vars {
            content.push_str(&format_var_entry(var, true));
        }

        content.push('\n');
    }

    content.trim_end_matches('\n').to_string()
}

fn format_var_entry(var: &EnvVar, _is_example: bool) -> String {
    let mut result = String::new();

    if let Some(desc) = &var.description {
        result.push_str(&format!("# {}\n", desc));
    }

    if var.optional {
        result.push('#');
    }

    let default_value = var.default.as_deref().unwrap_or("");
    result.push_str(&format!("{}={}\n", var.name, default_value));

    result
}
