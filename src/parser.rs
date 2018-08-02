//! Parser, based on Pest

use failure::Error;
use pest::iterators::Pair;
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

use ast;
use literal::Literal;
use op::Op;

// force grammar changes to rebuild
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("gala.pest");

#[derive(Parser)]
#[grammar = "gala.pest"]
pub struct GalaParser;

lazy_static! {
    static ref INFIX_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        // |, &
        Operator::new(Rule::op_or, Assoc::Left)
        | Operator::new(Rule::op_and, Assoc::Left),
        // <=, >=, <, >, ==, !=
        Operator::new(Rule::op_lte, Assoc::Left)
        | Operator::new(Rule::op_gte, Assoc::Left)
        | Operator::new(Rule::op_lt, Assoc::Left)
        | Operator::new(Rule::op_gt, Assoc::Left)
        | Operator::new(Rule::op_eq, Assoc::Left)
        | Operator::new(Rule::op_neq, Assoc::Left),
        // +, -
        Operator::new(Rule::op_add, Assoc::Left)
        | Operator::new(Rule::op_sub, Assoc::Left),
        // *, /, %
        Operator::new(Rule::op_mul, Assoc::Left)
        | Operator::new(Rule::op_div, Assoc::Left)
        | Operator::new(Rule::op_mod, Assoc::Left),
    ]);
}

pub fn parse_module(input: &str) -> Result<ast::Module, Error> {
    let mut pairs = match GalaParser::parse(Rule::module, input) {
        Ok(p) => p,
        Err(e) => bail!("{:?}", e),
    };
    let mut decls = Vec::new();
    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::decl => decls.push(convert_decl(p)),
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    Ok(ast::Module { decls })
}

pub fn parse_line(input: &str) -> Result<ast::Expr, Error> {
    let mut pairs = match GalaParser::parse(Rule::line, input) {
        Ok(p) => p,
        Err(e) => bail!("{:?}", e),
    };
    let mut expr = None;
    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::expr => expr = Some(convert_expr(p)),
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    Ok(expr.unwrap())
}

pub fn convert_decl(pair: Pair<Rule>) -> ast::Decl {
    assert_eq!(pair.as_rule(), Rule::decl);
    let mut decl = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::decl_fn => decl = Some(convert_function(p)),
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    ast::Decl::Fn(decl.unwrap())
}

pub fn convert_expr(pair: Pair<Rule>) -> ast::Expr {
    let mut expr = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::anon_fn => expr = Some(ast::Expr::Fn(convert_function(p))),
            Rule::literal => expr = Some(ast::Expr::Lit(convert_literal(p))),
            Rule::infix_expr => {
                let primary = |pair: Pair<Rule>| match pair.as_rule() {
                    Rule::literal => ast::Value::Lit(convert_literal(pair)),
                    Rule::expr => ast::Value::Expr(convert_expr(pair)),
                    _ => unreachable!("unexpected {:?}", pair),
                };
                let infix = |left: ast::Value, op: Pair<Rule>, right: ast::Value| {
                    ast::Value::Expr(ast::Expr::BinOp(ast::BinOp {
                        left: Box::new(ast::Expr::from(left)),
                        right: Box::new(ast::Expr::from(right)),
                        op: match op.as_rule() {
                            Rule::op_or => Op::Or,
                            Rule::op_and => Op::And,
                            Rule::op_lte => Op::Lte,
                            Rule::op_gte => Op::Gte,
                            Rule::op_lt => Op::Lt,
                            Rule::op_gt => Op::Gt,
                            Rule::op_eq => Op::Eq,
                            Rule::op_neq => Op::Neq,
                            Rule::op_add => Op::Add,
                            Rule::op_sub => Op::Sub,
                            Rule::op_mul => Op::Mul,
                            Rule::op_div => Op::Div,
                            Rule::op_mod => Op::Mod,
                            _ => unreachable!(),
                        },
                    }))
                };
                let value = INFIX_CLIMBER.climb(p.into_inner(), primary, infix);
                expr = Some(ast::Expr::from(value))
            }
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    expr.unwrap()
}

pub fn convert_function(pair: Pair<Rule>) -> ast::Function {
    let mut name = None;
    let mut expr = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident => name = Some(p.into_span().as_str().to_owned()),
            Rule::expr => expr = Some(convert_expr(p)),
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    ast::Function {
        name,
        body: Box::new(expr.unwrap()),
    }
}

pub fn convert_literal(pair: Pair<Rule>) -> Literal {
    let p = pair.into_inner().next().unwrap();
    match p.as_rule() {
        Rule::int => Literal::Int,
        _ => unreachable!("unexpected {:?}", p),
    }
}
