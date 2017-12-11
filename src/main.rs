mod ast;
mod tokenizer;
mod types;
mod parser;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut itr = tokenizer::create_file_stream(&args[1])
                             .expect("Failed to create file stream");

    match parser::parse_stream(&mut itr) {
        Ok(p) => print!("{}", p),
        Err(types::JsishError::Message(e)) => println!("{}", e),
        Err(e) => println!("{}", e)
    }
}
