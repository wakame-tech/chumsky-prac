use parser::lexer::lex;
use tower_lsp::lsp_types::{Diagnostic, Position, Range, SemanticToken};

use crate::{
    analyzer::{diagnostics::to_diagnostics, semantic_tokens::to_token_type},
    globals::TOKEN_TYPES,
};

use self::semantic_tokens::to_semantic_tokens;

mod diagnostics;
mod semantic_tokens;

#[derive(Debug, Clone)]
pub struct RangedTokenType {
    /// token type index
    pub token_type: u32,
    /// token location
    pub range: Range,
}

/// convert source to tokens and diagnostics
/// TODO: ASTを作っていないので全然semanticじゃない
/// TODO: line毎にlexしている
pub fn analyze_src(src: String) -> (Vec<SemanticToken>, Vec<Diagnostic>) {
    let map = &mut TOKEN_TYPES.lock().unwrap();
    let mut diagnostics: Vec<Diagnostic> = vec![];
    let mut ranged_types: Vec<RangedTokenType> = vec![];
    src.lines().into_iter().enumerate().for_each(|(i, line)| {
        let (tokens, errs) = lex(format!("{}\n", line).as_str());
        log::debug!("{:?}", errs);
        let diag_errs = errs
            .iter()
            .map(|err| to_diagnostics(i, err))
            .collect::<Vec<_>>();
        let types = tokens
            .unwrap_or(vec![])
            .into_iter()
            .filter_map(|(tok, pos)| {
                if let Some(key) = to_token_type(&tok) {
                    Some(RangedTokenType {
                        range: Range {
                            start: Position {
                                line: i as u32,
                                character: pos.start as u32,
                            },
                            end: Position {
                                line: i as u32,
                                character: pos.end as u32,
                            },
                        },
                        token_type: *map.get(&key).unwrap(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        ranged_types.extend(types);
        diagnostics.extend(diag_errs);
    });

    ranged_types.iter().for_each(|t| {
        log::debug!(
            "{} {}:{}-{}:{}",
            t.token_type,
            t.range.start.line,
            t.range.start.character,
            t.range.end.line,
            t.range.end.character
        )
    });
    let semantic_tokens = to_semantic_tokens(ranged_types);
    (semantic_tokens, diagnostics)
}
