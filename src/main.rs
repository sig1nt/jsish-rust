mod tokenizer;
mod types;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    tokenizer::nextToken(&args[1]);
}
