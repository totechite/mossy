use std::fs;
use std::io::{stdout, Write, BufWriter, prelude::*};

extern crate regex;


pub mod token;
pub mod lexer;
use lexer::Lexer;
pub mod parser;
use parser::Parser;

fn main(){
    let mut file = fs::File::open("sample.md").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut lexer: Lexer = Lexer::new(contents);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    // writeln!(out, "{:#?}", &lexer.exec()).unwrap();
    writeln!(out, "{:?}", Parser::new(lexer.exec()).exec()).unwrap();
}