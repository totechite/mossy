use token::Token;
use regex::Regex;
use std::slice::Iter;
#[derive(Debug, Clone)]
pub struct Parser {
    point: usize,
    tokens: Vec<Token>,
    token: Option<Token>
}

impl Parser {

    pub fn new(tokens: Vec<Token>)-> Parser{
        let tokens = tokens;
        let token = tokens.clone().iter().nth(0).unwrap().to_owned();
        Parser{
            point: 0usize,
            tokens: tokens,
            token: Some(token)
        }
    }

    fn next(&mut self) {
        self.point+=1usize;
        match self.tokens.iter().nth(self.point) {
            Some(token) => {
                self.token = Some(token.to_owned());
            },
            None => {
                self.token = None;
            }
        }
    }

    pub fn exec(&mut self) -> String{
        let mut output = String::new();
        while let Some(token) = self.token.clone() {
            match token{
                Token::Heading{ depth, text } => {
                    let text = self.clone().inline_parser(text.to_owned());
                    let s = format!("<h{}>{}</h{}>\n", depth, text, depth);
                    output += s.as_str();
                    self.next();
                },
                Token::Paragraph{ text } => {
                    let text = self.clone().inline_parser(text.to_owned()).trim().to_string();
                    let s = format!("<p>{}</p>\n", text);
                    output += s.as_str();
                    self.next();
                },
                Token::Code{lang, text} => {
                    let mut s = if lang.to_owned()==String::from(""){ String::from("") }else{ format!(" class=\"{}\"", lang)  };
                    s = format!("<pre><code{}>\n{}\n</code></pre>\n", s, text);
                    output += s.as_str();
                    self.next();
                },
                Token::ListStart{ordered} => {
                    let list_parent: String = if ordered.to_owned() { String::from("ol") }else { String::from("ul")};
                    self.next();
                    output+=format!("<{}>\n", &list_parent).as_str();
                    let li_tags: String = String::new();
                    let mut is_inList: bool = true;
                    while is_inList{
                        match self.token.clone().unwrap() {
                            Token::ListItem{text, task, checked} => {
                                output+=format!("    <li>{}</li>\n", self.clone().inline_parser(text)).as_str();
                                self.next();
                            },
                            Token::ListEnd => {
                                output+=format!("</{}>\n", list_parent).as_str();
                                is_inList = false;
                                self.next();
                            },
                            _ => is_inList = false,
                        }
                    }  
                },
                _ => {
                    self.next();
                }
            }
        }
        output
    }

    fn inline_parser(self, text: String) -> String{
        if Regex::new(r"(`+)\B").unwrap().is_match(text.as_str()){
            if Regex::new(r"`<\w+>`").unwrap().is_match(text.as_str()){
                let e = Regex::new(r"`<\w+>`").unwrap().captures(text.as_str()).unwrap().get(0).unwrap().as_str();
                let pat = Regex::new(r"\w+").unwrap().captures(e).unwrap().get(0).unwrap().as_str();
                let replace_string: String = format!("&lt;{}&gt;", pat);
                self.inline_parser(Regex::new(r"<\w+>").unwrap().replace(text.as_str(), replace_string.as_str()).to_string())
            }else{
                let f = Regex::new(r"\B(`+)").unwrap().replace(text.as_str(), "<code>").to_string();
                self.inline_parser(Regex::new(r"(`+)\B").unwrap().replace(f.as_str(), "</code>").to_string())
            }
        }else if Regex::new(r"[_*][[:word:]]+[_*]").unwrap().is_match(text.as_str()) {
            let f = Regex::new(r"[_*]").unwrap().replace(text.as_str(), "<em>").to_string();
            self.inline_parser(Regex::new(r"[_*]").unwrap().replace(f.as_str(), "</em>").to_string())
        }else if Regex::new(r"([_*]{2})[[[:punct:]]\w\W]+([_*]{2})").unwrap().is_match(text.as_str()) {
            let f = Regex::new(r"[_*]{2}").unwrap().replace(text.as_str(), "<strong>").to_string();
            self.inline_parser(Regex::new(r"[_*]{2}").unwrap().replace(f.as_str(), "</strong>").to_string())
        }else{
            text
        }
    }
}