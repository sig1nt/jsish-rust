mod tokenizer;
mod parser;
mod interpreter;
pub mod types;
mod ast;

use types::*;

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
    pub fn new(mut args: std::env::Args) -> JsishResult<Config> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(JsishError::from("No filename given"))
        };

        let mode = match args.next() {
            None => Mode::Interpret,
            Some(arg) => match &arg[..] {
                "--ast" => Mode::Ast,
                "--print" => Mode::Print,
                _ => return Err(JsishError::from("Invalid mode"))
            }
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
