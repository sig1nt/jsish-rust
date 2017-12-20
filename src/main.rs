mod ast;
mod tokenizer;
mod types;
mod parser;
mod interpreter;

use std::env;

use types::JsishResult;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run_jsish(&args[1]) {
        Ok(()) => (),
        Err(types::JsishError::Message(e)) => println!("{}", e),
        Err(e) => println!("{}", e)
    }
}

fn run_jsish(filename: &str) -> JsishResult<()> {
    let p = parser::parse(filename)?;
    println!("{:?}", p);
    interpreter::interpret(p)
}
