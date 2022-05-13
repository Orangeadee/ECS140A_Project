use std::fmt;

#[derive(Clone, Copy)]
pub enum TokenType {
    START,
    INTCONSTANT,
    FLOATCONSTANT,
    KEYWORD,
    OPERATOR,
    IDENTIFIER,
    INVALID
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::START => write!(f,"None"),
            TokenType::INTCONSTANT => write!(f,"IntConstant"),
            TokenType::FLOATCONSTANT => write!(f,"FloatConstant"),
            TokenType::KEYWORD => write!(f,"Keyword"),
            TokenType::OPERATOR => write!(f,"Operator"),
            TokenType::IDENTIFIER => write!(f,"Identifier"),
            TokenType::INVALID => write!(f,"Invalid"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32
}

impl Token {
    // Constructor
    pub fn new(txt: String, tokenType: TokenType, line: i32, ch_pos: i32) -> Token {
        Token {
            text: txt,
            token_type: tokenType,
            line_num: line,
            char_pos: ch_pos
        }
    }

    pub fn get_char_pos(&self) -> i32 {
        self.char_pos
    }

    pub fn get_line_num(&self) -> i32 {
        self.line_num
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}