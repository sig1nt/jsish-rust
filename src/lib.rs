#[macro_use]
extern crate clap;

mod tokenizer;
mod parser;
mod interpreter;
pub mod types;
mod ast;

use types::*;

#[derive(Debug)]
enum Mode {
    Ast,
    Print,
    Interpret
}

pub struct Config {
    mode: Mode,
    filename: String
}

impl Config {
    pub fn new() -> JsishResult<Config> {
        let matches = clap_app!(jsish =>
            (@arg AST: -a --ast "print debug-style AST instead of interpretting")
            (@arg PRINT: -p --print "Pretty print AST instead of interpretting")
            (@arg FILENAME: +required "Specifies the input file to use")
        ).get_matches();

        let filename = String::from(matches.value_of("FILENAME").unwrap());

        let mode = match (matches.is_present("AST"), 
                          matches.is_present("PRINT")) {
            (true, true) => 
                return Err(JsishError::from("Only specify one mode")),
            (true, false) => Mode::Ast,
            (false, true) => Mode::Print,
            (false, false) => Mode::Interpret
        };

        Ok(Config {filename: filename, mode: mode})
    }
}

pub fn run(config: Config) -> JsishResult<()> {
    let prog = parser::parse(&config.filename)?;

    match config.mode {
        Mode::Ast => Ok(println!("{:?}", prog)),
        Mode::Print => Ok(print!("{}", prog)),
        Mode::Interpret => interpreter::interpret(prog)
    }
}
