use std::io::{BufRead, Cursor};
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
            }
            else if Regex::new(r"^(#{1,6})\s").unwrap().is_match(head_line.as_str()) {
                let depth: i8 = Regex::new("#").unwrap().find_iter(head_line.as_str()).count() as i8;
                let text: String = Regex::new("#").unwrap().replace_all(head_line.as_str(), "").trim().to_string();
                tokens.push(Token::Heading{depth: depth,text: text});
            }
            else if Regex::new(r"^\B(`{3})").unwrap().is_match(head_line.as_str()) {
                let mut find_lang: Option<_> = Regex::new(r"[\w[0-9]]+").unwrap().captures(head_line.as_str());
                let mut lang = String::new();
                if let Some(option) = find_lang{
                    lang = option.get(0).unwrap().as_str().to_string();
                }else{
                    lang = "".to_string();
                }
                self.consume();
                let code_token: Token = self.code_token(lang);
                tokens.push(code_token);
            }                                                                                                                                                                                                                                                                                           
            // else if Regex::new(r"^[-*]\s\w+").unwrap().is_match(head_line.as_str()) {
            //     let mut ordered: bool = false;
            //     self.list_token(ordered);
            // }
            else {
                let text: String = Regex::new(r"\s{3}\n*?$").unwrap().replace_all(head_line.as_str(), "\n").to_string();
                tokens.push(Token::Paragraph{text: text});
            }
            self.consume();
        }
        tokens.push(Token::EOF);
        tokens
    }

    // fn list_token(&mut self, ordered?: bool) -> Token{

    // }

    fn code_token(&mut self, lang: String) -> Token{
        let mut is_code: bool = true;
        let mut text = String::new();
        while is_code{
            if Regex::new(r"^\B(`{3})").unwrap().is_match(self.line.clone().unwrap_or("".to_string()).as_str()){
                is_code = false;
            }else{
                text += &self.line.clone().unwrap_or("\n".to_string());
            };
            &self.consume();
        }
        Token::Code{lang: lang, text: text}
    }

    fn inline(self, content: String) -> String{
        match content{
            _ => {}
        }
        String::new()
    }

}