#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Or,
    And,
    Lte,
    Gte,
    Lt,
    Gt,
    Eq,
    Neq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
