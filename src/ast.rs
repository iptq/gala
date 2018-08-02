//! Abstract Syntax Tree

use literal::Literal;

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
