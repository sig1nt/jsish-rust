mod tokenizer;
mod types;

use std::env;
use tokenizer::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut itr = tokenizer::create_file_stream(&args[1])
                             .expect("Failed to create file stream");

    loop {
        //println!("main");
        match tokenizer::next_token(&mut itr) {
            Err(err) => {println!("{}", err); break;},
            Ok(Token::TkEof) => {println!("EOF"); break},
            Ok(tk) => println!("{:?}", tk)
        }
    }
}
