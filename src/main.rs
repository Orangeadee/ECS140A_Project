#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::env;

mod cstream;
mod token;
mod scanner;

use cstream::*;
use token::*;
use scanner::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args[1]);

    let mut ex = CStream::new(args[1].as_str());
    // ex.set_content();
    println!("Contents are: {}", ex.get_content());
    println!("Size is: {}", ex.get_size());
    // while ex.more_available() {
    //     println!("ch: {:?}", ex.get_next_char());
    // }

    // let tk_ex = Token::new("hello".to_string(), TokenType::KEYWORD, -1, 0);
    // println!("{}", tk_ex.get_char_pos());
    // println!("{}", tk_ex.get_text());
    // println!("{}", tk_ex.get_line_num());
    // println!("{}", tk_ex.get_token_type());

    let mut sc_ex = Scanner::new(args[1].as_str());
    // while sc_ex.more_avail() {
    //     println!("{:?}", sc_ex.get_next_token());
    // }
    // println!("{:?}", sc_ex.get_all_token());
    sc_ex.get_all_token();
    // println!("{:?}", sc_ex.get_next_token());
}
