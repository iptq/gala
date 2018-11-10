#[derive(Debug)]
pub struct Program(pub Vec<TopDecl>);

#[derive(Debug)]
pub enum TopDecl {
    Fn(String, Vec<Stmt>),
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Name(String),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>),
    Return(Option<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(u32),
}

#[derive(Debug)]
pub enum Type {

}
