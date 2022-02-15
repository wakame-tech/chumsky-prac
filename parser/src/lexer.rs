use chumsky::prelude::*;

use crate::{tokens::*, Span};

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // A parser for strings
    // let str_ = just('"')
    //     .ignore_then(filter(|c| *c != '"').repeated())
    //     .then_ignore(just('"'))
    //     .collect::<String>()
    //     .map(Token::Str);

    // A parser for operators
    let op = just("+")
        .or(just("-"))
        .or(just("*"))
        .or(just("/"))
        .or(just("%"))
        .or(just("=="))
        .or(just("="))
        .or(just("<"))
        .or(just(">"))
        .map(|c| Token::Op(c.to_string()));

    // A parser for control characters (delimiters, semicolons, etc.)
    let ctrl = one_of("()[]{};,:").map(|c| Token::Ctrl(c));

    // A parser for identifiers and keywords
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "fn" => Token::Fn,
        "var" => Token::Var,
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        // "true" => Token::Bool(true),
        // "false" => Token::Bool(false),
        // "null" => Token::Null,
        _ => Token::Ident(ident),
    });

    let comment = just("//")
        .then(take_until(just('\n')))
        .padded()
        .map(|(a, (b, _))| Token::Comment(format!("{}{}", a, b.into_iter().collect::<String>())));

    // A parser for numbers
    let num = text::int(10).map(Token::Num);

    // A single token can be one of the above
    let token = comment
        .or(num)
        // .or(str_)
        .or(op)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    token
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

///
/// do lex
///
pub fn lex(src: &str) -> (Option<Vec<(Token, Span)>>, Vec<Simple<char>>) {
    let (tokens, lex_errs) = lexer().parse_recovery(src);
    (tokens, lex_errs)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{lexer::lex, tokens::Token};

    #[test]
    fn it_works() {
        assert_eq!(
            lex("+"),
            (Some(vec![(Token::Op("+".to_string()), 0..1)]), vec![])
        );

        assert_eq!(
            lex("=="),
            (Some(vec![(Token::Op("==".to_string()), 0..2)]), vec![])
        );

        assert_eq!(
            lex("// a\n"),
            (
                Some(vec![(Token::Comment("// a".to_string()), 0..5)]),
                vec![]
            )
        );

        assert_eq!(
            lex("// a\n1"),
            (
                Some(vec![
                    (Token::Comment("// a".to_string()), 0..5),
                    (Token::Num("1".to_string()), 5..6)
                ]),
                vec![]
            )
        )
    }
}
