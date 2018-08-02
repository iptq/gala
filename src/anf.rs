//! A-Normal Form

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
pub enum Expr {
    A(AExpr),
    C(CExpr),
}

pub mod convert {
    use anf;
    use ast;

    impl From<ast::Expr> for anf::Expr {
        fn from(expr: ast::Expr) -> Self {
            match expr {
                ast::Expr::Fn(_) => anf::Expr::C(anf::CExpr::D),
                ast::Expr::Lit(lit) => anf::Expr::A(anf::AExpr::Lit(lit)),
                ast::Expr::BinOp(binop) => anf::Expr::A(convert_aexpr(ast::Expr::BinOp(binop))),
                _ => anf::Expr::C(anf::CExpr::D),
            }
        }
    }

    pub fn convert_aexpr(expr: ast::Expr) -> anf::AExpr {
        match expr {
            ast::Expr::Lit(lit) => anf::AExpr::Lit(lit),
            ast::Expr::BinOp(ast::BinOp { left, op, right }) => anf::AExpr::BinOp(anf::BinOp {
                left: Box::new(convert_aexpr(*left)),
                op,
                right: Box::new(convert_aexpr(*right)),
            }),
            _ => anf::AExpr::Lit(::literal::Literal::Int),
        }
    }
}
