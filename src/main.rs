extern crate jsish_rust as jsish;

use std::process;
use jsish::*;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        match e {
            jsish::types::JsishError::Message(e) => println!("{}", e),
            e => println!("{}", e)
        }
    }
}
