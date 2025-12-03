//! LSP Backend implementation

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::capabilities::server_capabilities;
use crate::completion::provide_completions;
use crate::diagnostics::publish_diagnostics;
use crate::document::Document;
use crate::hover::provide_hover;
use crate::symbols::document_symbols;

/// The SketchDDD Language Server backend
pub struct SketchDDDBackend {
    /// LSP client for sending notifications
    client: Client,
    /// Open documents indexed by URI
    documents: DashMap<Url, Document>,
}

impl SketchDDDBackend {
    /// Create a new backend instance
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
        }
    }

    /// Get a document by URI
    #[allow(dead_code)]
    pub fn get_document(&self, uri: &Url) -> Option<dashmap::mapref::one::Ref<'_, Url, Document>> {
        self.documents.get(uri)
    }

    /// Update a document and publish diagnostics
    async fn update_document(&self, uri: Url, text: String, version: i32) {
        let document = Document::new(uri.clone(), text, version);
        let diagnostics = publish_diagnostics(&document);
        self.documents.insert(uri.clone(), document);
        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for SketchDDDBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        tracing::info!("Initializing SketchDDD Language Server");

        Ok(InitializeResult {
            capabilities: server_capabilities(),
            server_info: Some(ServerInfo {
                name: "sketchddd-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("SketchDDD Language Server initialized");
        self.client
            .log_message(MessageType::INFO, "SketchDDD Language Server ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down SketchDDD Language Server");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        tracing::debug!("Document opened: {}", params.text_document.uri);
        self.update_document(
            params.text_document.uri,
            params.text_document.text,
            params.text_document.version,
        )
        .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        tracing::debug!("Document changed: {}", params.text_document.uri);

        // We request full document sync, so there should be exactly one change
        if let Some(change) = params.content_changes.into_iter().next() {
            self.update_document(
                params.text_document.uri,
                change.text,
                params.text_document.version,
            )
            .await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        tracing::debug!("Document saved: {}", params.text_document.uri);

        // Re-validate on save if we have the text
        if let Some(text) = params.text {
            if let Some(doc) = self.documents.get(&params.text_document.uri) {
                self.update_document(params.text_document.uri.clone(), text, doc.version).await;
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        tracing::debug!("Document closed: {}", params.text_document.uri);
        self.documents.remove(&params.text_document.uri);
        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(uri) {
            Ok(provide_hover(&doc, position))
        } else {
            Ok(None)
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(uri) {
            Ok(Some(CompletionResponse::Array(provide_completions(
                &doc, position,
            ))))
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(location) = doc.find_definition(position) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }
        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(uri) {
            let refs = doc.find_references(position);
            if !refs.is_empty() {
                return Ok(Some(refs));
            }
        }
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            let symbols = document_symbols(&doc);
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        } else {
            Ok(None)
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(formatted) = doc.format() {
                let edit = TextEdit {
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(u32::MAX, u32::MAX),
                    },
                    new_text: formatted,
                };
                return Ok(Some(vec![edit]));
            }
        }
        Ok(None)
    }
}
