use token::Token;
use regex::{Regex, Captures};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parser {
    point: usize,
    links: HashMap<String, String>,
    tokens: Vec<Token>,
    token: Option<Token>
}

impl Parser {

    pub fn new(tokens: Vec<Token>)-> Parser{
        let tokens = tokens;
        let token = tokens.clone().iter().nth(0).unwrap().to_owned();
        Parser{
            point: 0usize,
            links: HashMap::new(),
            tokens: tokens,
            token: Some(token)
        }
    }

    fn consume(&mut self) {
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
        self.links = self.clone().collecte_link();
        let mut output = String::new();
        while let Some(token) = self.token.clone() {
            match token{
                Token::Heading{ depth, text } => {
                    let text = self.clone().inline_parser(text.to_owned());
                    let s = format!("<h{}>{}</h{}>\n", depth, text.trim(), depth);
                    output += s.as_str();
                    self.consume();
                },
                Token::Paragraph{ text } => {
                    let text = self.clone().inline_parser(text.to_owned()).trim().to_string();
                    let s = format!("<p>{}</p>\n", text);
                    output += s.as_str();
                    self.consume();
                },
                Token::Code{lang, text} => {
                    let mut s = if lang.to_owned()==String::from(""){ String::from("") }else{ format!(" class=\"language-{}\"", lang)  };
                    s = format!("<pre><code{}>\n{}</code></pre>\n", s, text);
                    output += s.as_str();
                    self.consume();
                },
                Token::ListStart{ordered} => {
                    let list_parent: String = if ordered.to_owned() { String::from("ol") }else { String::from("ul")};
                    self.consume();
                    output+=format!("<{}>\n", &list_parent).as_str();
                    let li_tags: String = String::new();
                    let mut is_inList: bool = true;
                    while is_inList{
                        match self.token.clone().unwrap() {
                            Token::ListItem{text, task, checked} => {
                                output+=format!("    <li>{}</li>\n", self.clone().inline_parser(text)).as_str();
                                self.consume();
                            },
                            Token::ListEnd => {
                                output+=format!("</{}>\n", list_parent).as_str();
                                is_inList = false;
                                self.consume();
                            },
                            _ => is_inList = false,
                        }
                    }  
                },
                Token::BlockquoteStart => {
                    self.consume();
                    match self.token.clone().unwrap(){
                        Token::Paragraph{text} => {
                            output+=format!("<blockquote>\n<p>{}</p>\n</blockquote>\n", text).as_str();
                        },
                        _ => {}
                    };
                    self.consume();
                    self.consume();
                },
                _ => {
                    self.consume();
                }
            }
        }
        output.trim().to_string()
    }

    fn collecte_link(self) -> HashMap<String, String>{
        let mut links: HashMap<String, String> = HashMap::new();
        self.tokens.iter().map(|x|{
            match x{
                Token::Paragraph{text} => {
                    if Regex::new(r"^\[(\w+)\]:\s+{0,}([\w\W]+)\s+{0,}[\w\s]+").unwrap().is_match(text.as_str()){
                        let mut linkname: String = String::new();
                        let text:String = Regex::new(r"^\[(\w+)\]:\s+{0,}([\w\W]+)\s+{0,}[\w\s]+").unwrap().replace(text.as_str(), |caps: &Captures| {
                            linkname = caps.get(1).map_or("", |m| m.as_str()).to_string();
                            match &caps.len(){
                                2 => {
                                    return format!("<a href={}>{}</a>", &caps[2].to_string(), &caps[1].to_string());
                                },
                                3 =>{
                                    return format!("<a href={} title={}>{}</a>", &caps[2].to_string(), &caps[3].to_string(), &caps[1].to_string());
                                },
                                _ => {
                                    return format!("<a href={}>{}</a>", &caps[2].to_string(), &caps[1].to_string());
                                }
                            }
                        }).to_string();
                        links.insert(linkname, text);
                    };
                },
                _ => {}
            };
        });
        links

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