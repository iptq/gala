//! A-Normal Form

use symbol::Symbol;

use literal::Literal;
use op::Op;

/// Atomic expressions (aexp) are those for which evaluation must always terminate, never cause an error and never produce a side effect.
#[derive(Clone, Debug)]
pub enum AExpr {
    BinOp(BinOp),
    Lit(Literal),
}

#[derive(Clone, Debug)]
pub struct BinOp {
    left: Box<AExpr>,
    op: Op,
    right: Box<AExpr>,
}

/// Complex expressions (cexp) may not terminate, may produce an error and may have a side effect. However, a complex expression may defer execution to only one other complex expression. For instance, letrec defers directly to its body, and if defers to only one of its arms.
#[derive(Clone, Debug)]
pub enum CExpr {
    D,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Fn(Expr),
}

#[derive(Clone, Debug)]
pub enum Expr {
    A(AExpr),
    C(CExpr),
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Symbol,
    pub body: Vec<Decl>,
}

pub mod convert {
    use anf;
    use ast;

    impl From<ast::Decl> for anf::Decl {
        fn from(decl: ast::Decl) -> Self {
            match decl {
                ast::Decl::Fn(_func) => anf::Decl::Fn(anf::Expr::C(anf::CExpr::D)),
            }
        }
    }

    impl From<ast::Expr> for anf::Expr {
        fn from(expr: ast::Expr) -> Self {
            match expr {
                ast::Expr::Fn(_) => anf::Expr::C(anf::CExpr::D),
                ast::Expr::Lit(lit) => anf::Expr::A(anf::AExpr::Lit(lit)),
                ast::Expr::BinOp(binop) => anf::Expr::A(convert_aexpr(ast::Expr::BinOp(binop))),
            }
        }
    }

    impl From<ast::Module> for anf::Module {
        fn from(module: ast::Module) -> Self {
            anf::Module {
                name: module.name,
                body: module.body.into_iter().map(anf::Decl::from).collect(),
            }
        }
    }

    pub fn convert_aexpr(expr: ast::Expr) -> anf::AExpr {
        match expr {
            ast::Expr::Fn(_) => anf::AExpr::Lit(::literal::Literal::Int),
            ast::Expr::Lit(lit) => anf::AExpr::Lit(lit),
            ast::Expr::BinOp(ast::BinOp { left, op, right }) => anf::AExpr::BinOp(anf::BinOp {
                left: Box::new(convert_aexpr(*left)),
                op,
                right: Box::new(convert_aexpr(*right)),
            }),
        }
    }
}
