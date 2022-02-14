use crate::parsers::Spanned;

use super::expr::Expr;

// A function node in the AST.
#[derive(Debug)]
pub struct Func {
    pub args: Vec<String>,
    pub body: Spanned<Expr>,
}