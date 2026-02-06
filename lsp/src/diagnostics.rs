use crate::validation::ValidationError;

use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, Position, Range, NumberOrString,
};

pub fn create_missing_var_diagnostic(error: &ValidationError) -> Diagnostic {
    let position = Position::new(0, 0);
    let range = Range::new(position, position);

    let mut message = format!(
        "Missing required environment variable: '{}'",
        error.var_name
    );

    if let Some(description) = &error.schema_var.description {
        message.push_str(&format!("\n  Description: {}", description));
    }

    if let Some(default) = &error.schema_var.default {
        message.push_str(&format!("\n  Default: {}", default));
    }

    let severity = Some(DiagnosticSeverity::ERROR);

    Diagnostic {
        range,
        severity,
        code: Some(NumberOrString::String("missing-env-var".to_string())),
        source: Some("env-checker".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
        code_description: None,
    }
}

pub fn create_unused_var_diagnostic(var_name: &str, line: usize) -> Diagnostic {
    let position = Position::new(line as u32, 0);
    let end_position = Position::new(line as u32, var_name.len() as u32);
    let range = Range::new(position, end_position);

    let message = format!(
        "Environment variable '{}' is not defined in any schema",
        var_name
    );

    let severity = Some(DiagnosticSeverity::INFORMATION);

    Diagnostic {
        range,
        severity,
        code: Some(NumberOrString::String("unused-env-var".to_string())),
        source: Some("env-checker".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
        code_description: None,
    }
}
