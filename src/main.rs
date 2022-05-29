#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::env;

mod cstream;
mod token;
mod scanner;

use cstream::*;
use token::*;
use scanner::*;

extern crate custom_error;
use custom_error::custom_error;

custom_error!{MyError
    Syntax{ln_num: i32, char_num: i32, ebnf: i32} = 
        "Error at Line {ln_num} Character {char_num}. The syntax should be: {ebnf}"
}

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
    // sc_ex.get_all_token();
    // println!("{:?}", sc_ex.get_next_token());
    // let result: Result<String, MyError> = example_cstream(ex);
    match example_cstream(ex) {
        Ok(description) => println!("{}",description),
        Err(err) => println!("{}", err),
    }
    match example_scanner(sc_ex) {
        Ok(description) => println!("{}",description),
        Err(err) => println!("{}", err),
    }


    fn example_cstream(mut ex: CStream) -> Result<String, MyError> {
        match ex.get_next_char() {
            Some(ch) => {
                return Ok("Syntax is valid".to_string())
            }
            _ => {
                return Err(MyError::Syntax { ln_num: 10, char_num: 5, ebnf: 404 })
            }
        }
        
    }

    fn example_scanner(mut scan: Scanner) -> Result<String, MyError> {
        match  scan.get_next_token() {
            Some(token_type) => {
                Err(MyError::Syntax { ln_num: token_type.get_line_num(), char_num: token_type.get_char_pos(), ebnf: 123 })
            }
            _ => {
                Ok("No token type (it shouldn't reach here)".to_string())
            }
        }
    }
    
}
