use interface::{
    nodes::{binaryop::BinaryOp, expr::Expr, value::Value},
    tokens::Token,
    Spanned,
};

use chumsky::prelude::*;

pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let raw_expr = recursive(|raw_expr| {
            let val = filter_map(|span, tok| match tok {
                // Token::Null => Ok(Expr::Value(Value::Null)),
                // Token::Bool(x) => Ok(Expr::Value(Value::Bool(x))),
                Token::Num(n) => Ok(Expr::Value(Value::I32(n.parse().unwrap()))),
                // Token::Str(s) => Ok(Expr::Value(Value::Str(s))),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
            })
            .labelled("value");

            let ident = filter_map(|span, tok| match tok {
                Token::Ident(ident) => Ok(ident.clone()),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(tok))),
            })
            .labelled("identifier");

            // // A list of expressions
            let items = expr
                .clone()
                .chain(just(Token::Ctrl(',')).ignore_then(expr.clone()).repeated())
                .then_ignore(just(Token::Ctrl(',')).or_not())
                .or_not()
                .map(|item| item.unwrap_or_else(Vec::new));

            // A let expression
            let let_ = just(Token::Var)
                .ignore_then(ident)
                .then_ignore(just(Token::Op("=".to_string())))
                .then(raw_expr)
                .then_ignore(just(Token::Ctrl(';')))
                .then(expr.clone())
                .map(|((name, val), body)| Expr::Var(name, Box::new(val), Box::new(body)));

            let r#return = just(Token::Return)
                .ignore_then(expr.clone())
                .map(|expr| Expr::Return(Box::new(expr)));

            // let list = items
            //     .clone()
            //     .delimited_by(just(Token::Ctrl('[')), just(Token::Ctrl(']')))
            //     .map(Expr::List);

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Local))
                .or(let_)
                // .or(list)
                .or(r#return)
                .map_with_span(|expr, span| (expr, span))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('('),
                    Token::Ctrl(')'),
                    [
                        (Token::Ctrl('['), Token::Ctrl(']')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                ))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(nested_delimiters(
                    Token::Ctrl('['),
                    Token::Ctrl(']'),
                    [
                        (Token::Ctrl('('), Token::Ctrl(')')),
                        (Token::Ctrl('{'), Token::Ctrl('}')),
                    ],
                    |span| (Expr::Error, span),
                ));

            // Function calls have very high precedence so we prioritise them
            let call = atom
                .then(
                    items
                        .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                        .map_with_span(|args, span| (args, span))
                        .repeated(),
                )
                .foldl(|f, args| {
                    let span = f.1.start..args.1.end;
                    (Expr::Call(Box::new(f), args), span)
                });

            // * | / | %
            let op = just(Token::Op("*".to_string()))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/".to_string())).to(BinaryOp::Div))
                .or(just(Token::Op("%".to_string())).to(BinaryOp::Mod));
            let product = call
                .clone()
                .then(op.then(call).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // + | -
            let op = just(Token::Op("+".to_string()))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-".to_string())).to(BinaryOp::Sub));
            let sum = product
                .clone()
                .then(op.then(product).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // > | < | <= | >=
            let op = just(Token::Op("<".to_string()))
                .to(BinaryOp::Lt)
                .or(just(Token::Op(">".to_string())).to(BinaryOp::Gt))
                .or(just(Token::Op("<=".to_string())).to(BinaryOp::Leq))
                .or(just(Token::Op(">=".to_string())).to(BinaryOp::Geq));
            let relational = sum
                .clone()
                .then(op.then(sum).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::Op("==".to_string()))
                .to(BinaryOp::Eq)
                .or(just(Token::Op("!=".to_string())).to(BinaryOp::Neq));
            let compare = relational
                .clone()
                .then(op.then(relational).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // &&
            let op = just(Token::Op("&&".to_string())).to(BinaryOp::And);
            let and = compare
                .clone()
                .then(op.then(compare).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            // ||
            let op = just(Token::Op("||".to_string())).to(BinaryOp::Or);
            let or = and
                .clone()
                .then(op.then(and).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Binary(Box::new(a), op, Box::new(b)), span)
                });

            or

            // let r#return = just(Token::Return)
            //     .ignore_then(or.clone())
            //     .then(or.c)
            //     .then_ignore(just(Token::Ctrl(';')))
            //     .map(|(ret, body)| {
            //         let span = ret.1.start..body.1.end;
            //         (Expr::Return(Box::new(body)), span)
            //     });
            // r#return
        });

        // Blocks are expressions but delimited with braces
        let block = expr
            .clone()
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            // Attempt to recover anything that looks like a block but contains errors
            .recover_with(nested_delimiters(
                Token::Ctrl('{'),
                Token::Ctrl('}'),
                [
                    (Token::Ctrl('('), Token::Ctrl(')')),
                    (Token::Ctrl('['), Token::Ctrl(']')),
                ],
                |span| (Expr::Error, span),
            ));

        let if_ = recursive(|if_| {
            just(Token::If)
                .ignore_then(expr.clone())
                .then(block.clone())
                .then(
                    just(Token::Else)
                        .ignore_then(block.clone().or(if_))
                        .or_not(),
                )
                .map_with_span(|((cond, a), b), span| {
                    (
                        Expr::If(
                            Box::new(cond),
                            Box::new(a),
                            Box::new(match b {
                                Some(b) => b,
                                // If an `if` expression has no trailing `else` block, we magic up one that just produces null
                                None => (Expr::Value(Value::Null), span.clone()),
                            }),
                        ),
                        span,
                    )
                })
        });

        // Both blocks and `if` are 'block expressions' and can appear in the place of statements
        let block_expr = block.or(if_).labelled("block");

        let block_chain = block_expr
            .clone()
            .then(block_expr.clone().repeated())
            .foldl(|a, b| {
                let span = a.1.start..b.1.end;
                (Expr::Then(Box::new(a), Box::new(b)), span)
            });

        block_chain
            // Expressions, chained by semicolons, are statements
            .or(raw_expr.clone())
            .then(just(Token::Ctrl(';')).ignore_then(expr.or_not()).repeated())
            .foldl(|a, maybe_b| {
                if let Some(b) = maybe_b {
                    let span = a.1.start..b.1.end;
                    (Expr::Then(Box::new(a), Box::new(b)), span)
                } else {
                    a
                }
            })
    })
}
