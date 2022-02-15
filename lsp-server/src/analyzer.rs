use parser::{lexer::lex, parsers::Spanned, tokens::Token};
use tower_lsp::lsp_types::Url;
///
/// convert source to tokens and diagnostics
///
pub fn analyze_src(uri: Url, src: String) -> Vec<Spanned<Token>> {
    let (tokens, _) = lex(src.as_str());
    tokens.unwrap_or(vec![])
}
