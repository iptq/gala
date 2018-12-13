use std::sync::{Arc, Mutex};
use typeck::Substitution;

lazy_static! {
    static ref counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

pub fn next_int() -> u32 {
    let mut c = counter.lock().unwrap();
    *c += 1;
    *c
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Type {
    T(u32),
    Fn(Vec<Type>, Box<Type>), // list of args -> type, maybe will use tuple later
    Bool,
    Int,
    String,
}

impl Type {
    pub fn ir_repr(&self) -> impl AsRef<str> {
        match self {
            Type::T(_) => "i32", // panic!("Should not encounter untyped variables in a typed tree."),
            Type::Fn(_, _) => panic!("d"),
            Type::Bool => "i1",
            Type::Int => "i32",
            Type::String => "i8*", // lol
        }
    }

    pub fn apply_subst(&mut self, subst: &Substitution) {
        for (a, b) in subst.iter() {
            self.sub(*a, b);
        }
    }

    pub fn sub(&mut self, var: u32, t: &Type) {
        let new_self = match self {
            Type::Bool | Type::Int | Type::String => None,
            Type::T(n) if *n == var => Some(t.clone()),
            Type::Fn(args, ret) => {
                let mut args = args.clone();
                let mut ret = ret.clone();
                for arg in args.iter_mut() {
                    arg.sub(var, t);
                }
                ret.sub(var, t);
                Some(Type::Fn(args, ret))
            }
            _ => None,
        };
        if let Some(new_self) = new_self {
            *self = new_self;
        }
    }
}

pub trait Typed {
    fn get_type(&self) -> Type;
}

#[derive(Debug)]
pub struct Field(pub String, pub Type);

impl Typed for Field {
    fn get_type(&self) -> Type {
        self.1.clone()
    }
}

#[derive(Debug)]
pub struct Arg(pub String, pub Type);

impl Typed for Arg {
    fn get_type(&self) -> Type {
        self.1.clone()
    }
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
