use std::collections::HashMap;

use crate::{
    nodes::{binaryop::BinaryOp, expr::Expr, func::Func, value::Value},
    parsers::{Error, Spanned},
};

pub fn eval_expr(
    expr: &Spanned<Expr>,
    funcs: &HashMap<String, Func>,
    stack: &mut Vec<(String, Value)>,
) -> Result<Value, Error> {
    Ok(match &expr.0 {
        Expr::Error => unreachable!(), // Error expressions only get created by parser errors, so cannot exist in a valid AST
        Expr::Value(val) => val.clone(),
        // Expr::List(items) => Value::List(
        //     items
        //         .iter()
        //         .map(|item| eval_expr(item, funcs, stack))
        //         .collect::<Result<_, _>>()?,
        // ),
        Expr::Local(name) => stack
            .iter()
            .rev()
            .find(|(l, _)| l == name)
            .map(|(_, v)| v.clone())
            .or_else(|| {
                Some(Value::Func(name.clone()))
                    .filter(|_| funcs.contains_key(name) || name == "print")
            })
            .ok_or_else(|| Error {
                span: expr.1.clone(),
                msg: format!("No such variable '{}' in scope", name),
            })?,
        Expr::Var(local, val, body) => {
            let val = eval_expr(val, funcs, stack)?;
            stack.push((local.clone(), val));
            let res = eval_expr(body, funcs, stack)?;
            stack.pop();
            res
        }
        Expr::Then(a, b) => {
            eval_expr(a, funcs, stack)?;
            eval_expr(b, funcs, stack)?
        }
        // +
        Expr::Binary(a, BinaryOp::Add, b) => Value::I32(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                + eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // -
        Expr::Binary(a, BinaryOp::Sub, b) => Value::I32(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                - eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // *
        Expr::Binary(a, BinaryOp::Mul, b) => Value::I32(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                * eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // /
        Expr::Binary(a, BinaryOp::Div, b) => Value::I32(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                / eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // ==
        Expr::Binary(a, BinaryOp::Eq, b) => {
            Value::Bool(eval_expr(a, funcs, stack)? == eval_expr(b, funcs, stack)?)
        }
        // !=
        Expr::Binary(a, BinaryOp::Neq, b) => {
            Value::Bool(eval_expr(a, funcs, stack)? != eval_expr(b, funcs, stack)?)
        }
        // >
        Expr::Binary(a, BinaryOp::Gt, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                > eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // >=
        Expr::Binary(a, BinaryOp::Geq, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                >= eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // <
        Expr::Binary(a, BinaryOp::Lt, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                < eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // <=
        Expr::Binary(a, BinaryOp::Leq, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                <= eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        // &&
        Expr::Binary(a, BinaryOp::And, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.bool(a.1.clone())?
                && eval_expr(b, funcs, stack)?.bool(b.1.clone())?,
        ),
        // ||
        Expr::Binary(a, BinaryOp::Or, b) => Value::Bool(
            eval_expr(a, funcs, stack)?.bool(a.1.clone())?
                || eval_expr(b, funcs, stack)?.bool(b.1.clone())?,
        ),
        // %
        Expr::Binary(a, BinaryOp::Mod, b) => Value::I32(
            eval_expr(a, funcs, stack)?.num_i32(a.1.clone())?
                % eval_expr(b, funcs, stack)?.num_i32(b.1.clone())?,
        ),
        Expr::Call(func, (args, args_span)) => {
            let f = eval_expr(func, funcs, stack)?;
            match f {
                Value::Func(name) => {
                    if name == "print" {
                        let evaled_args = args
                            .iter()
                            .map(|arg| eval_expr(arg, funcs, stack).unwrap())
                            .collect::<Vec<Value>>();
                        dbg!(evaled_args);
                        return Ok(Value::Null);
                    }

                    let f = &funcs[&name];
                    let mut stack = if f.args.len() != args.len() {
                        return Err(Error {
                            span: args_span.clone(),
                            msg: format!("'{}' called with wrong number of arguments (expected {}, found {})", name, f.args.len(), args.len()),
                        });
                    } else {
                        f.args
                            .iter()
                            .zip(args.iter())
                            .map(|(name, arg)| Ok((name.clone(), eval_expr(arg, funcs, stack)?)))
                            .collect::<Result<_, _>>()?
                    };
                    eval_expr(&f.body, funcs, &mut stack)?
                }
                f => {
                    return Err(Error {
                        span: func.1.clone(),
                        msg: format!("'{:?}' is not callable", f),
                    })
                }
            }
        }
        Expr::If(cond, a, b) => {
            let c = eval_expr(cond, funcs, stack)?;
            match c {
                Value::Bool(true) => eval_expr(a, funcs, stack)?,
                Value::Bool(false) => eval_expr(b, funcs, stack)?,
                c => {
                    return Err(Error {
                        span: cond.1.clone(),
                        msg: format!("Conditions must be booleans, found '{:?}'", c),
                    })
                }
            }
        }
        Expr::Return(a) => {
            let val = eval_expr(a, funcs, stack)?;
            println!("{}", val);
            val
        }
    })
}
