use std::fs::File;
use std::io::prelude::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

fn init_logger() {
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("lsp.log").unwrap(),
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
        let mut result = InitializeResult::default();
        // sync
        result.capabilities.text_document_sync =
            Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full));
        // completion
        result.capabilities.completion_provider = Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: None,
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
        });
        // highlight
        result.capabilities.semantic_tokens_provider = Some(
            SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: vec![SemanticTokenType::VARIABLE.into()],
                    token_modifiers: vec![],
                },
                range: Some(false),
                full: Some(SemanticTokensFullOptions::Bool(true)),
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
            }),
        );
        Ok(result)
    }

    async fn initialized(&self, params: InitializedParams) {
        log::debug!("initialize: {:?}", &params);
        log::debug!("server initialized");

        self.client
            .log_message(MessageType::Info, format!("{:?}", &params))
            .await;
        self.client
            .log_message(MessageType::Info, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        log::debug!("shutdown");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        log::debug!("did_open: {:?}", &params);
        self.client
            .log_message(MessageType::Info, format!("{:?}", &params))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        log::debug!("did_change: {:?}", &params);
        self.client
            .log_message(MessageType::Info, format!("{:?}", &params))
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::debug!("completion: {:?}", &params);
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
        let mut tokens: Vec<SemanticToken> = vec![];
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
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
    //     .await
    //     .unwrap();
    // let (stream, _) = listener.accept().await.unwrap();
    // let (read, write) = tokio::io::split(stream);

    // Server::new(read, write)
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
