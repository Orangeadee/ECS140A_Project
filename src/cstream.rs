// Use professor's solution in hw2.
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Clone)]
pub struct CStream {
    filename: String,
    line_num: i32,
    char_pos: i32,
    content: String,
    overall_pos: i32,
    size: i32
}

impl CStream {
    pub fn new(f: &str) -> CStream {
        match File::open(f) {
            Ok(mut file) => {
                let mut temp_content = String::new();
                file.read_to_string(&mut temp_content).unwrap();
                let temp_size = temp_content.chars().count() as i32;
                CStream {
                    filename: f.to_string(),
                    line_num: -1,
                    char_pos: -1,
                    content: temp_content,
                    overall_pos: -1,
                    size: temp_size
                }
            }
            Err(_) => {
                panic!("Error opening file {}", f);
            }
        }
    }

    // pub fn set_content(&mut self) {
    //     match File::open(self.filename.as_str()) {
    //         Ok(mut file) => {
    //             let mut temp_content = String::new();
    //             file.read_to_string(&mut temp_content).unwrap();
    //             self.content = temp_content;
    //             self.size = self.content.chars().count() as i32;
    //         }
    //         Err(_) => {
    //             panic!("Error open file {}", self.filename);
    //         }
    //     }
    // }
    pub fn get_size(&self) -> i32 {
        self.size
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn more_available(&self) -> bool {
        // self.overall_pos < self.size - 1
        !self.content.is_empty()
    }

    pub fn get_cur_char(&self) -> Option<char> {
        self.content.chars().nth(0 as usize)
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        let curr = self.content.chars().nth((self.overall_pos) as usize);
        match curr {
            None => {
                self.char_pos += 1;
                self.line_num += 1;
            },
            Some(x) => {
                if x == '\n' {
                    self.char_pos = 0;
                    self.line_num += 1;
                } else {
                    self.char_pos += 1;
                    if self.overall_pos == -1 {
                        self.line_num += 1;
                    }
                }
            }
        }
        self.overall_pos += 1;
        
        let temp_char = self.peek_next_char();
        let mut temp_cont = self.content.chars();
        if temp_char != None {
            temp_cont.next();
            self.content = String::from( temp_cont.as_str() );
            temp_char
        } else {
            None
        }
        // self.content.chars().nth((self.overall_pos) as usize)
    }

    pub fn peek_next_char(&self) -> Option<char> {
        self.content.chars().nth(0 as usize)
    }

    pub fn peek_ahead_char(&self, k: i32) -> Option<char> {
        if k >= 0 {
            self.content.chars().nth(k as usize)
        } else {
            None
        }
    }
}