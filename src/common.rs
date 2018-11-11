#[derive(Debug)]
pub enum Type {
    T(u32),
    Int,
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
