use failure::Error;
use pest::Parser;
use pest::iterators::Pair;

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
    let decls = Vec::new();
    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::decl => println!("p: {:?}", p),
            _ => unreachable!("Got {:?}", p),
        }
    }
    Ok(ast::Module { decls })
}
