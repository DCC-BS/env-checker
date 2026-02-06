use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::{lsp_types::*, Client, LanguageServer, LspService, Server};
use tracing::{error, info};

mod code_actions;
mod config;
mod diagnostics;
mod env_file;
mod hover;
mod schema;
mod types;
mod validation;

use types::{Config, EnvVar, EnvFileEntry};

struct Backend {
    client: Client,
    workspace_root: Arc<RwLock<Option<PathBuf>>>,
    config: Arc<RwLock<Config>>,
    schemas: Arc<RwLock<Vec<EnvVar>>>,
    env_files: Arc<RwLock<HashMap<String, Vec<EnvFileEntry>>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(Config::default())),
            schemas: Arc::new(RwLock::new(Vec::new())),
            env_files: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn reload_schemas(&self) {
        let workspace_root = self.workspace_root.read().await;
        let config = self.config.read().await;
        
        if let Some(root) = workspace_root.as_ref() {
            let schema_sources = match config::discover_schemas(root, &config) {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to discover schemas: {:?}", e);
                    return;
                }
            };
            
            let mut all_vars = Vec::new();
            let mut seen_names = std::collections::HashSet::new();
            
            for source in &schema_sources {
                if let Ok(Some(parsed)) = schema::parse_schema(source) {
                    for var in parsed.variables {
                        if seen_names.insert(var.name.clone()) {
                            all_vars.push(var);
                        }
                    }
                }
            }

            info!("Loaded {} environment variable definitions", all_vars.len());
            *self.schemas.write().await = all_vars;
        }
    }

    async fn load_env_files(&self) {
        let workspace_root = self.workspace_root.read().await;
        let config = self.config.read().await;
        
        if let Some(root) = workspace_root.as_ref() {
            let env_file_paths = match config::get_env_file_paths(root, &config) {
                Ok(p) => p,
                Err(e) => {
                    error!("Failed to get env file paths: {:?}", e);
                    return;
                }
            };
            
            let mut env_files = HashMap::new();
            
            for path in env_file_paths {
                if let Ok(entries) = env_file::parse_env_file(Path::new(&path)) {
                    env_files.insert(path, entries);
                }
            }

            info!("Loaded {} .env files", env_files.len());
            *self.env_files.write().await = env_files;
        }
    }

    async fn validate_and_publish_diagnostics(&self, file_path: &Path) {
        let schemas = self.schemas.read().await;
        let env_files = self.env_files.read().await;
        
        let file_uri = match Url::from_file_path(file_path) {
            Ok(uri) => uri,
            Err(_) => {
                error!("Invalid file path: {:?}", file_path);
                return;
            }
        };
        
        let file_str = file_uri.to_string();
        
        let file_entries = env_files.get(&file_str).cloned().unwrap_or_default();
        let all_entries: Vec<_> = env_files.values().flat_map(|v| v.iter().cloned()).collect();
        
        let merged_entries = env_file::merge_env_files(vec![file_entries, all_entries]);
        
        let validation = validation::validate(&*schemas, &merged_entries);
        
        let mut diagnostics = Vec::new();
        
        for error in &validation.errors {
            diagnostics.push(diagnostics::create_missing_var_diagnostic(error));
        }

        self.client.publish_diagnostics(file_uri, diagnostics, None).await;
    }

    async fn get_last_line_info(&self, file_path: &str) -> tower_lsp::jsonrpc::Result<(usize, Option<String>)> {
        let env_files = self.env_files.read().await;
        
        if let Some(entries) = env_files.get(file_path) {
            if let Some(last_entry) = entries.last() {
                return Ok((last_entry.line + 1, Some(format!("{}={}", 
                    last_entry.name, 
                    last_entry.value.as_deref().unwrap_or("")))));
            }
        }
        
        Ok((0, None))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri {
            let root_path = root_uri.to_file_path().map_err(|e| {
                tower_lsp::jsonrpc::Error::invalid_params(format!("Failed to convert root URI to path: {:?}", e))
            })?;
            
            *self.workspace_root.write().await = Some(root_path.clone());
            
            if let Ok(Some(config)) = config::load_config(&root_path) {
                *self.config.write().await = config;
                info!("Loaded configuration from .envchecker.json");
            }
            
            self.reload_schemas().await;
            self.load_env_files().await;
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["=".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                ..Default::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Server initialized");
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        info!("Server shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let file_path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return,
        };

        if file_path.extension().and_then(|e| e.to_str()) == Some("env") {
            self.load_env_files().await;
            self.validate_and_publish_diagnostics(&file_path).await;
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let file_path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return,
        };

        if file_path.extension().and_then(|e| e.to_str()) == Some("env") {
            self.load_env_files().await;
            self.validate_and_publish_diagnostics(&file_path).await;
        }
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        let file_path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return Ok(None),
        };

        let env_files = self.env_files.read().await;
        let file_str = uri.to_string();
        
        if let Some(entries) = env_files.get(&file_str) {
            if let Some(entry) = entries.iter().find(|e| e.line as u32 == position.line) {
                let schemas = self.schemas.read().await;
                if let Some(var) = schemas.iter().find(|v| v.name == entry.name) {
                    return Ok(hover::create_hover(var));
                }
            }
        }

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> tower_lsp::jsonrpc::Result<Option<Vec<CodeActionOrCommand>>> {
        let uri = params.text_document.uri;
        match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return Ok(None),
        };

        let mut actions = Vec::new();
        
        let schemas = self.schemas.read().await;
        let env_files = self.env_files.read().await;
        let file_str = uri.to_string();
        
        let validation = if let Some(entries) = env_files.get(&file_str) {
            let all_entries: Vec<_> = env_files.values().flat_map(|v| v.iter().cloned()).collect();
            let merged = env_file::merge_env_files(vec![entries.clone(), all_entries]);
            validation::validate(&*schemas, &merged)
        } else {
            validation::validate(&*schemas, &[])
        };

        if !validation.errors.is_empty() {
            let missing_vars: Vec<_> = validation.errors
                .iter()
                .map(|e| e.schema_var.clone())
                .collect();

            let (last_line, last_content) = self.get_last_line_info(&file_str).await?;

            actions.push(code_actions::create_append_missing_action(
                &missing_vars,
                &uri.to_string(),
                last_line,
                last_content,
            ));
        }

        actions.push(code_actions::create_create_example_action(
            &schemas,
            &uri.to_string(),
        ));

        if !validation.errors.is_empty() {
            Ok(Some(actions))
        } else {
            Ok(Some(vec![actions[0].clone()]))
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Environment Checker LSP");

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(Backend::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
