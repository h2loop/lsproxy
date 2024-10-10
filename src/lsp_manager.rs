use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;
use log::{error, info};
use tokio::sync::Mutex;
use strum_macros::{EnumString, Display};
use crate::lsp_client::LspClient;

pub struct LspManager {
    clients: HashMap<(RepoKey, SupportedLSPs), Arc<Mutex<LspClient>>>,
}

#[derive(Debug, EnumString, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum SupportedLSPs {
    Python,
    TypeScript,
    Rust,
}

impl LspManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub async fn start_lsps(&mut self, key: RepoKey, repo_path: String, lsps: &[SupportedLSPs]) -> Result<(), String> {
        for &lsp in lsps {
            match lsp {
                SupportedLSPs::Python => self.start_python_lsp(&key, &repo_path).await?,
                SupportedLSPs::TypeScript => self.start_typescript_lsp(&key, &repo_path).await?,
                SupportedLSPs::Rust => self.start_rust_lsp(&key, &repo_path).await?,
            }
        }
        Ok(())
    }

    async fn start_python_lsp(&mut self, key: &RepoKey, repo_path: &str) -> Result<(), String> {
        let process = match Command::new("pyright-langserver")
            .arg("--stdio")
            .current_dir(repo_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Ok(child) => child,
                Err(e) => {
                    error!("Failed to start Pyright LSP for repo {}: {}", repo_path, e);
                    return Err(format!("Failed to start Pyright LSP: {}", e));
                }
            };

        self.create_and_initialize_client(key.clone(), SupportedLSPs::Python, process, repo_path.to_string()).await
    }

    async fn start_typescript_lsp(&mut self, key: &RepoKey, repo_path: &str) -> Result<(), String> {
        warn!("TypeScript LSP start requested but not implemented for repo: {}", repo_path);
        Err("TypeScript LSP not implemented".to_string())
    }

    async fn start_rust_lsp(&mut self, key: &RepoKey, repo_path: &str) -> Result<(), String> {
        warn!("Rust LSP start requested but not implemented for repo: {}", repo_path);
        Err("Rust LSP not implemented".to_string())
    }

    async fn create_and_initialize_client(&mut self, key: RepoKey, lsp_type: SupportedLSPs, process: std::process::Child, repo_path: String) -> Result<(), String> {
        let client = LspClient::new(process)
            .map_err(|e| format!("Failed to create LSP client: {}", e))?;
        let client = Arc::new(Mutex::new(client));
        self.clients.insert((key.clone(), lsp_type), client.clone());

        // Initialize the client
        let mut locked_client = client.lock().await;
        locked_client.initialize(Some(repo_path.clone()))
            .await
            .map_err(|e| format!("Failed to initialize LSP client: {}", e))?;

        info!("Started and initialized {:?} LSP for repo: {}", lsp_type, repo_path);
        Ok(())
    }

    pub fn get_client(&self, key: &RepoKey, lsp_type: SupportedLSPs) -> Option<Arc<Mutex<LspClient>>> {
        self.clients.get(&(key.clone(), lsp_type)).cloned()
    }
}
