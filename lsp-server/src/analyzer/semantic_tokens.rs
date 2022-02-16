use interface::tokens::Token;

use super::RangedTokenType;
use itertools::Itertools;
use tower_lsp::lsp_types::SemanticToken;

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

pub fn to_token_type(token: &Token) -> Option<String> {
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

/// calc relative position to create semantic tokens
pub fn to_semantic_tokens(types: Vec<RangedTokenType>) -> Vec<SemanticToken> {
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
