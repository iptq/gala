extern crate gala;
extern crate rustyline;
extern crate failure;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use failure::Error;
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

            let module = gala::parser::parse_module(&contents);
            println!("{:?}", module);
        }
        None => {
            let mut rl = Editor::<()>::new();
            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_ref());
                        println!("{:?}", gala::parser::parse_expr(&line));
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
