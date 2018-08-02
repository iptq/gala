extern crate failure;
extern crate gala;
extern crate rustyline;
#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Error;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    let args = Opt::from_args();
    match args.file {
        Some(path) => {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let ast = gala::parser::parse_module(&contents)?;
            println!("ast: {:?}", ast);
            let anf = gala::anf::Module::from(ast);
            println!("anf: {:?}", anf);
        }
        None => {
            let mut rl = Editor::<()>::new();
            match rl.load_history("history.txt") {
                _ => (), // i don't care lol
            }
            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_ref());
                        match gala::parser::parse_line(&line) {
                            Ok(ast) => {
                                println!("ast: {:?}", ast);
                                let anf = gala::anf::Expr::from(ast);
                                println!("anf: {:?}", anf);
                            }
                            Err(err) => eprintln!("{:?}", err),
                        }
                    }
                    Err(ReadlineError::Interrupted) => break,
                    Err(ReadlineError::Eof) => break,
                    Err(err) => panic!("Error: {:?}", err),
                }
            }
            rl.save_history("history.txt")?;
        }
    }
    Ok(())
}
