use itertools::Itertools;
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
    let tokens = src
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let (tokens, _) = lex(format!("{}\n", line).as_str());
            tokens
                .unwrap_or(vec![])
                .into_iter()
                .filter_map(|(tok, pos)| {
                    if let Some(key) = to_token_type(&tok) {
                        Some((i as u32, pos, key))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    tokens.iter().for_each(|t| log::debug!("{:?}", t));

    // calc relative position
    let semantic_tokens = vec![
        vec![SemanticToken {
            delta_line: tokens[0].0,
            delta_start: tokens[0].1.start as u32,
            length: tokens[0].1.len() as u32,
            token_type: *map.get(&tokens[0].2).unwrap(),
            token_modifiers_bitset: 0,
        }],
        tokens
            .into_iter()
            .tuple_windows()
            .map(|(pre, cur)| SemanticToken {
                delta_line: cur.0 - pre.0,
                delta_start: if cur.0 != pre.0 {
                    cur.1.start as u32
                } else {
                    (cur.1.start - pre.1.start) as u32
                },
                length: cur.1.len() as u32,
                token_type: *map.get(&cur.2).unwrap(),
                token_modifiers_bitset: 0,
            })
            .collect::<Vec<_>>(),
    ]
    .concat();

    log::debug!("{:?}", semantic_tokens);

    semantic_tokens
}
