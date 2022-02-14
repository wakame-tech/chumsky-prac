use chumsky::{prelude::*};

use crate::{tokens::*, Span};

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // A parser for numbers
    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Num);

    // A parser for strings
    // let str_ = just('"')
    //     .ignore_then(filter(|c| *c != '"').repeated())
    //     .then_ignore(just('"'))
    //     .collect::<String>()
    //     .map(Token::Str);

    // A parser for operators
    let op = one_of("+-*/!=")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Op);

    // A parser for control characters (delimiters, semicolons, etc.)
    let ctrl = one_of("()[]{};,").map(|c| Token::Ctrl(c));

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "let" => Token::Let,
        // "print" => Token::Print,
        "if" => Token::If,
        "else" => Token::Else,
        // "true" => Token::Bool(true),
        // "false" => Token::Bool(false),
        // "null" => Token::Null,
        _ => Token::Ident(ident),
    });

    // A single token can be one of the above
    let token = num
        // .or(str_)
        .or(op)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .padded_by(comment.repeated())
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}
