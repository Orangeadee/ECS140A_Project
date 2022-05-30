#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use std::collections::HashMap;

use crate::cstream::CStream;
use crate::token::*;

#[derive(Clone)]
pub struct Scanner {
    content: CStream,
    line_num: i32,
    char_pos: i32,
    eof_chars: Vec<char>, // all chars that represents an end of checking a token
    operators: Vec<char>,
    eof: bool, // check if reach the end of the line or the block
    digits: Vec<char>,
    new_word: bool,
    total_token: Vec<Token>,
    keywords: Vec<String>,
    token_count: i32,
    all_token: Vec<Token>
}

impl Scanner {
    pub fn new(filename: &str) -> Scanner {
        Scanner {
            content: CStream::new(filename),
            line_num: 1,
            char_pos: 1,
            eof_chars: vec!['{', '}', '(', ')', '\n', ';', '\t', ',', ' '],
            eof: false,
            operators: vec!['+', '-', '*', '/', '=', '>', '<', '!'],
            digits: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
            new_word: false,
            total_token: Vec::new(),
            keywords: vec![
                String::from("unsigned"),
                String::from("char"),
                String::from("short"),
                String::from("int"),
                String::from("long"),
                String::from("float"),
                String::from("double"),
                String::from("while"),
                String::from("if"),
                String::from("return"),
                String::from("main")
            ],
            token_count: -1,
            all_token: Vec::new()
        }
    }

    pub fn more_avail(&self) -> bool {
        self.content.more_available()
    }

    pub fn get_contents(&self) -> &String {
        self.content.get_content()
    }

    fn check_eof(&mut self) {
        self.eof = false;
        while self.more_avail() {
            match self.content.peek_next_char() {
                Some(ch) => {
                    if !self.eof_chars.contains(&ch) {
                        break;
                    } else {
                        if ch == '}' {
                            // self.content.get_next_char();
                            // end = true;
                            self.eof = true;
                            self.content.get_next_char();
                        } else if ch == '\n' {
                            self.char_pos = 1;
                            self.line_num += 1;
                            self.content.get_next_char();
                        } else {
                            self.content.get_next_char();
                        }
                    }
                }
                None => break
            }
        }
    }

    pub fn get_curr_token(&self) -> Option<Token> {
        // if self.total_token.get(self.token_count as usize).is_none() {
        //     None
        // } else {
        //     Some(self.total_token.swap_remove(self.token_count as usize))
        // }
        let vec = self.all_token.clone();
        let res = vec.into_iter().nth(self.token_count as usize);
        res
    }
    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut currType: TokenType = TokenType::START;
        let curr_char_pos: i32 = self.char_pos;
        let mut curr = String::from("");
        let mut next_operator: bool = false;
        self.new_word = false;

        // self.eof = self.check_eof();
        self.check_eof();
        if self.eof == true {
            return Some(Token::new(String::from(""), TokenType::START, 0, 0));
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
                                    // println!("{}",curr);
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
                                    curr.push(ch);
                                    // break;
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
                                    curr.push(ch);
                                    currType = TokenType::INVALID;
                                    // break;
                                }
                            }
                            TokenType::INVALID => {
                                curr.push(ch);
                                // break;
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
                        // self.char_pos += 1;
                        if currType == TokenType::IDENTIFIER {
                            // println!("identifier");
                            if ch == '\n' {
                                self.char_pos = 1;
                                self.line_num += 1;
                                break;
                            } else if ch == '(' {
                                // println!("{}",curr);
                                // println!("{}",self.keyword_map.contains_key(&curr));
                                if !self.keywords.contains(&curr) {
                                    // self.keyword_map.insert(curr.clone(), currType);
                                    // println!("{:?}",self.keyword_map);
                                    break;
                                } else {
                                    currType = TokenType::KEYWORD;
                                    break;
                                }
                                
                            } else {
                                if !self.keywords.contains(&curr) {
                                    break;
                                } else {
                                    currType = TokenType::KEYWORD;
                                    // println!("keyword");
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                },
                None => break,
            }
        }
        // println!("new word: {}", self.new_word);
        if self.new_word == true {
            // println!("current word: {}", &curr);
            let next_token = Token::new(curr, currType, self.line_num, curr_char_pos);
            let res = next_token.clone();
            // let temp_token = Token::new(curr, currType, self.line_num, curr_char_pos);
            self.all_token.push(next_token);
            self.token_count+=1;
            Some(res)
        } else {
            None
        }
    }

    pub fn get_all_token(&mut self) -> Vec<Token> {
        let mut all_token: Vec<Option<Token>> = Vec::new();
        let mut result: Vec<Token> = Vec::new();
        while !self.content.get_content().is_empty() {
            let curr_token = self.get_next_token();
            // println!("{:?}",curr_token);
            all_token.push(curr_token);
            // self.total_token.push(curr_token);
        }
        result = all_token.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
        println!("{:#?}",result);
        // println!("{}",self.get_contents());
        // Some(all_token)
        result
    }
}
