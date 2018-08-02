use failure::Error;
use pest::iterators::Pair;
use pest::Parser;

use ast;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("gala.pest");

#[derive(Parser)]
#[grammar = "gala.pest"]
pub struct GalaParser;

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

pub fn parse_expr(input: &str) -> Result<ast::Expr, Error> {
    let mut pairs = match GalaParser::parse(Rule::expr, input) {
        Ok(p) => p,
        Err(e) => bail!("{:?}", e),
    };
    println!("expr:p: {:?}", pairs);
    Ok(convert_expr(pairs.next().unwrap()))
}

pub fn convert_decl(pair: Pair<Rule>) -> ast::Decl {
    assert_eq!(pair.as_rule(), Rule::decl);
    println!("p: {:?}", pair);
    let mut decl = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::decl_fn => decl = Some(convert_function(p, true)),
            _ => unreachable!("unexpected {:?}", p),
        }
    }
    ast::Decl::Fn(decl.unwrap())
}

pub fn convert_expr(pair: Pair<Rule>) -> ast::Expr {
    let p = pair.into_inner().next().unwrap();
    match p.as_rule() {
        Rule::anon_fn => ast::Expr::Fn(convert_function(p, false)),
        Rule::literal => ast::Expr::Lit(convert_literal(p)),
        _ => unreachable!("unexpected {:?}", p),
    }
}

pub fn convert_function(pair: Pair<Rule>, named: bool) -> ast::Function {
    ast::Function {
        name: Some("shiet".to_owned()),
    }
}

pub fn convert_literal(pair: Pair<Rule>) -> ast::Literal {
    ast::Literal::Int
}
