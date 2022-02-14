use crate::Span;

pub mod expr;
pub mod funcs;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}
