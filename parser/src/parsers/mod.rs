use std::{collections::HashMap, vec::IntoIter};

use chumsky::{prelude::Simple, Parser, Stream};

use crate::{nodes::func::Func, tokens::Token, Span};

use self::funcs::funcs_parser;

pub mod expr;
pub mod funcs;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}

///
/// do parse
///
pub fn parse(
    token_stream: Stream<Token, Span, IntoIter<Spanned<Token>>>,
) -> (Option<HashMap<String, Func>>, Vec<Simple<Token>>) {
    funcs_parser().parse_recovery(token_stream)
}
