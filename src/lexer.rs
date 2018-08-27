#![feature(rc_unique)]

use std::rc::Rc;
use std::io::{BufRead, Cursor};
use std::collections::HashMap;
use regex::Regex;
use token::Token;

#[derive(Debug, Clone)]
pub struct Lexer {
    cursor: Cursor<String>,
    pub line: Option<String>,
}

impl Lexer {

    pub fn new(src: String) -> Lexer{
        let mut cursor = Cursor::new(src);
        let mut buf = String::new();
        cursor.read_line(&mut buf);
        Lexer{
            cursor: cursor,
            line: Some(buf)
        }
    }

    fn consume(&mut self) -> &mut Lexer{
            let mut temp: String = "".to_string();
            self.cursor.read_line(&mut temp);
            match temp.as_str(){
                "" => self.line = None,
                x => self.line = Some(x.to_string())
            }
            self
    }
    
    pub fn exec(&mut self) -> Vec<Token>{
        let mut tokens = vec![];
        while let Some(head_line) = self.line.clone(){
            if "\n" == head_line.as_str(){
                // Nothing to do
                self.consume();
            }
            // Heading
            else if Regex::new(r"^(#{1,6})\s").unwrap().is_match(head_line.as_str()) {
                let depth: i8 = Regex::new("#").unwrap().find_iter(head_line.as_str()).count() as i8;
                let text: String = Regex::new("#").unwrap().replace_all(head_line.as_str(), "").trim().to_string();
                tokens.push(Token::Heading{depth: depth,text: text});
                self.consume();
            }
            // Code
            else if Regex::new(r"^\B[(`{3})(\t{1})]").unwrap().is_match(head_line.as_str()) {
                let mut find_lang: Option<_> = Regex::new(r"[\w[0-9]]+").unwrap().captures(head_line.as_str());
                let mut lang: String = String::new();
                if let Some(option) = find_lang{
                    lang = option.get(0).unwrap().as_str().to_string();
                }else{
                    lang = "".to_string();
                }
                self.consume();
                let code_token: Token = self.code_token(lang);
                tokens.push(code_token);
                self.consume();
            }
            // List                                                                                                                                                                                                                                                                                           
            else if Regex::new(r"^[-+*]\s+[\W[[:punct:]]\w]").unwrap().is_match(head_line.as_str()) {
                tokens.append(&mut self.list_token(false));
            }
            // OrderedList
            else if Regex::new(r"^([0-9]+.)\s+[\W[[:punct:]]\w]").unwrap().is_match(head_line.as_str()) {
                tokens.append(&mut self.list_token(true));
            }
            //Blockquote
            else if Regex::new(r"^>").unwrap().is_match(head_line.as_str()){
                tokens.push(Token::BlockquoteStart);
                let mut parags: String;
                parags = Regex::new(r"\s{2,}$").unwrap().replace(Regex::new(r"\n$").unwrap().replace(Regex::new(r"^>+").unwrap().replace(head_line.as_str(), "").to_string().as_str(), "").to_string().as_str(), "\n").trim().to_string();
                self.consume();
                let mut frag = true;
                while frag{
                    if Regex::new(r"^>").unwrap().is_match(self.line.clone().unwrap().as_str()){
                        parags += Regex::new(r"\s{2,}$").unwrap().replace(Regex::new(r"\n$").unwrap().replace(Regex::new(r"^>+").unwrap().replace(self.line.clone().unwrap().as_str(), "").to_string().as_str(), "").to_string().as_str(), "\n").trim();
                        self.consume();
                    }else{
                        frag = false;
                    };
                };
                tokens.push(Token::Paragraph{text: parags});
                tokens.push(Token::BlockquoteEnd);
            }
            // Paragraph
            else {
                let text: String = Regex::new(r"\s{3}\n*?$").unwrap().replace_all(head_line.as_str(), "\n").to_string();
                self.consume();
                if let Some(line) = self.line.clone(){
                    if Regex::new(r"^([=-]{2,})\n$").unwrap().is_match(line.as_str()){
                        if Regex::new(r"^(={2,})\n$").unwrap().is_match(self.line.clone().unwrap().as_str()){
                            tokens.push(Token::Heading{depth: 1, text: text});
                            self.consume();
                        }else{
                            tokens.push(Token::Heading{depth: 2, text: text});
                            self.consume();
                        };
                    }else {
                        tokens.push(Token::Paragraph{text: text});
                    }
                }else{
                    tokens.push(Token::Paragraph{text: text});
                };

            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn list_token(&mut self, ordered: bool) -> Vec<Token>{
        let mut stack = vec![];
        stack.push(Token::ListStart{ordered: ordered});
        while Regex::new(r"^[-+*[0-9]+].?\s+[\W[[:punct:]]\w]").unwrap().is_match(self.line.clone().unwrap().as_str()){
            let text: String = Regex::new(r"^[-+*[0-9]+].?\s+").unwrap().replace_all(self.line.clone().unwrap().as_str(), "").trim().to_string();
            stack.push(Token::ListItem{text: text, task: false, checked: false});
            self.consume();
        }
        stack.push(Token::ListEnd);
        stack
    }

    fn code_token(&mut self, lang: String) -> Token{
        let mut is_code: bool = true;
        let mut text = String::new();
        while is_code{
            if Regex::new(r"^\B[(`{3})(\t{1})]").unwrap().is_match(self.line.clone().unwrap_or("".to_string()).as_str()){
                is_code = false;
            }else{
                text += &self.line.clone().unwrap_or("\n".to_string());
            };
            &self.consume();
        }
        Token::Code{lang: lang, text: text}
    }

}