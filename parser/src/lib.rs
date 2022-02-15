pub mod interpreter;
pub mod lexer;
pub mod nodes;
pub mod parsers;
pub mod tokens;

pub type Span = std::ops::Range<usize>;
