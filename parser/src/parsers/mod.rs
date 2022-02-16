use interface::{nodes::func::Func, tokens::Token, Span, Spanned};
use std::{collections::HashMap, vec::IntoIter};

use chumsky::{prelude::Simple, Parser, Stream};

use self::funcs::funcs_parser;

pub mod expr;
pub mod funcs;

///
/// do parse
///
pub fn parse(
    token_stream: Stream<Token, Span, IntoIter<Spanned<Token>>>,
) -> (Option<HashMap<String, Func>>, Vec<Simple<Token>>) {
    funcs_parser().parse_recovery(token_stream)
}
