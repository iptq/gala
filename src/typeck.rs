use std::collections::{BTreeMap, HashSet};

use failure::Error;

use common::Type;
use mir;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Constraint(Type, Type);

pub struct TypeContext {
    bindings: BTreeMap<String, Type>,
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
            Stmt::Assign(name, expr) => expr.generate_constraints(ctx),
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
        let mut c = HashSet::new();
        match self {
            _ => unimplemented!(),
        }
        c
    }
}
