#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::env;

mod cstream;
use cstream::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args[1]);

    let mut ex = CStream::new(args[1].as_str());
    ex.set_content();
    println!("contents are: {}", ex.get_content());
}
