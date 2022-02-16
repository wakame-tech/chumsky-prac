use language_server::Backend;
use logger::init_logger;
use tower_lsp::{LspService, Server};

mod analyzer;
mod globals;
mod language_server;
mod logger;

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
