use common::{Literal, Type};

pub struct Context {
    inc: u32,
}

impl Context {
    pub fn new() -> Self {
        Context { inc: 0 }
    }
    pub fn next(&mut self) -> Type {
        let result = self.inc;
        self.inc += 1;
        Type::T(result)
    }
}

pub trait IntoMir<T> {
    fn into_mir(self, &mut Context) -> T;
}

#[derive(Debug)]
pub struct Program(pub Vec<TopDecl>);

#[derive(Debug)]
pub enum TopDecl {
    Fn(String, Type, Vec<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>),
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal, Type),
    Name(String, Type),
    Plus(Box<Expr>, Box<Expr>, Type),
    Minus(Box<Expr>, Box<Expr>, Type),
}
