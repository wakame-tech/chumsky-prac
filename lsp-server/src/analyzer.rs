use chumsky::prelude::Simple;
use itertools::Itertools;
use parser::{lexer::lex, tokens::Token};
use tower_lsp::lsp_types::{Diagnostic, Position, Range, SemanticToken};

use crate::TOKEN_TYPES;

#[derive(Debug, Clone)]
struct RangedTokenType {
    /// token type index
    token_type: u32,
    /// token location
    range: Range,
}

/// TODO: lexer 側で提供する
impl RangedTokenType {
    /// initial semantic token
    fn semantic_token(&self) -> SemanticToken {
        SemanticToken {
            delta_line: self.range.start.line,
            delta_start: self.range.start.character,
            length: self.range.end.character - self.range.start.character,
            token_type: self.token_type,
            token_modifiers_bitset: 0,
        }
    }

    /// delta semantic token
    fn semantic_token_from(&self, pre: RangedTokenType) -> SemanticToken {
        SemanticToken {
            delta_line: self.range.end.line - pre.range.start.line,
            delta_start: if pre.range.end.line != self.range.start.line {
                self.range.start.character
            } else {
                self.range
                    .start
                    .character
                    .checked_sub(pre.range.start.character)
                    .unwrap_or(self.range.start.character)
            },
            length: self
                .range
                .end
                .character
                .checked_sub(self.range.start.character)
                .unwrap_or(self.range.end.character),
            token_type: self.token_type,
            token_modifiers_bitset: 0,
        }
    }
}

fn to_token_type(token: &Token) -> Option<String> {
    match token {
        Token::Var | Token::Fn | Token::If | Token::Else | Token::Return => {
            Some("keyword".to_string())
        }
        Token::Ident(_) => Some("variable".to_string()),
        Token::Num(_) => Some("number".to_string()),
        Token::Op(_) => Some("operator".to_string()),
        Token::Comment(_) => Some("comment".to_string()),
        _ => None,
    }
}

fn to_diagnostics(line: usize, err: &Simple<char>) -> Diagnostic {
    let span = err.span();
    let expected = err.expected().filter_map(|s| *s).join(", ");
    let found = err.found().unwrap();
    Diagnostic {
        range: Range {
            start: Position {
                line: line as u32,
                character: span.start as u32,
            },
            end: Position {
                line: line as u32,
                character: span.end as u32,
            },
        },
        severity: None,
        code: None,
        code_description: None,
        source: Some("source".to_string()),
        message: format!("expected {:?} but {:?}", expected, found).to_string(),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// calc relative position to create semantic tokens
fn to_semantic_tokens(types: Vec<RangedTokenType>) -> Vec<SemanticToken> {
    vec![
        vec![types[0].semantic_token()],
        types
            .into_iter()
            .tuple_windows()
            .map(|(pre, cur)| cur.semantic_token_from(pre))
            .collect::<Vec<_>>(),
    ]
    .concat()
}

/// convert source to tokens and diagnostics
/// TODO: ASTを作っていないので全然semanticじゃない
/// TODO: line毎にlexしている
pub fn analyze_src(src: String) -> Vec<SemanticToken> {
    let map = &mut TOKEN_TYPES.lock().unwrap();
    let mut diagnostics: Vec<Diagnostic> = vec![];
    let mut ranged_types: Vec<RangedTokenType> = vec![];
    src.lines().into_iter().enumerate().for_each(|(i, line)| {
        let (tokens, errs) = lex(format!("{}\n", line).as_str());
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
    log::debug!("{:?}", semantic_tokens);
    semantic_tokens
}
