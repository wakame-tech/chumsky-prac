pub mod interpreter;
pub mod lexer;
pub mod nodes;
pub mod parsers;
pub mod tokens;

/// represents a position range in a source file
pub type Span = std::ops::Range<usize>;
