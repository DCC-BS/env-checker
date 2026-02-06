use crate::types::EnvVar;

use tower_lsp::lsp_types::{
    Hover, HoverContents, MarkupContent, MarkupKind,
};

pub fn create_hover(var: &EnvVar) -> Option<Hover> {
    let mut markdown = String::new();

    markdown.push_str(&format!("**Type:** `{}`\n\n", format_type(&var.var_type)));

    if let Some(desc) = &var.description {
        markdown.push_str(&format!("**Description:** {}\n\n", desc));
    }

    if let Some(default) = &var.default {
        markdown.push_str(&format!("**Default:** `{}`\n\n", default));
    }

    if var.optional {
        markdown.push_str("**Required:** `false`\n");
    } else {
        markdown.push_str("**Required:** `true`\n");
    }

    if let Some(group) = &var.group {
        markdown.push_str(&format!("\n**Group:** `{}`", group));
    }

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: markdown,
        }),
        range: None,
    })
}

fn format_type(env_type: &crate::types::EnvType) -> String {
    match env_type {
        crate::types::EnvType::String => "string".to_string(),
        crate::types::EnvType::Boolean => "boolean".to_string(),
        crate::types::EnvType::Number => "number".to_string(),
        crate::types::EnvType::Integer => "integer".to_string(),
    }
}
