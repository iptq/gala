use common::{Arg, Literal, Type, Typed};
use mir::{self, Context, IntoMir};

#[derive(Debug)]
pub struct Program(pub Vec<TopDecl>);

impl IntoMir<mir::Program> for Program {
    fn into_mir(self, ctx: &mut Context) -> mir::Program {
        mir::Program(
            self.0
                .into_iter()
                .map(|decl| decl.into_mir(ctx))
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
pub enum TopDecl {
    Extern(String, Type),
    Fn(String, Vec<Arg>, Type, Vec<Stmt>),
}

impl IntoMir<mir::TopDecl> for TopDecl {
    fn into_mir(self, ctx: &mut Context) -> mir::TopDecl {
        match self {
            TopDecl::Extern(name, ty) => mir::TopDecl::Extern(name, ty),
            TopDecl::Fn(name, args, ty, body) => mir::TopDecl::Fn(
                name,
                args,
                ty,
                body.into_iter()
                    .map(|stmt| stmt.into_mir(ctx))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Assign(bool, String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Return(Option<Expr>),
}

impl IntoMir<mir::Stmt> for Stmt {
    fn into_mir(self, ctx: &mut Context) -> mir::Stmt {
        match self {
            Stmt::Assign(re, name, expr) => mir::Stmt::Assign(re, name, expr.into_mir(ctx)),
            Stmt::Expr(expr) => mir::Stmt::Expr(expr.into_mir(ctx)),
            Stmt::If(cond, body1, body2) => mir::Stmt::If(
                cond.into_mir(ctx),
                body1
                    .into_iter()
                    .map(|stmt| stmt.into_mir(ctx))
                    .collect::<Vec<_>>(),
                body2.map(|body| {
                    body.into_iter()
                        .map(|stmt| stmt.into_mir(ctx))
                        .collect::<Vec<_>>()
                }),
            ),
            Stmt::While(cond, body) => mir::Stmt::While(
                cond.into_mir(ctx),
                body.into_iter()
                    .map(|stmt| stmt.into_mir(ctx))
                    .collect::<Vec<_>>(),
            ),
            Stmt::Return(expr) => mir::Stmt::Return(expr.map(|expr| expr.into_mir(ctx))),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Call(String, Vec<Expr>),
    Literal(Literal),
    Name(String),
    NotEquals(Box<Expr>, Box<Expr>),
    Equals(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
}

impl IntoMir<mir::Expr> for Expr {
    fn into_mir(self, ctx: &mut Context) -> mir::Expr {
        match self {
            Expr::Call(func, args) => {
                let args = args
                    .into_iter()
                    .map(|expr| expr.into_mir(ctx))
                    .collect::<Vec<_>>();
                mir::Expr::Call(func, args, ctx.next())
            }
            Expr::Literal(lit) => {
                let ty = lit.get_type();
                mir::Expr::Literal(lit.into(), ty)
            }
            Expr::Name(name) => mir::Expr::Name(name, ctx.next()),
            Expr::NotEquals(left, right) => {
                let left = Box::new((*left).into_mir(ctx));
                let right = Box::new((*right).into_mir(ctx));
                mir::Expr::NotEquals(left, right, Type::Bool)
            }
            Expr::Equals(left, right) => {
                let left = Box::new((*left).into_mir(ctx));
                let right = Box::new((*right).into_mir(ctx));
                mir::Expr::Equals(left, right, Type::Bool)
            }
            Expr::Plus(left, right) => {
                let left = Box::new((*left).into_mir(ctx));
                let right = Box::new((*right).into_mir(ctx));
                mir::Expr::Plus(left, right, ctx.next())
            }
            Expr::Minus(left, right) => {
                let left = Box::new((*left).into_mir(ctx));
                let right = Box::new((*right).into_mir(ctx));
                mir::Expr::Minus(left, right, ctx.next())
            }
            Expr::Times(left, right) => {
                let left = Box::new((*left).into_mir(ctx));
                let right = Box::new((*right).into_mir(ctx));
                mir::Expr::Times(left, right, ctx.next())
            }
        }
    }
}
