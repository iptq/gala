use symbol::Symbol;

#[derive(Clone, Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Fn(Function),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal,
}
