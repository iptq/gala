#[derive(Clone, Debug)]
pub enum Type {
    T(u32),
    Bool,
    Int,
    String,
}

impl Type {
    pub fn ir_repr(&self) -> impl AsRef<str> {
        match self {
            Type::T(_) => "i32", // panic!("Should not encounter untyped variables in a typed tree."),
            Type::Bool => "i1",
            Type::Int => "i32",
            Type::String => "i8*", // lol
        }
    }
}

#[derive(Debug)]
pub struct Arg(pub String, pub Type);

pub trait Typed {
    fn get_type(&self) -> Type;
}

#[derive(Debug)]
pub enum Literal {
    Int(u32),
    String(String),
}

impl Typed for Literal {
    fn get_type(&self) -> Type {
        match self {
            Literal::Int(_) => Type::Int,
            Literal::String(_) => Type::String,
        }
    }
}
