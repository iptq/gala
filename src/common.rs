#[derive(Debug)]
pub enum Type {
    T(u32),
    Int,
}

impl Type {
    pub fn ir_repr(&self) -> impl AsRef<str> {
        match self {
            Type::T(_) => panic!("Should not encounter untyped variables in a typed tree."),
            Type::Int => "u32",
        }
    }
}

pub trait Typed {
    fn get_type(&self) -> Type;
}

#[derive(Debug)]
pub enum Literal {
    Int(u32),
}

impl Typed for Literal {
    fn get_type(&self) -> Type {
        match self {
            Literal::Int(_) => Type::Int,
        }
    }
}
