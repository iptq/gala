use std::collections::{BTreeMap, HashMap, HashSet};

use failure::Error;

use common::{next_int, Type, Typed};
use mir;

pub type Substitution = HashMap<u32, Type>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Constraint(pub Type, pub Type);

impl Constraint {
    pub fn new(left: &Type, right: &Type) -> Self {
        Constraint(left.clone(), right.clone())
    }
}

pub trait TypeLookup: ::std::fmt::Debug {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type>;
    fn variable(&mut self, name: impl AsRef<str>, ty: &Type);
}

#[derive(Default, Debug)]
pub struct TypeContext {
    bindings: BTreeMap<String, Type>,
}

impl TypeLookup for TypeContext {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type> {
        self.bindings.get(name.as_ref()).cloned()
    }
    fn variable(&mut self, name: impl AsRef<str>, ty: &Type) {
        self.bindings.insert(name.as_ref().to_owned(), ty.clone());
    }
}

#[derive(Debug)]
pub struct TypeStack(Vec<TypeContext>);

impl Default for TypeStack {
    fn default() -> Self {
        TypeStack(vec![TypeContext::default()])
    }
}

impl TypeLookup for TypeStack {
    fn lookup(&self, name: impl AsRef<str>) -> Option<Type> {
        for item in self.0.iter().rev() {
            if let Some(t) = item.lookup(&name) {
                return Some(t);
            }
        }
        None
    }
    fn variable(&mut self, name: impl AsRef<str>, ty: &Type) {
        if let Some(scope) = self.0.last_mut() {
            scope.variable(name, ty)
        }
    }
}

impl TypeStack {
    pub fn scope(&mut self) {
        self.0.push(TypeContext::default());
    }
    pub fn unscope(&mut self) {
        self.0.pop();
    }
}

impl mir::Program {
    pub fn typeck(&mut self, ctx: &mut TypeStack) -> Result<(), Error> {
        self.0
            .iter_mut()
            .map(|decl| {
                decl.build_ctx(ctx);
                decl
            }).collect::<Vec<_>>()
            .iter_mut()
            .map(|decl| decl.typeck(ctx))
            .collect::<Result<(), _>>()
    }
}

