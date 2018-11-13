#[macro_use]
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate structopt;

mod ast;
mod codegen;
mod common;
mod lexer;
mod mir;
mod typeck;

use std::fs::File;
use std::io::{stdin, Read, Stdin};
use std::path::PathBuf;

use failure::Error;
use structopt::StructOpt;

use codegen::{Codegen, Emitter};
use mir::IntoMir;
use typeck::TypeStack;

enum Input {
    File(File),
    Stdin(Stdin),
}

impl AsMut<Read> for Input {
    fn as_mut(&mut self) -> &mut (Read + 'static) {
        match self {
            Input::File(file) => file,
            Input::Stdin(stdin) => stdin,
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

lalrpop_mod!(pub parser);

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut input = match opt.file {
        Some(path) => Input::File(File::open(&path)?),
        None => Input::Stdin(stdin()),
    };

    let mut buf = Vec::new();
    let reader = input.as_mut();
    reader.read_to_end(&mut buf)?;
    let contents = String::from_utf8(buf)?;

    let lexer = lexer::Lexer::new(&contents);
    let parser = parser::ProgramParser::new();
    let ast = parser
        .parse(&contents)
        .map_err(|err| format_err!("{}", err))?;

    let mut context = mir::Context::default();
    let mut mir = ast.into_mir(&mut context);

    let mut stack = TypeStack::default();
    mir.typeck(&mut stack);
    eprintln!("{:?}", mir);

    let mut emitter = Emitter::new();
    mir.generate(&mut emitter);

    println!("{}", emitter.as_string());
    Ok(())
}
