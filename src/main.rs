#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_assignments, unused_mut))]
use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::Write;

mod cstream;
mod token;
mod scanner;
mod xhtml;

use cstream::*;
use token::*;
use scanner::*;
use xhtml::*;

extern crate custom_error;
use custom_error::custom_error;

custom_error!{MyError
    Syntax{ln_num: i32, char_num: i32, ebnf: String} = 
        "Error at Line {ln_num} Character {char_num}. The syntax should be: {ebnf}"
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args[1]);

    // ==================================================== //
    // ************* Testing for Stage1 ******************* //
    let mut cst = CStream::new(args[1].as_str());
    println!("Contents are: {}", cst.get_content());
    println!("Size is: {}", cst.get_size());
    while cst.more_available() {
        println!("ch: {:?}", cst.get_next_char());
    }

    let tk_ex = Token::new("hello".to_string(), TokenType::KEYWORD, -1, 0);
    println!("{}", tk_ex.get_char_pos());
    println!("{}", tk_ex.get_text());
    println!("{}", tk_ex.get_line_num());
    println!("{}", tk_ex.get_token_type());
    // ***************** End Testing ********************** //
    // ==================================================== //


    // ==================================================== //
    // ************* Testing for Stage2 ******************* //
    let mut sc_ex = Scanner::new(args[1].as_str());
    sc_ex.get_next_token();
    println!("{:?}",sc_ex.get_curr_token());
    // ***************** End Testing ********************** //
    // ==================================================== //

    // ==================================================== //
    // ************* Testing for Stage4 ******************* //
    let mut xhtml = XHTML::new(sc_ex.get_all_token());
    xhtml.all_output();
    let mut file_no = 1;
    let data = xhtml.get_str();
    let path = format!("./example{}.xhtml",file_no);
    file_no+=1;
    let mut f = File::create(path).expect("unable to create file");
    f.write_all(data.as_bytes()).expect("unable to write data");
    println!("{:?}",sc_ex.get_curr_token());
    // ***************** End Testing ********************** //
    // ==================================================== //

    // ==================================================== //
    // *************** Begin of Stage3 ******************** //

    /* Please comment out all of the below to test previous stages */
    /* */
    let mut ex = CStream::new(args[1].as_str());
    let mut sc_ex = Scanner::new(args[1].as_str());
    let mut ex1 = ex.clone();
    let mut sc_ex1 = sc_ex.clone();
    let mut ex2 = ex.clone();
    let mut sc_ex2 = sc_ex.clone();
    let mut ex3 = ex.clone();
    let mut sc_ex3 = sc_ex.clone();
    let mut ex4 = ex.clone();
    let mut sc_ex4 = sc_ex.clone();
    let mut ex5 = ex.clone();
    let mut sc_ex5 = sc_ex.clone();
    let mut ex6 = ex.clone();
    let mut sc_ex6 = sc_ex.clone();
    let mut ex7 = ex.clone();
    let mut sc_ex7 = sc_ex.clone();
    let mut ex8 = ex.clone();
    let mut sc_ex8 = sc_ex.clone();
    let mut ex9 = ex.clone();
    let mut sc_ex9 = sc_ex.clone();
    let mut ex10 = ex.clone();
    let mut sc_ex10 = sc_ex.clone();
    let mut ex11 = ex.clone();
    let mut sc_ex11 = sc_ex.clone();
    let mut ex12 = ex.clone();
    let mut sc_ex12 = sc_ex.clone();
    let mut ex13 = ex.clone();
    let mut sc_ex13 = sc_ex.clone();
    let mut ex14 = ex.clone();
    let mut sc_ex14 = sc_ex.clone();
    let mut ex15 = ex.clone();
    let mut sc_ex15 = sc_ex.clone();
    let mut ex16 = ex.clone();
    let mut sc_ex16 = sc_ex.clone();
    let mut ex17 = ex.clone();
    let mut sc_ex17 = sc_ex.clone();
    let mut ex18 = ex.clone();
    let mut sc_ex18 = sc_ex.clone();
    let mut ex19 = ex.clone();
    let mut sc_ex19 = sc_ex.clone();
    let mut ex20 = ex.clone();
    let mut sc_ex20 = sc_ex.clone();
    let mut ex21 = ex.clone();
    let mut ex22 = ex.clone();
    let mut ex23 = ex.clone();
    let mut ex24 = ex.clone();
    let mut ex25 = ex.clone();
    let mut ex26 = ex.clone();
    //let mut to_ex = Token::new(args[1].as_str());
    // while sc_ex.more_avail() {
    //     println!("{:?}", sc_ex.get_next_token());
    // }
    // println!("{:?}", sc_ex.get_all_token());
    sc_ex.get_all_token();
    // println!("{:?}", sc_ex.get_next_token());

    fun_program(ex,sc_ex);

    match fun_program(ex1, sc_ex1) {
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
    match fun_declaration(ex2,sc_ex2) {
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
    match fun_mainDeclaration(ex3,sc_ex3) {
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
    match fun_functionDefinition(ex4,sc_ex4) {
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
    match fun_declarationType(ex5,sc_ex5) {
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
    match fun_variableDeclaration(ex6,sc_ex6) {
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
    match fun_functionDeclaration(ex7,sc_ex7) {
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
    match fun_block(ex8,sc_ex8) {
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
    match fun_parameterBlock(ex9,sc_ex9) {
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
    match fun_dataType(ex10) {
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
    match fun_constant(ex11,sc_ex10) {
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
    match fun_statement(ex12,sc_ex11) {
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
    match fun_parameter(ex13,sc_ex12) {
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
    match fun_integerType(ex14) {
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
    match fun_floatType(ex15) {
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
    match fun_assignment(ex16,sc_ex13) {
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
    match fun_whileLoop(ex17,sc_ex14) {
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
    match fun_ifStatement(ex18,sc_ex15) {
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
    match fun_returnStatement(ex19,sc_ex16) {
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
    match fun_expression(ex20,sc_ex17) {
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
    match fun_simpleExpression(ex21,sc_ex18) {
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
    match fun_term(ex22,sc_ex19) {
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
    match fun_factor(ex23,sc_ex20) {
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
    match fun_relationOperator(ex24) {
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
    match fun_addOperator(ex25) {
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
    match fun_multOperator(ex26) {
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
    

    
    fn fun_program(mut ex: CStream, mut sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        let mut ex3 = ex.clone();
        let mut sc_ex3 = sc_ex.clone();
        let mut ex4 = ex.clone();
        let mut sc_ex4 = sc_ex.clone();
        let mut ex5 = ex.clone();
        let mut sc_ex5 = sc_ex.clone();
        let mut sc_ex6 = sc_ex.clone();
        let mut ex7 = ex.clone();
        let mut sc_ex7 = sc_ex.clone();
        let mut sc_ex8 = sc_ex.clone();
        if fun_declaration(ex,sc_ex).unwrap() == true {
            
            while !fun_declaration(ex2,sc_ex2).unwrap() {
                let mut curr = sc_ex3.get_curr_token().unwrap();
                //let mut to_ex = Token::new(ex.get_content(), sc_ex.get_curr_token(), curr.get_line_num(), curr.get_char_pos());
                return Err(MyError::Syntax { ln_num: curr.get_line_num(), char_num: curr.get_char_pos(), ebnf: "Program := {DeclaRATION} mAINdECLARATION {fUNCTIONdEFINITION}".to_string()});
            }
            if fun_mainDeclaration(ex4,sc_ex5).unwrap() == true&&fun_functionDefinition(ex3,sc_ex4).unwrap() == true {
                while !fun_functionDefinition(ex7,sc_ex7).unwrap() {
                    let mut curr = sc_ex6.get_curr_token().unwrap();
                    return Err(MyError::Syntax { ln_num: curr.get_line_num(), char_num: curr.get_char_pos(), ebnf: "Program := {DeclaRATION} mAINdECLARATION {fUNCTIONdEFINITION}".to_string() })
                }
            }
            return Ok(true)
        }
        else{
            let mut curr = sc_ex8.get_curr_token().unwrap();
            return Err(MyError::Syntax { ln_num: curr.get_line_num(), char_num: curr.get_char_pos(), ebnf: "Program := {DeclaRATION} mAINdECLARATION {fUNCTIONdEFINITION}".to_string() })
        }
    }

    fn fun_declaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        if fun_declarationType(ex,sc_ex).unwrap() == true&&(fun_variableDeclaration(ex1,sc_ex1).unwrap() == true||fun_functionDeclaration(ex2,sc_ex2).unwrap() == true){
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_mainDeclaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        if ex1.get_next_char()==Some('v')&&ex1.get_next_char()==Some('o')
        &&ex1.get_next_char()==Some('i')&&ex1.get_next_char()==Some('d')
        &&ex1.get_next_char()==Some('m')&&ex1.get_next_char()==Some('a')
        &&ex1.get_next_char()==Some('i')&&ex1.get_next_char()==Some('n')
        &&ex1.get_next_char()==Some('(')&&ex1.get_next_char()==Some(')')
        &&fun_block(ex,sc_ex).unwrap() ==true{
           return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_functionDefinition(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
   
        if fun_declarationType(ex,sc_ex).unwrap()==true&&fun_parameterBlock(ex1,sc_ex1).unwrap() == true&&fun_block(ex2,sc_ex2).unwrap() == true{
            return Ok(true);
        }
        else{
            return Ok(false);
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
        let mut sc_ex1 = sc_ex.clone();
        let temp_type = sc_ex1.get_next_token().unwrap().get_token_type();
        
        if fun_dataType(ex).unwrap() == true&& *temp_type == TokenType::IDENTIFIER{
            return Ok(true);
        }
        else{
            return Ok(false);
        } 
    
    }
    fn fun_variableDeclaration(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        while !(ex1.get_next_char() == Some(';')&& fun_constant(ex,sc_ex).unwrap() == true){
            return Ok(false);
        }
        return Ok(true);
    }

    fn fun_functionDeclaration(ex: CStream,sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        if fun_parameterBlock(ex,sc_ex).unwrap() == true&&ex1.get_next_char() == Some(';'){
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_block(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        let mut ex3 = ex.clone();
        let mut sc_ex3 = sc_ex.clone();
        if ex1.get_next_char() == Some('{'){
            while !(fun_declaration(ex,sc_ex).unwrap() == true&&fun_statement(ex2,sc_ex2).unwrap() == true
            &&fun_functionDefinition(ex3,sc_ex3).unwrap() == true){
                return Ok(false);
            }
        }
        return Ok(true);
    }

    fn fun_parameterBlock(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        let mut ex3 = ex.clone();
        let mut sc_ex3 = sc_ex.clone();
        let mut ex4 = ex.clone();
        let mut sc_ex4 = sc_ex.clone();
        let mut ex5 = ex.clone();
        let mut sc_ex5 = sc_ex.clone();
        let mut ex6 = ex.clone();
        let mut sc_ex6 = sc_ex.clone();
        
        if ex1.get_next_char() == Some('(')&&fun_parameter(ex,sc_ex).unwrap() == true&&ex2.get_next_char() == Some(',')
        &&fun_parameter(ex3,sc_ex1).unwrap() == true&&ex4.get_next_char() == Some(')'){
            while !(ex5.get_next_char() == Some(',')&&fun_parameter(ex6,sc_ex6).unwrap() == true){
                return Ok(false);
            }
        }
        return Ok(true);
    }

    fn fun_dataType(ex: CStream)-> Result<bool, MyError>{
        let mut ex7 = ex.clone();
        
        if fun_integerType(ex).unwrap() == true&&fun_floatType(ex7).unwrap() == true{
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_constant(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        
        let mut sc_ex7 = sc_ex.clone();
        let mut sc_ex8 = sc_ex.clone();
        
        let temp_type1 = sc_ex7.get_next_token().unwrap().get_token_type();
        let temp_type2 = sc_ex8.get_next_token().unwrap().get_token_type();
        if *temp_type1 == TokenType::INTCONSTANT && *temp_type2 == TokenType::FLOATCONSTANT{
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }
    
    fn fun_statement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex7 = ex.clone();
        let mut sc_ex7 = sc_ex.clone();
        let mut ex8 = ex.clone();
        let mut sc_ex8 = sc_ex.clone();
        let mut ex9 = ex.clone();
        let mut sc_ex9 = sc_ex.clone();
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        let mut ex3 = ex.clone();
        let mut sc_ex3 = sc_ex.clone();
        if fun_assignment(ex7,sc_ex7).unwrap() == true&&fun_whileLoop(ex1,sc_ex1).unwrap() == true
        &&fun_ifStatement(ex8,sc_ex8).unwrap() == true&&fun_returnStatement(ex2,sc_ex2).unwrap() == true
        &&fun_expression(ex9,sc_ex9).unwrap() == true&&ex.get_next_char() == Some(';'){
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_parameter(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex7 = ex.clone();
        let mut sc_ex7 = sc_ex.clone();
        let temp_type = sc_ex7.get_next_token().unwrap().get_token_type();
        if fun_dataType(ex).unwrap() == true && *temp_type == TokenType::IDENTIFIER{
            return Ok(true);
        }
        else{
            return Ok(false);
        }
    }

    fn fun_integerType(ex: CStream)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
    
    let mut ex2 = ex.clone();
    let mut ex3 = ex.clone();
    let mut ex4 = ex.clone();
    let mut ex5 = ex.clone();
    let mut ex6 = ex.clone();
    let mut ex7 = ex.clone();
    let mut ex8 = ex.clone();
    let mut ex9 = ex.clone();
    let mut ex10 = ex.clone();
    let mut ex11 = ex.clone();
    let mut ex12 = ex.clone();
    let mut ex13 = ex.clone();
    let mut ex14 = ex.clone();
    let mut ex15 = ex.clone();
    let mut ex16 = ex.clone();
    let mut ex17 = ex.clone();
    let mut ex18 = ex.clone();
    let mut ex19 = ex.clone();
    let mut ex20 = ex.clone();
    let mut ex21 = ex.clone();
    let mut ex22 = ex.clone();
    let mut ex23 = ex.clone();
    let mut ex24 = ex.clone();
    let mut ex25 = ex.clone();
    let mut ex26 = ex.clone();
    let mut ex27 = ex.clone();
    let mut ex28 = ex.clone();
    let mut ex29 = ex.clone();
    let mut ex30 = ex.clone();
    let mut ex31 = ex.clone();
    let mut ex32 = ex.clone();
    let mut ex33 = ex.clone();
    let mut ex34 = ex.clone();
    let mut ex35 = ex.clone();
    let mut ex36 = ex.clone();
    let mut ex37 = ex.clone();
    let mut ex38 = ex.clone();
    let mut ex39 = ex.clone();
    let mut ex40 = ex.clone();
        if ex1.get_next_char()==Some('u')&&ex5.get_next_char()==Some('n')
        &&ex2.get_next_char()==Some('s')&&ex6.get_next_char()==Some('i')
        &&ex3.get_next_char()==Some('g')&&ex7.get_next_char()==Some('n')
        &&ex4.get_next_char()==Some('e')&&ex8.get_next_char()==Some('d'){
            if ex9.get_next_char()==Some('c')&&ex10.get_next_char()==Some('h')
            &&ex11.get_next_char()==Some('a')&&ex12.get_next_char()==Some('r'){
                return Ok(true);
            }
            else if ex13.get_next_char()==Some('s')&&ex14.get_next_char()==Some('h')
            &&ex15.get_next_char()==Some('o')&&ex16.get_next_char()==Some('r')
            &&ex17.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex18.get_next_char()==Some('i')&&ex19.get_next_char()==Some('n')
            &&ex20.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex21.get_next_char()==Some('l')&&ex22.get_next_char()==Some('o')
            &&ex23.get_next_char()==Some('n')&&ex24.get_next_char()==Some('g'){
                return Ok(true);
            }
            else{
                return Ok(false);
            }
            
        }
        if ex25.get_next_char()==Some('c')&&ex26.get_next_char()==Some('h')
            &&ex27.get_next_char()==Some('a')&&ex28.get_next_char()==Some('r'){
                return Ok(true);
            }
            else if ex29.get_next_char()==Some('s')&&ex30.get_next_char()==Some('h')
            &&ex31.get_next_char()==Some('o')&&ex32.get_next_char()==Some('r')
            &&ex33.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex34.get_next_char()==Some('i')&&ex35.get_next_char()==Some('n')
            &&ex36.get_next_char()==Some('t'){
                return Ok(true);
            }
            else if ex37.get_next_char()==Some('l')&&ex38.get_next_char()==Some('o')
            &&ex39.get_next_char()==Some('n')&&ex40.get_next_char()==Some('g'){
                return Ok(true);
            }
            else{
                return Ok(false);
            }
        
    }

    fn fun_floatType(ex: CStream)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut ex2 = ex.clone();
        let mut ex3 = ex.clone();
        let mut ex4 = ex.clone();
        let mut ex5 = ex.clone();
        let mut ex6 = ex.clone();
        let mut ex7 = ex.clone();
        let mut ex8 = ex.clone();
        let mut ex9 = ex.clone();
        let mut ex10 = ex.clone();
        let mut ex11 = ex.clone();
        if ex1.get_next_char()==Some('f')&&ex2.get_next_char()==Some('l')
        &&ex3.get_next_char()==Some('o')&&ex4.get_next_char()==Some('a')
        &&ex5.get_next_char()==Some('t'){
            return Ok(true);
        }
        else if ex6.get_next_char()==Some('d')&&ex7.get_next_char()==Some('o')
        &&ex8.get_next_char()==Some('u')&&ex9.get_next_char()==Some('b')
        &&ex10.get_next_char()==Some('l')&&ex11.get_next_char()==Some('e'){
            return Ok(true);
        }
        else {
            return Ok(false);
        }
    }
    
    fn fun_assignment(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        let mut ex1 = ex.clone();
        let mut sc_ex1 = sc_ex.clone();
        let mut ex2 = ex.clone();
        let mut sc_ex2 = sc_ex.clone();
        let mut ex3 = ex.clone();
        let mut sc_ex3 = sc_ex.clone();
        let mut sc_ex4 = sc_ex.clone();
        let temp_type1 = sc_ex1.get_next_token().unwrap().get_token_type();
        let temp_type2 = sc_ex2.get_next_token().unwrap().get_token_type();
        let temp_type3 = sc_ex3.get_next_token().unwrap().get_token_type();
        if *temp_type1 == TokenType::IDENTIFIER&&ex1.get_next_char()==Some('=')
        && *temp_type2 == TokenType::IDENTIFIER&&ex2.get_next_char()==Some('='){
            while *sc_ex4.get_next_token().unwrap().get_token_type() == TokenType::IDENTIFIER&&ex.get_next_char()==Some('=') {
                return Ok(false);
            } 
            return Ok(true);
        }
        return Ok(false);
        
    }
    
    fn fun_whileLoop(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('w')&&ex.get_next_char()==Some('h')
        &&ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('l')
        &&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex,sc_ex).unwrap() == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex,sc_ex).unwrap() == true {
           return Ok(true);
        }
        else {
            return Ok(false);
        }
    }
    
    fn fun_ifStatement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('i')&&ex.get_next_char()==Some('f')
        &&ex.get_next_char()==Some('(')&&fun_expression(ex,sc_ex).unwrap() == true
        &&ex.get_next_char()==Some(')')&&fun_block(ex,sc_ex).unwrap() == true{
           return Ok(true);
        }
        else {
            return Ok(false);
        }
    }
    
    fn fun_returnStatement(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('e')
        &&ex.get_next_char()==Some('t')&&ex.get_next_char()==Some('u')
        &&ex.get_next_char()==Some('r')&&ex.get_next_char()==Some('n')
        &&fun_expression(ex,sc_ex).unwrap() == true&&ex.get_next_char()==Some(';'){
            return Ok(true);
        }
        else {
            return Ok(false);
        }
    }
    
    fn fun_expression(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_simpleExpression(ex,sc_ex).unwrap() == true {
            while !(fun_multOperator(ex).unwrap() == true||fun_multOperator(ex).unwrap()==true) {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    
    fn fun_simpleExpression(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_term(ex,sc_ex).unwrap() == true {
            if fun_addOperator(ex).unwrap() == true&&fun_term(ex,sc_ex).unwrap()==true{
                while !fun_addOperator(ex).unwrap() == true||!fun_term(ex,sc_ex).unwrap()==true {
                    return Ok(false);
                }
            }
            else{
                return Ok(false);
            }
            
        }
        return Ok(true);
    }
    
    fn fun_term(ex: CStream, sc_ex: Scanner)-> Result<bool, MyError>{
        if fun_factor(ex,sc_ex).unwrap() == true {
            if fun_multOperator(ex).unwrap() == true||fun_multOperator(ex).unwrap()==true{
                while !fun_multOperator(ex).unwrap() == true||!fun_multOperator(ex).unwrap()==true {
                    return Ok(false);
                }
            }
            else{
                return Ok(false);
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
                    if fun_expression(ex,sc_ex).unwrap()==true{
                        if ex.get_next_char()==Some(')'){
                                return Ok(true);
                            }
                        }
                    }
                    return Ok(true);
                }
            }
        return Ok(false);
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
        return Ok(false);  
    }
    fn fun_addOperator(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('+')||ex.get_next_char()==Some('-'){
            return Ok(true);
        }
        return Ok(false);      
    }
    fn fun_multOperator(ex: CStream)-> Result<bool, MyError>{
        if ex.get_next_char()==Some('*')||ex.get_next_char()==Some('/') {
            return Ok(true);
        }  
        return Ok(false); /* */
    }

