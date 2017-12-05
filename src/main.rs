mod tokenizer;
mod types;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut itr = tokenizer::createFileStream(args[1])
                             .expect("Failed to create file stream");

    tokenizer::nextToken(&mut itr);
}
