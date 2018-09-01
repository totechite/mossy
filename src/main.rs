use std::fs;
use std::env;
use std::io::{ Write, prelude::*};
extern crate regex;
use regex::Regex;

pub mod token;
pub mod lexer;
use lexer::Lexer;
pub mod parser;
use parser::Parser;

fn main() -> std::io::Result<()> {
//  Get the Path to markdownFile.
	let filepath: Option<String> = if env::args().count() == 2usize{env::args().nth(1)}else{None};
	let mut filepath: String = filepath.unwrap();
    if !Regex::new(r"(.md)$").unwrap().is_match(filepath.as_str()){
    	filepath += ".md";
    };
    let mut file = fs::File::open(&filepath.as_str()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lexer: Lexer = Lexer::new(contents);//gen tokens.
	let tokens: String = Parser::new(lexer.exec()).exec();	//parse markdown_tokens to HTML.
	let mut write_buffer = fs::File::create(Regex::new(r"(.md)$").unwrap().replace(&filepath.as_str(), ".html").trim().to_string())?;
	write_buffer.write(&tokens.as_bytes())?;
	Ok(())
}