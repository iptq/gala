use symbol::Symbol;

#[derive(Clone, Debug)]
pub struct Module {
    pub decls: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Fn,
}

#[derive(Clone, Debug)]
pub enum Expr {}
