#[derive(Clone, Debug)]
pub enum BinaryOp {
    Or,  // ||
    And, // &&
    Eq,  // ==
    Neq, // !=
    Geq, // >=
    Leq, // <=
    Gt,  // >
    Lt,  // <
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
}
