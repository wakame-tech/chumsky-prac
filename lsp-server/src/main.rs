use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::analyzer::analyze_src;

mod analyzer;

fn init_logger() {
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let log_path = format!("lsp-{}.log", timestamp);
    let path = Path::new(&log_path);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        file,
    )])
    .unwrap();
    log::debug!("logger initialized");
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::debug!("initialize: {:?}", params);

        // semantic tokens
        let legend = params
            .capabilities
            .text_document
            .unwrap()
            .semantic_tokens
            .unwrap();

        log::debug!("token_types: {:?}", &legend.token_types);

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
                                token_types: legend.token_types,
                                token_modifiers: vec![],
                            },
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
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

        let tokens = analyze_src(params.text_document.uri, params.text_document.text);
        log::debug!("tokens: {:?}", tokens);
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
        let tokens: Vec<SemanticToken> = vec![];
        dbg!(&params);
        let uri = params.text_document.uri.to_string();
        let mut f = File::open(&uri).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        dbg!(&contents);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: Some("".to_string()),
            data: tokens,
        })))
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        log::debug!("semantic_tokens_full_delta: {:?}", &params);
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

#[tokio::main]
async fn main() {
    init_logger();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
