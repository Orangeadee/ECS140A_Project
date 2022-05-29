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

    fun_program(ex);



    
    fn fun_program(ex: CStream)-> bool{
        if fun_declaration(ex) == true {
            while !fun_declaration(ex) {
                return false;
            }
            if fun_mainDeclaration(ex) == true&&fun_functionDefinition(ex) == true {
                while !fun_functionDefinition(ex) {
                    return false;
                }
            }
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_declaration(ex: CStream)-> bool{
        if fun_declarationType(ex) == true&&(fun_variableDeclaration(ex) == true||fun_functionDeclaration(ex) == true){
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_mainDeclaration(ex: CStream)-> bool{
        if ex.get_next_char()==Some('v')&&ex.get_next_char()==Some('o')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('d')
        &&ex.get_next_char()==Some('m')&&ex.get_next_char()==Some('a')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('(')&&ex.get_next_char()==Some(')')
        &&fun_block(ex) == true{
           return true;
        }
        else{
            return false;
        }
    }

    fn fun_functionDefinition(ex: CStream)-> bool{
        if fun_declarationType(ex)==true&&fun_parameterBlock(ex) == true&&fun_block(ex) == true{
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_declarationType(ex: CStream)-> bool{
        /*if fun_dataType(ex) == true{
            match sc_ex.get_next_token(){
                TokenType::IDENTIFIER => return true,
            }
        }
        else{
            return false;
        } */
        
        
        if fun_dataType(ex) == true&&sc_ex.get_next_token() == TokenType::IDENTIFIER{
            return true;
        }
        else{
            return false;
        } 
    
    }
    fn fun_variableDeclaration(ex: CStream)-> bool{
        while !(ex.get_next_char() == Some(';')&& fun_constant(ex) == true){
            return false;
        }
        return true;
    }

    fn fun_functionDeclaration(ex: CStream)-> bool{
        if fun_parameterBlock(ex) == true&&ex.get_next_char() == Some(';'){
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_block(ex: CStream)-> bool{
        if ex.get_next_char() == Some('{'){
            while !(fun_declaration(ex) == true&&fun_statement(ex) == true
        &&fun_functionDefinition(ex) == true){
                return false;
            }
        }
        return true;
    }

    fn fun_parameterBlock(ex: CStream)-> bool{
        if ex.get_next_char() == Some('(')&&fun_parameter(ex) == true&&ex.get_next_char() == Some(',')
        &&fun_parameter(ex) == true&&ex.get_next_char() == Some(')'){
            while !(ex.get_next_char() == Some(',')&&fun_parameter(ex) == true){
                return false;
            }
        }
        return true;
    }

    fn fun_dataType(ex: CStream)-> bool{
        if fun_integerType(ex) == true&&fun_floatType(ex) == true{
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_constant(ex: CStream)-> bool{
        if sc_ex.get_next_token() == TokenType::INTCONSTANT&&sc_ex.get_next_token() == TokenType::FLOATCONSTANT{
            return true;
        }
        else{
            return false;
        }
    }
    
    fn fun_statement(ex: CStream)-> bool{
        if fun_assignment(ex) == true&&fun_whileLoop(ex) == true
        &&fun_ifStatement(ex) == true&&fun_returnStatement(ex) == true
        &&fun_expression(ex) == true&&ex.get_next_char() == Some(';'){
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_parameter(ex: CStream)-> bool{
        if fun_dataType(ex) == true&&sc_ex.get_next_token() == TokenType::IDENTIFIER{
            return true;
        }
        else{
            return false;
        }
    }

    fn fun_integerType(ex: CStream)-> bool{
        if ex.get_next_char()==Some('u')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('i')
        &&ex.get_next_char()==Some('g')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('e')&&ex.get_next_char()==Some('d'){
            if ex.get_next_char()==Some('c')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('a')&&ex.get_next_char()==Some('r'){
                return true;
            }
            else if ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('r')
            &&ex.get_next_char()==Some('t'){
                return true;
            }
            else if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
            &&ex.get_next_char()==Some('t'){
                return true;
            }
            else if ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('o')
            &&ex.get_next_char()==Some('n')&&ex.get_next_char()==Some('g'){
                return true;
            }
            else{
                return false;
            }
            
        }
        if ex.get_next_char()==Some('c')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('a')&&ex.get_next_char()==Some('r'){
                return true;
            }
            else if ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('r')
            &&ex.get_next_char()==Some('t'){
                return true;
            }
            else if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
            &&ex.get_next_char()==Some('t'){
                return true;
            }
            else if ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('o')
            &&ex.get_next_char()==Some('n')&&ex.get_next_char()==Some('g'){
                return true;
            }
            else{
                return false;
            }
        
    }

    fn fun_floatType(ex: CStream)-> bool{
        if ex.get_next_char()==Some('f')&&ex.get_next_char()==Some('l')
        &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('a')
        &&ex.get_next_char()==Some('t'){
            return true;
        }
        else if ex.get_next_char()==Some('d')&&ex.get_next_char()==Some('o')
        &&ex.get_next_char()==Some('u')&&ex.get_next_char()==Some('b')
        &&ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('e'){
            return true;
        }
        else {
            return false;
        }
    }
    
    fn fun_assignment(ex: CStream)-> bool{
        if sc_ex.get_next_token() == TokenType::IDENTIFIER&&ex.get_next_char()==Some('=')
        &&sc_ex.get_next_token() == TokenType::IDENTIFIER&&ex.get_next_char()==Some('='){
            while !sc_ex.get_next_token() == TokenType::IDENTIFIER&&ex.get_next_char()==Some('=') {
                return false;
            } 
            return true;
        }
            
    }
    
    fn fun_whileLoop(ex: CStream)-> bool{
        if ex.get_next_char()==Some('w')&&ex.get_next_char()==Some('h')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('l')
        &&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex) == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex) == true {
           return true;
        }
        else {
            return false;
        }
    }
    
    fn fun_ifStatement(ex: CStream)-> bool{
        if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('f')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex) == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex) == true{
           return true;
        }
        else {
            return false;
        }
    }
    
    fn fun_returnStatement(ex: CStream)-> bool{
        if ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('t')&&ex.get_next_char()==Some('u')
        &&ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('n')
        &&fun_expression(ex) == true&&ex.get_next_char()==Some(';'){
            return true;
        }
        else {
            return false;
        }
    }
    
    fn fun_expression(ex: CStream)-> bool{
        if fun_simpleExpression(ex) == true {
            while !(fun_multOperator(ex) == true||fun_multOperator(ex)==true) {
                return false;
            }
        }
        return true;
    }
    
    fn fun_simpleExpression(ex: CStream)-> bool{
        if fun_term(ex) == true {
            if fun_addOperator(ex) == true&&fun_term(ex)==true{
                while !fun_addOperator(ex) == true||!fun_term(ex)==true {
                    return false;
                }
            }
            else{
                return false;
            }
            
        }
        return true;
    }
    
    fn fun_term(ex: CStream)-> bool{
        if fun_factor(ex) == true {
            if fun_multOperator(ex) == true||fun_multOperator(ex)==true{
                while !fun_multOperator(ex) == true||!fun_multOperator(ex)==true {
                    return false;
                }
            }
            else{
                return false;
            }
            
        }
        return true;
    }
    
    fn fun_factor(ex: CStream)->bool{
        fun_expression(ex);

        if sc_ex.get_next_token() == TokenType::INTCONSTANT||(sc_ex.get_next_token() == TokenType::FLOATCONSTANT){
            if sc_ex.get_next_token == TokenType::IDENTIFIER {
                if ex.get_next_char()==Some('('){
                    if fun_expression(ex)==true{
                        if ex.get_next_char()==Some(')'){
                                return true;
                            }
                        }
                    }
                    return true;
                }
            }
        return false;
    }
    fn fun_relationOperator(ex: CStream)->bool{
        if ex.get_next_char()==Some('=')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('<')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('>')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('!')&&ex.peek_next_char()==Some('=')
        {
            ex.get_next_char();
            return true;
        }
        else if ex.get_next_char()==Some('<')||ex.get_next_char()==Some('>'){
            return true;
        }
        return false;  
    }
    fn fun_addOperator(ex: CStream)-> bool{
        if ex.get_next_char()==Some('+')||ex.get_next_char()==Some('-'){
            return true;
        }
        return false;      
    }
    fn fun_multOperator(ex: CStream)-> bool{
        if ex.get_next_char()==Some('*')||ex.get_next_char()==Some('/') {
            return true;
        }  
        return false; 
    }
}
