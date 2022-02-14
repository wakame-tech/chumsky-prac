use std::{env, fs};
use ariadne::{Report, ReportKind, Color, Label, Source, Fmt};

mod tokens;
mod nodes;
mod lexer;
mod parsers;
mod interpreter;

use chumsky::{Parser, Stream, prelude::Simple};
use lexer::lexer;
use parsers::funcs::funcs_parser;
use tokens::Token;
use interpreter::eval_expr;

pub type Span = std::ops::Range<usize>;

fn report_errs(src: &String, lex_errs: Vec<Simple<char>>, parse_errs: Vec<Simple<Token>>) {
    lex_errs.into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .chain(parse_errs.into_iter().map(|e| e.map(|tok| tok.to_string())))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");
    let (tokens, mut lex_errs) = lexer()
        .parse_recovery(src.as_str());
    let parse_errs = if let Some(tokens) = tokens {
        // println!("Tokens = {:?}", tokens);

        let len = src.chars().count();
        let (ast, parse_errs) =
            funcs_parser()
                .parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));
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