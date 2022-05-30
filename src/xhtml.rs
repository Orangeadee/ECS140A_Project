#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::cstream::CStream;
use crate::token::*;
use crate::scanner::*;

pub struct XHTML {
    tokens: Vec<Token>,
    output: String
}

impl XHTML {
    pub fn new(tks: Vec<Token>) -> XHTML {
        XHTML {
            tokens: tks,
            output: "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n<head>\n<title>\nX Formatted file\n</title>\n</head>\n<body bgcolor=\"navy\" text=\"orange\" link=\"orange\" vlink=\"orange\">\n<font face=\"Courier New\">\n".to_string()
        }
    }

    pub fn all_output(&mut self) {
        while !self.tokens.is_empty() {
            let temp = self.tokens.remove(0);
            let mut temp_str: String = "".to_string();
            match temp.get_token_type() {
                TokenType::IDENTIFIER => {
                    temp_str = format!("<font color=\"{}\">{}</font>","yellow",temp.get_text());
                }
                TokenType::FLOATCONSTANT => {
                    temp_str = format!("<font color=\"{}\"><b>{}</b></font>","aqua",temp.get_text());
                }
                TokenType::INTCONSTANT => {
                    temp_str = format!("<font color=\"{}\"><b>{}</b></font>","aqua",temp.get_text());
                }
                TokenType::OPERATOR => {
                    temp_str = format!("<font color=\"{}\"><b>{}</b></font>","white",temp.get_text());
                }
                TokenType::KEYWORD => {
                    temp_str = format!("<font color=\"{}\"><b>{}</b></font>","white",temp.get_text());
                }
                _ => {}
            }
            self.output.push_str(&temp_str);
        }
        let final_str = "\n</font>\n</body>\n</html>\n".to_string();
        self.output.push_str(&final_str);
    }

    pub fn get_str(&self) -> &String {
        &self.output
    }
}