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
    //let mut to_ex = Token::new(args[1].as_str());
    // while sc_ex.more_avail() {
    //     println!("{:?}", sc_ex.get_next_token());
    // }
    // println!("{:?}", sc_ex.get_all_token());
    sc_ex.get_all_token();
    // println!("{:?}", sc_ex.get_next_token());

    fun_program(ex,sc_ex);

    match fun_program(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_declaration(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_mainDeclaration(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_functionDefinition(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_declarationType(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_variableDeclaration(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_functionDeclaration(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_block(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_parameterBlock(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_dataType(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_constant(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_statement(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_parameter(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_integerType(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_floatType(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_assignment(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_whileLoop(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_ifStatement(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_returnStatement(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_expression(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_simpleExpression(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_term(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_factor(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_relationOperator(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_addOperator(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }
    match fun_multOperator(ex,sc_ex) {
        Ok(desc) => {
            if desc == true {
                true;
            } else {
                false;
            }
        }
        Err(err) => {
            println!("{}",err)
        }
    }

}
    // match fun_program(ex) {
    //     Ok(description) => match description{
    //         true => return true
    //     },
    //     Err(err) => println!("{}", err),
    // }
    

    
    fn fun_program(ex: CStream, mut sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_declaration(ex,sc_ex) == true {
            while !fun_declaration(ex,sc_ex) {

                return Err(MyError::Syntax { ln_num: token_type.get_line_num(), char_num: token_type.get_char_pos(), ebnf: 123 })
            }
            if fun_mainDeclaration(ex,sc_ex) == true&&fun_functionDefinition(ex,sc_ex) == true {
                while !fun_functionDefinition(ex,sc_ex) {
                    return Err(MyError::Syntax { ln_num: token_type.get_line_num(), char_num: token_type.get_char_pos(), ebnf: 123 })
                }
            }
            return Ok(true)
        }
        else{
            return Err(MyError::Syntax { ln_num: token_type.get_line_num(), char_num: token_type.get_char_pos(), ebnf: 123 })
        }
    }

    fn fun_declaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_declarationType(ex,sc_ex) == true&&(fun_variableDeclaration(ex,sc_ex) == true||fun_functionDeclaration(ex,sc_ex) == true){
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_mainDeclaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('v')&&ex.get_next_char()==Some('o')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('d')
        &&ex.get_next_char()==Some('m')&&ex.get_next_char()==Some('a')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('(')&&ex.get_next_char()==Some(')')
        &&fun_block(ex,sc_ex) == Some(true){
           return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_functionDefinition(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_declarationType(ex,sc_ex)==true&&fun_parameterBlock(ex,sc_ex) == true&&fun_block(ex,sc_ex) == true{
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_declarationType(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        /*if fun_dataType(ex) == true{
            match sc_ex.get_next_token(){
                TokenType::IDENTIFIER => return true,
            }
        }
        else{
            return false;
        } */
        let temp_type = sc_ex.get_next_token().unwrap().get_token_type();
        
        if fun_dataType(ex) == true&& *temp_type == TokenType::IDENTIFIER{
            return Ok(true);
        }
        else{
            return false;
        } 
    
    }
    fn fun_variableDeclaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        while !(ex.get_next_char() == Some(';')&& fun_constant(ex,sc_ex) == true){
            return false;
        }
        return Ok(true);
    }

    fn fun_functionDeclaration(ex: CStream,sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_parameterBlock(ex,sc_ex) == true&&ex.get_next_char() == Some(';'){
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_block(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char() == Some('{'){
            while !(fun_declaration(ex,sc_ex) == true&&fun_statement(ex,sc_ex) == true
        &&fun_functionDefinition(ex,sc_ex) == true){
                return false;
            }
        }
        return Ok(true);
    }

    fn fun_parameterBlock(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char() == Some('(')&&fun_parameter(ex,sc_ex) == true&&ex.get_next_char() == Some(',')
        &&fun_parameter(ex,sc_ex) == true&&ex.get_next_char() == Some(')'){
            while !(ex.get_next_char() == Some(',')&&fun_parameter(ex,sc_ex) == true){
                return false;
            }
        }
        return Ok(true);
    }

    fn fun_dataType(ex: CStream)-> Result<bool, MyError>{
        if fun_integerType(ex) == true&&fun_floatType(ex) == true{
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_constant(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let temp_type1 = sc_ex.get_next_token().unwrap().get_token_type();
        let temp_type2 = sc_ex.get_next_token().unwrap().get_token_type();
        if *temp_type1 == TokenType::INTCONSTANT && *temp_type2 == TokenType::FLOATCONSTANT{
            return Ok(true);
        }
        else{
            return false;
        }
    }
    
    fn fun_statement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_assignment(ex,sc_ex) == true&&fun_whileLoop(ex,sc_ex) == true
        &&fun_ifStatement(ex,sc_ex) == true&&fun_returnStatement(ex,sc_ex) == true
        &&fun_expression(ex,sc_ex) == true&&ex.get_next_char() == Some(';'){
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_parameter(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let temp_type = sc_ex.get_next_token().unwrap().get_token_type();
        if fun_dataType(ex) == true && *temp_type == TokenType::IDENTIFIER{
            return Ok(true);
        }
        else{
            return false;
        }
    }

    fn fun_integerType(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('u')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('i')
        &&ex.get_next_char()==Some('g')&&ex.get_next_char()==Some('n')
        &&ex.get_next_char()==Some('e')&&ex.get_next_char()==Some('d'){
            if ex.get_next_char()==Some('c')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('a')&&ex.get_next_char()==Some('r'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('r')
            &&ex.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
            &&ex.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('o')
            &&ex.get_next_char()==Some('n')&&ex.get_next_char()==Some('g'){
                return Ok(true);
            }
            else{
                return false;
            }
            
        }
        if ex.get_next_char()==Some('c')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('a')&&ex.get_next_char()==Some('r'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('s')&&ex.get_next_char()==Some('h')
            &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('r')
            &&ex.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('n')
            &&ex.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('o')
            &&ex.get_next_char()==Some('n')&&ex.get_next_char()==Some('g'){
                return Ok(true);
            }
            else{
                return false;
            }
        
    }

    fn fun_floatType(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('f')&&ex.get_next_char()==Some('l')
        &&ex.get_next_char()==Some('o')&&ex.get_next_char()==Some('a')
        &&ex.get_next_char()==Some('t'){
            return Ok(true);
        }
        else if ex.get_next_char()==Some('d')&&ex.get_next_char()==Some('o')
        &&ex.get_next_char()==Some('u')&&ex.get_next_char()==Some('b')
        &&ex.get_next_char()==Some('l')&&ex.get_next_char()==Some('e'){
            return Ok(true);
        }
        else {
            return false;
        }
    }
    
    fn fun_assignment(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        /*
        let temp_type1 = sc_ex.get_next_token().unwrap().get_token_type();
        let temp_type2 = sc_ex.get_next_token().unwrap().get_token_type();
        let temp_type3 = sc_ex.get_next_token().unwrap().get_token_type();
        if *temp_type1 == TokenType::IDENTIFIER&&ex.get_next_char()==Some('=')
        && *temp_type2 == TokenType::IDENTIFIER&&ex.get_next_char()==Some('='){
            // 这里不是很明白要怎么改，所以我就不改啦
            while *sc_ex.get_next_token().unwrap().get_token_type() == TokenType::IDENTIFIER&&ex.get_next_char()==Some('=') {
                return false;
            } 
            return true;
        }
        return false;
         */
        return Ok(true); 
    }
    
    fn fun_whileLoop(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('w')&&ex.get_next_char()==Some('h')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('l')
        &&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex,sc_ex) == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex,sc_ex) == true {
           return Ok(true);
        }
        else {
            return false;
        }
    }
    
    fn fun_ifStatement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('f')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex,sc_ex) == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex,sc_ex) == true{
           return Ok(true);
        }
        else {
            return false;
        }
    }
    
    fn fun_returnStatement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('t')&&ex.get_next_char()==Some('u')
        &&ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('n')
        &&fun_expression(ex,sc_ex) == true&&ex.get_next_char()==Some(';'){
            return Ok(true);
        }
        else {
            return false;
        }
    }
    
    fn fun_expression(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_simpleExpression(ex,sc_ex) == true {
            while !(fun_multOperator(ex) == true||fun_multOperator(ex)==true) {
                return false;
            }
        }
        return Ok(true);
    }
    
    fn fun_simpleExpression(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_term(ex,sc_ex) == true {
            if fun_addOperator(ex) == true&&fun_term(ex,sc_ex)==true{
                while !fun_addOperator(ex) == true||!fun_term(ex,sc_ex)==true {
                    return false;
                }
            }
            else{
                return false;
            }
            
        }
        return Ok(true);
    }
    
    fn fun_term(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_factor(ex,sc_ex) == true {
            if fun_multOperator(ex) == true||fun_multOperator(ex)==true{
                while !fun_multOperator(ex) == true||!fun_multOperator(ex)==true {
                    return false;
                }
            }
            else{
                return false;
            }
            
        }
        return Ok(true);
    }
    
    fn fun_factor(ex: CStream, sc_ex: Scanner)->Result<bool, MyError>{
        fun_expression(ex,sc_ex);
        let temp_type1 = sc_ex.get_next_token().unwrap().get_token_type();
        let temp_type2 = sc_ex.get_next_token().unwrap().get_token_type();
        if *temp_type1 == TokenType::INTCONSTANT||(*temp_type2 == TokenType::FLOATCONSTANT){
            let temp_type3 = sc_ex.get_next_token().unwrap().get_token_type();
            if *temp_type3 == TokenType::IDENTIFIER {
                if ex.get_next_char()==Some('('){
                    if fun_expression(ex,sc_ex)==true{
                        if ex.get_next_char()==Some(')'){
                                return Ok(true);
                            }
                        }
                    }
                    return Ok(true);
                }
            }
        return false;
    }
    fn fun_relationOperator(ex: CStream)->Result<bool, MyError>{
        if ex.get_next_char()==Some('=')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('<')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('>')&&ex.peek_next_char()==Some('=')
        ||ex.get_next_char()==Some('!')&&ex.peek_next_char()==Some('=')
        {
            ex.get_next_char();
            return Ok(true);
        }
        else if ex.get_next_char()==Some('<')||ex.get_next_char()==Some('>'){
            return Ok(true);
        }
        return false;  
    }
    fn fun_addOperator(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('+')||ex.get_next_char()==Some('-'){
            return Ok(true);
        }
        return false;      
    }
    fn fun_multOperator(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('*')||ex.get_next_char()==Some('/') {
            return Ok(true);
        }  
        return false; 
    }

