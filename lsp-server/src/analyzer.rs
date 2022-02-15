use parser::{lexer::lex, tokens::Token};
use tower_lsp::lsp_types::SemanticToken;

use crate::TOKEN_TYPES;

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

///
/// convert source to tokens and diagnostics
///
pub fn analyze_src(src: String) -> Vec<SemanticToken> {
    let map = &mut TOKEN_TYPES.lock().unwrap();
    let data = src
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let (tokens, _) = lex(line);
            log::debug!("{:?}", &tokens);
            tokens
                .unwrap_or(vec![])
                .into_iter()
                .filter_map(|(tok, pos)| {
                    if let Some(key) = to_token_type(&tok) {
                        Some(SemanticToken {
                            delta_line: i as u32,
                            delta_start: pos.start as u32,
                            length: pos.len() as u32,
                            token_type: *map.get(&key).unwrap(),
                            token_modifiers_bitset: 0,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    data
}
