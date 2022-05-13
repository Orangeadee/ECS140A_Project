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
    eof: bool
}

impl Scanner {
    pub fn new(filename: &str) -> Scanner {
        Scanner {
            content: CStream::new(filename),
            line_num: 1,
            char_pos: 1,
            eof_chars: vec!['{','}','(',')','\n',';','\t',',',' '],
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
                (String::from("main"), TokenType::KEYWORD)
            ]),
            eof: false,
            operators: vec!['+','-','*','/','=','>','<','!']
        }
    }

    fn check_eof(&mut self) -> bool{
        while self.content.more_available() {
            match self.content.peek_next_char() {
                Some(ch) => {
                    if !self.eof_chars.contains(&ch) {
                        break;
                    } else {
                        if ch == '{' {
                            return true;
                        } 
                        else if ch == '\n' {
                            self.char_pos = 1;
                            self.line_num += 1;
                        }
                        self.content.get_next_char();
                    }
                }
            }
        }
        return false;
    }
    
    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut currType: TokenType = TokenType::START;
        let curr = String::from("");
        let curr_char_pos = self.char_pos;
        
        self.eof = self.check_eof();
        match self.eof {
            true => {
                return Some(Token::new(String::from(""), TokenType::START, 0, 0));
            }
            false => {}
        }

        while self.content.more_available() {
            match self.content.get_next_char() {
                None => break,
                Some(ch) => match self.eof_chars.contains(&ch) {
                    true => {}
                    false => {
                        
                    }
                }
            }
        }
    }
}