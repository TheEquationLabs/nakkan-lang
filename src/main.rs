use lexer;
use parser;
use interpreter;
use ast;


use std::fs;

fn  main(){
    let script_file_path="";
    let code_file=fs::read_to_String(script_file_path)
    .expect("Error occured while reading the code file");

    //tokenize
    let tokens=lexer::tokenize(&code_file)
    .expect("Error occured during tokenization");
    println!("{:?}",tokens);

    //parsing
    let ast=parser::parse(token)
    .expect("Error occured during the parsing");

    //Abstarct Syntax Tree
    println!("{:?}",ast);

    interpreter::interpret(*ast);

}