use std::fs::File;
use std::io::prelude::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::analyzer::analyze_src;
use crate::globals::TOKEN_TYPES;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // log::debug!("initialize: {:?}", params);

        // semantic tokens
        let legend = params
            .capabilities
            .text_document
            .unwrap()
            .semantic_tokens
            .unwrap();
        let types = legend.token_types;

        log::debug!("token_types: {:?}", &types);
        log::debug!("token_modifiers: {:?}", &legend.token_modifiers);

        let map = &mut TOKEN_TYPES.lock().unwrap();
        for (i, token) in types.iter().enumerate() {
            map.insert(token.as_str().to_owned(), i as u32);
        }

        return Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: None,
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: types,
                                token_modifiers: legend.token_modifiers,
                            },
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(false) }),
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: Some(true),
                            },
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
        });
    }

    async fn initialized(&self, params: InitializedParams) {
        log::debug!("server initialized");

        // self.client
        //     .log_message(MessageType::Info, "server initialized!")
        //     .await;
    }

    async fn shutdown(&self) -> Result<()> {
        log::debug!("shutdown");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        log::debug!("did_open");
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        log::debug!("did_close: {:?}", &params.text_document.uri);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        log::debug!("did_change: {:?}", &params.text_document.uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::info!("completion: {:?}", &params);
        self.client
            .log_message(MessageType::Info, format!("{:?}", &params))
            .await;

        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        log::debug!("semantic_tokens_full: {:?}", &params);
        // let tokens: Vec<SemanticToken> = vec![];
        let path = params.text_document.uri.to_file_path().unwrap();
        log::debug!("open file: {:?}, exists: {:?}", path, path.exists());

        if let Ok(mut f) = File::open(&path) {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            let (data, diagnostics) = analyze_src(contents);

            log::debug!("diagnostics: {:?}", diagnostics);

            self.client
                .publish_diagnostics(params.text_document.uri.clone(), diagnostics, None)
                .await;

            return Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: Some(params.text_document.uri.to_string()),
                data,
            })));
        }
        Ok(None)
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        log::debug!("hover");
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }
}