impl mir::TopDecl {
    pub fn build_ctx(&mut self, ctx: &mut TypeStack) {
        use mir::TopDecl;
        match self {
            TopDecl::Extern(name, ty) => ctx.variable(name, ty),
            TopDecl::Fn(name, args, ty, _body) => ctx.variable(
                name,
                &Type::Fn(
                    args.iter().map(|arg| arg.get_type()).collect::<Vec<_>>(),
                    Box::new(ty.clone()),
                ),
            ),
            _ => (),
        }
    }
    pub fn typeck(&mut self, ctx: &mut TypeStack) -> Result<(), Error> {
        use mir::TopDecl;
        match self {
            TopDecl::Fn(_name, args, _ty, body) => {
                ctx.scope();
                for arg in args {
                    ctx.variable(&arg.0, &arg.get_type());
                }
                let constraints = body
                    .iter_mut()
                    .flat_map(|stmt| stmt.generate_constraints(ctx))
                    .collect::<HashSet<_>>();
                let substitutions = unify(constraints)?;
                for stmt in body.iter_mut() {
                    stmt.apply_subst(&substitutions);
                }
                ctx.unscope();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl mir::Stmt {
    pub fn apply_subst(&mut self, subst: &Substitution) {
        use mir::Stmt;
        match self {
            Stmt::Assign(_, _, expr) => expr.apply_subst(subst),
            Stmt::Expr(expr) => expr.apply_subst(subst),
            Stmt::If(cond, body1, body2) => {
                cond.apply_subst(subst);
                for stmt in body1 {
                    stmt.apply_subst(subst);
                }
                if let Some(body) = body2 {
                    for stmt in body {
                        stmt.apply_subst(subst);
                    }
                }
            }
            Stmt::While(cond, body) => {
                cond.apply_subst(subst);
                for stmt in body.iter_mut() {
                    stmt.apply_subst(subst);
                }
            }
            Stmt::Return(expr) => if let Some(expr) = expr {
                expr.apply_subst(subst);
            },
        }
    }
    pub fn generate_constraints(&mut self, ctx: &mut TypeStack) -> HashSet<Constraint> {
        use mir::Stmt;
        match self {
            Stmt::Assign(re, name, expr) => {
                if !*re {
                    ctx.variable(&name, &Type::T(next_int()));
                }

                let mut result = expr.generate_constraints(ctx);
                result.extend(match ctx.lookup(&name) {
                    Some(ty) => vec![Constraint::new(&ty, &expr.get_type())],
                    None => panic!("Name '{}' not found.", name),
                });
                result
            }
            Stmt::Expr(expr) => expr.generate_constraints(ctx),
            Stmt::If(cond, body1, body2) => vec![
                cond.generate_constraints(ctx),
                {
                    ctx.scope();
                    body1
                        .iter_mut()
                        .flat_map(|stmt| stmt.generate_constraints(ctx))
                        .collect::<HashSet<_>>()
                },
                {
                    ctx.scope();
                    match body2 {
                        Some(body) => body
                            .iter_mut()
                            .flat_map(|stmt| stmt.generate_constraints(ctx))
                            .collect::<HashSet<_>>(),
                        None => HashSet::new(),
                    }
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
    pub fn apply_subst(&mut self, subst: &Substitution) {
        use mir::Expr;
        match self {
            Expr::Literal(_lit, ty) => ty.apply_subst(subst),
            Expr::Name(_, ty) => ty.apply_subst(subst),
            Expr::Call(_, args, ty) => {
                for arg in args {
                    arg.apply_subst(subst);
                }
                ty.apply_subst(subst);
            }
            Expr::NotEquals(left, right, ty)
            | Expr::Equals(left, right, ty)
            | Expr::Times(left, right, ty)
            | Expr::Minus(left, right, ty)
            | Expr::Plus(left, right, ty) => {
                left.apply_subst(subst);
                right.apply_subst(subst);
                ty.apply_subst(subst);
            }
        }
    }
    pub fn generate_constraints(&mut self, ctx: &mut TypeStack) -> HashSet<Constraint> {
        use mir::Expr;
        match self {
            Expr::Call(name, args, ty) => match ctx.lookup(&name) {
                Some(Type::Fn(args_t, ret)) => {
                    if args.len() != args_t.len() {
                        panic!("Function argument length mismatch.");
                    }

                    let mut result = vec![Constraint::new(&ret, &ty)];
                    result.extend(
                        args.iter()
                            .zip(args_t.iter())
                            .map(|(a, b)| Constraint::new(&a.get_type(), &b))
                            .collect::<Vec<_>>(),
                    );
                    result
                }
                _ => panic!("Name '{}' not bound or not a function.", name),
            },
            Expr::Literal(lit, ty) => vec![Constraint::new(&lit.get_type(), ty)],
            Expr::Name(name, ty) => match ctx.lookup(&name) {
                Some(ty1) => vec![Constraint::new(&ty1, ty)],
                None => panic!("Name '{}' not bound.", name),
            },
            Expr::NotEquals(left, right, ty) | Expr::Equals(left, right, ty) => {
                let left = left.get_type();
                let right = right.get_type();
                vec![
                    Constraint::new(&left, &right),
                    Constraint::new(ty, &Type::Bool),
                ]
            }
            Expr::Times(left, right, ty)
            | Expr::Minus(left, right, ty)
            | Expr::Plus(left, right, ty) => {
                let left = left.get_type();
                let right = right.get_type();
                vec![
                    Constraint::new(&left, &right),
                    Constraint::new(ty, &left),
                    Constraint::new(ty, &right),
                ]
            }
        }.into_iter()
        .collect::<HashSet<_>>()
    }
}

fn unify(constraints: HashSet<Constraint>) -> Result<Substitution, Error> {
    let mut constraints = constraints.into_iter().collect::<Vec<_>>();
    let mut substitution = Substitution::new();

    while let Some(Constraint(t1, t2)) = constraints.pop() {
        if t1 == t2 {
            continue;
        }

        match (&t1, &t2) {
            (Type::T(n), t) | (t, Type::T(n)) => {
                for Constraint(c1, c2) in &mut constraints {
                    c1.sub(*n, &t);
                    c2.sub(*n, &t);
                }
                substitution.insert(*n, t.clone());
            }
            _ => bail!("Can't unify {:?} ~ {:?}", t1, t2),
        };
    }

    loop {
        let mut flag = false;
        let mut inserts = Vec::new();
        for (n, t) in substitution.iter() {
            if let Type::T(m) = t {
                flag = true;
                let s = substitution.get(m);

                if let Some(t) = s {
                    inserts.push((*n, t.clone()));
                }
            }
        }
        substitution.extend(inserts);
        if !flag {
            break;
        }
    }

    Ok(substitution)
}
