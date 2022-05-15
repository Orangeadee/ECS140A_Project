#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::collections::HashMap;

use crate::cstream::CStream;
use crate::token::*;

#[derive(Clone)]
pub struct Scanner {
    content: CStream,
    line_num: i32,
    char_pos: i32,
    eof_chars: Vec<char>,
    keyword_map: HashMap<String, TokenType>,
    operators: Vec<char>,
    eof: bool,
    digits: Vec<char>,
    new_word: bool, // all_token: Vec<Token>
}

impl Scanner {
    pub fn new(filename: &str) -> Scanner {
        Scanner {
            content: CStream::new(filename),
            line_num: 1,
            char_pos: 1,
            eof_chars: vec!['{', '}', '(', ')', '\n', ';', '\t', ',', ' '],
            keyword_map: HashMap::from([
                (String::from("unsigned"), TokenType::KEYWORD),
                (String::from("char"), TokenType::KEYWORD),
                (String::from("short"), TokenType::KEYWORD),
                (String::from("int"), TokenType::KEYWORD),
                (String::from("long"), TokenType::KEYWORD),
                (String::from("float"), TokenType::KEYWORD),
                (String::from("double"), TokenType::KEYWORD),
                (String::from("while"), TokenType::KEYWORD),
                (String::from("if"), TokenType::KEYWORD),
                (String::from("return"), TokenType::KEYWORD),
                (String::from("void"), TokenType::KEYWORD),
                (String::from("main"), TokenType::KEYWORD),
            ]),
            eof: false,
            operators: vec!['+', '-', '*', '/', '=', '>', '<', '!'],
            digits: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            new_word: false, // all_token: Vec<Token>::new()
        }
    }

    pub fn more_avail(&self) -> bool {
        self.content.more_available()
    }

    pub fn get_contents(&self) -> &String {
        self.content.get_content()
    }

    fn check_eof(&mut self) -> bool {
        let mut end = false;
        while self.content.more_available() {
            match self.content.peek_next_char() {
                Some(ch) => {
                    if !self.eof_chars.contains(&ch) {
                        break;
                    } else {
                        if ch == '}' {
                            // self.content.get_next_char();
                            end = true;
                        } else if ch == '\n' {
                            self.char_pos = 1;
                            self.line_num += 1;
                        }
                        self.content.get_next_char();
                    }
                }
                None => break,
            }
        }
        return end;
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut currType: TokenType = TokenType::START;
        let mut curr = String::from("");
        let curr_char_pos: i32 = self.char_pos;
        let mut next_operator: bool = false;
        self.new_word = false;

        self.eof = self.check_eof();
        match self.eof {
            true => {

                return Some(Token::new(String::from(""), TokenType::START, 0, 0));
            }
            false => {}
        }
        // println!("curr type {}", currType);
        // println!("more avail? {}", self.content.more_available());
        while self.content.more_available() {
            // println!("next char is: {:#?}", self.content.peek_next_char());

            match self.content.get_next_char() {
                // None => println!("here..?"),
                Some(ch) => match self.eof_chars.contains(&ch) {
                    false => {
                        // println!("here");
                        self.char_pos += 1;
                        // If it's beginning, currType will be START
                        match currType {
                            TokenType::START => {
                                curr.push(ch);
                                self.new_word = true;
                                // print!("{}", self.new_word);
                                if self.digits.contains(&ch) {
                                    currType = TokenType::INTCONSTANT;
                                } else if self.operators.contains(&ch) {
                                    currType = TokenType::OPERATOR;
                                    if self.content.peek_next_char() == Some('=') {
                                        curr.push('=');
                                        self.content.get_next_char();
                                        self.char_pos += 1;
                                    }
                                } else if ch.is_ascii_alphabetic() {
                                    currType = TokenType::IDENTIFIER;
                                } else {
                                    currType = TokenType::INVALID;
                                    break;
                                }
                            }
                            TokenType::IDENTIFIER => {
                                curr.push(ch);
                            }
                            TokenType::FLOATCONSTANT => {
                                if !self.digits.contains(&ch) {
                                    currType = TokenType::INVALID;
                                    break;
                                } else {
                                    curr.push(ch);
                                }
                            }
                            TokenType::INTCONSTANT => {
                                if ch == '.' {
                                    curr.push(ch);
                                    currType = TokenType::FLOATCONSTANT;
                                } else if self.digits.contains(&ch) {
                                    curr.push(ch);
                                } else {
                                    currType = TokenType::INVALID;
                                    break;
                                }
                            }
                            TokenType::INVALID => {
                                break;
                            }
                            TokenType::OPERATOR => {
                                break;
                            }
                            TokenType::KEYWORD => {
                                break;
                            }
                        }

                        match self.content.peek_next_char() {
                            Some(ch) => {
                                if self.operators.contains(&ch) {
                                    next_operator = true;
                                    break;
                                }
                            }
                            None => break,
                        }
                        if next_operator == true {
                            break;
                        }
                    }
                    true => {
                        // println!("here..");
                        // println!("curr in true: {}",ch);
                        self.char_pos += 1;
                        if currType == TokenType::IDENTIFIER {
                            // println!("identifier");
                            if ch == '\n' {
                                self.char_pos = 1;
                                self.line_num += 1;
                            } else if ch == '(' {
                                // println!("{}",curr);
                                println!("{}",self.keyword_map.contains_key(&curr));
                                if !self.keyword_map.contains_key(&curr) {
                                    self.keyword_map.insert(curr.clone(), currType);
                                    // println!("{:?}",self.keyword_map);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                },
                None => break,
            }
        }
        // println!("new word: {}", self.new_word);
        if self.new_word == true {
            println!("current word: {}", &curr);
            let next_token = Token::new(curr, currType, self.line_num, curr_char_pos);
            Some(next_token)
        } else {
            None
        }
    }

    pub fn get_all_token(&mut self) -> Option<Vec<Option<Token>>> {
        let mut all_token: Vec<Option<Token>> = Vec::new();
        while !self.content.get_content().is_empty() {
            let curr_token = self.get_next_token();
            println!("{:?}",curr_token);
            all_token.push(curr_token);
        }

        // println!("{}",self.get_contents());
        Some(all_token)
    }
}
