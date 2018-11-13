use common::{Arg, Literal, Type, Typed};

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
    Extern(String, Type),
    Fn(String, Vec<Arg>, Type, Vec<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(bool, String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Call(String, Vec<Expr>, Type),
    Literal(Literal, Type),
    Name(String, Type),
    NotEquals(Box<Expr>, Box<Expr>, Type),
    Equals(Box<Expr>, Box<Expr>, Type),
    Plus(Box<Expr>, Box<Expr>, Type),
    Minus(Box<Expr>, Box<Expr>, Type),
    Times(Box<Expr>, Box<Expr>, Type),
}

impl Typed for Expr {
    fn get_type(&self) -> Type {
        match self {
            Expr::Call(_, _, t)
            | Expr::Literal(_, t)
            | Expr::Name(_, t)
            | Expr::NotEquals(_, _, t)
            | Expr::Equals(_, _, t)
            | Expr::Plus(_, _, t)
            | Expr::Minus(_, _, t)
            | Expr::Times(_, _, t) => t.clone(),
        }
    }
}
