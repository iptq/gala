//! Abstract Syntax Tree

use literal::Literal;
use op::Op;

#[derive(Clone, Debug)]
pub struct BinOp {
    pub left: Box<Expr>,
    pub op: Op,
    pub right: Box<Expr>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Fn(Function),
    Enum(Enum),
}

#[derive(Clone, Debug)]
pub enum Enum {}

#[derive(Clone, Debug)]
pub enum Expr {
    Fn(Function),
    Lit(Literal),
    BinOp(BinOp),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Option<String>,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Lit(Literal),
    Expr(Expr),
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        match value {
            Value::Lit(lit) => Expr::Lit(lit),
            Value::Expr(expr) => expr,
        }
    }
}
