pub mod nodes;
pub mod tokens;

/// represents a position range in a source file
pub type Span = std::ops::Range<usize>;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub span: Span,
    pub msg: String,
}
