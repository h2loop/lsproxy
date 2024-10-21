use std::{error::Error, path::Path, process::Stdio};

use async_trait::async_trait;
use tokio::process::Command;

use crate::lsp::{
    workspace_documents::WorkspaceDocumentsHandler, JsonRpcHandler, LspClient, ProcessHandler,
    DEFAULT_EXCLUDE_PATTERNS,
};

pub struct RustAnalyzerClient {
    process: ProcessHandler,
    json_rpc: JsonRpcHandler,
    workspace_documents: WorkspaceDocumentsHandler,
}

pub const RUST_ANALYZER_ROOT_FILES: &[&str] = &["Cargo.toml"];
pub const RUST_ANALYZER_FILE_PATTERNS: &[&str] = &["**/*.rs"];

#[async_trait]
impl LspClient for RustAnalyzerClient {
    fn get_process(&mut self) -> &mut ProcessHandler {
        &mut self.process
    }

    fn get_json_rpc(&mut self) -> &mut JsonRpcHandler {
        &mut self.json_rpc
    }

    fn get_root_files(&mut self) -> Vec<String> {
        RUST_ANALYZER_ROOT_FILES
            .iter()
            .map(|&s| s.to_owned())
            .collect()
    }

    fn get_workspace_documents(&mut self) -> &mut WorkspaceDocumentsHandler {
        &mut self.workspace_documents
    }

    async fn setup_workspace(
        &mut self,
        _root_path: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // This is required for workspace features like go to definition to work
        self.send_lsp_request("rust-analyzer/reloadWorkspace", None)
            .await?;
        Ok(())
    }
}

impl RustAnalyzerClient {
    pub async fn new(root_path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let process = Command::new("rust-analyzer")
            .current_dir(root_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let process_handler = ProcessHandler::new(process)
            .await
            .map_err(|e| format!("Failed to create ProcessHandler: {}", e))?;
        let json_rpc_handler = JsonRpcHandler::new();

        let workspace_documents = WorkspaceDocumentsHandler::new(
            Path::new(root_path),
            RUST_ANALYZER_FILE_PATTERNS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            DEFAULT_EXCLUDE_PATTERNS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        );

        Ok(Self {
            process: process_handler,
            json_rpc: json_rpc_handler,
            workspace_documents,
        })
    }
}
