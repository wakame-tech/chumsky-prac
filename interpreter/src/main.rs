use std::{env, fs};

use chumsky::{prelude::Simple, Stream};
use error_reporter::report_errs;
use interface::tokens::Token;
use parser::{lexer::lex, parsers::parse};

use crate::interpreter::eval_expr;

mod error_reporter;
mod interpreter;

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");
    let (tokens, mut lex_errs) = lex(&src);

    let parse_errs = if let Some(tokens) = tokens {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.0, Token::Comment(_)))
            .collect::<Vec<_>>();
        tokens.iter().for_each(|t| println!("{} {:?}", t.0, t.1));

        let len = src.chars().count();
        let token_stream = Stream::from_iter(len..len + 1, tokens.into_iter());
        let (ast, parse_errs) = parse(token_stream);

        println!("{:#?}", ast);

        if let Some(funcs) = ast.filter(|_| lex_errs.len() + parse_errs.len() == 0) {
            if let Some(main) = funcs.get("main") {
                assert_eq!(main.args.len(), 0);
                match eval_expr(&main.body, &funcs, &mut Vec::new()) {
                    Ok(val) => println!("Return value: {}", val),
                    Err(e) => lex_errs.push(Simple::custom(e.span, e.msg)),
                }
            } else {
                panic!("No main function!");
            }
        }

        parse_errs
    } else {
        Vec::new()
    };
    report_errs(&src, lex_errs, parse_errs);
}
