use std::collections::{BTreeMap, HashSet};

use failure::Error;

use common::{Type, Typed};
use mir;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Constraint(pub Type, pub Type);

impl Constraint {
    pub fn new(left: &Type, right: &Type) -> Self {
        Constraint(left.clone(), right.clone())
    }
}

pub trait TypeLookup {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type>;
}

pub struct TypeContext {
    bindings: BTreeMap<String, Type>,
}

impl TypeLookup for TypeContext {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type> {
        self.bindings.get(name.as_ref()).map(|ty| ty.clone())
    }
}

impl<T: TypeLookup> TypeLookup for Vec<T> {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type> {
        for item in self.iter().rev() {
            if let Some(t) = item.lookup(&name) {
                return Some(t);
            }
        }
        None
    }
}

impl mir::Program {
    pub fn typeck(&mut self) -> Result<(), Error> {
        self.0
            .iter_mut()
            .map(|decl| decl.typeck())
            .collect::<Result<(), _>>()
    }
}

impl mir::TopDecl {
    pub fn typeck(&mut self) -> Result<(), Error> {
        use mir::TopDecl;
        match self {
            TopDecl::Extern(_, _) => Ok(()),
            TopDecl::Fn(name, args, ty, body) => {
                let mut ctx = Vec::new();
                let constraints = body
                    .iter_mut()
                    .flat_map(|stmt| stmt.generate_constraints(&mut ctx))
                    .collect::<HashSet<_>>();
                println!("constraints: {:?}", constraints);
                Ok(())
            }
        }
    }
}

impl mir::Stmt {
    pub fn generate_constraints(&mut self, ctx: &mut Vec<TypeContext>) -> HashSet<Constraint> {
        use mir::Stmt;
        match self {
            Stmt::Assign(_re, _name, expr) => expr.generate_constraints(ctx),
            Stmt::Expr(expr) => expr.generate_constraints(ctx),
            Stmt::If(cond, body1, body2) => vec![
                cond.generate_constraints(ctx),
                body1
                    .iter_mut()
                    .flat_map(|stmt| stmt.generate_constraints(ctx))
                    .collect::<HashSet<_>>(),
                match body2 {
                    Some(body) => body
                        .iter_mut()
                        .flat_map(|stmt| stmt.generate_constraints(ctx))
                        .collect::<HashSet<_>>(),
                    None => HashSet::new(),
                },
            ].into_iter()
            .flatten()
            .collect::<HashSet<_>>(),
            Stmt::While(cond, body) => vec![
                cond.generate_constraints(ctx),
                body.iter_mut()
                    .flat_map(|stmt| stmt.generate_constraints(ctx))
                    .collect::<HashSet<_>>(),
            ].into_iter()
            .flatten()
            .collect::<HashSet<_>>(),
            Stmt::Return(expr) => match expr {
                Some(expr) => expr.generate_constraints(ctx),
                None => HashSet::new(),
            },
        }
    }
}

impl mir::Expr {
    pub fn generate_constraints(&mut self, ctx: &mut Vec<TypeContext>) -> HashSet<Constraint> {
        use mir::Expr;
        match self {
            Expr::Name(name, ty) => match ctx.lookup(name) {
                Some(ty1) => vec![Constraint::new(&ty1, ty)],
                None => vec![],
            },
            Expr::Equals(left, right, ty) => {
                let left = left.get_type();
                let right = right.get_type();
                vec![
                    Constraint::new(&left, ty),
                    Constraint::new(&right, ty),
                    Constraint::new(&left, &right),
                ]
            }
            _ => {
                panic!("{:?}", self);
                unimplemented!()
            }
        }.into_iter()
        .collect::<HashSet<_>>()
    }
}
