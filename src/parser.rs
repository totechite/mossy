use crate::token::Token;
use regex::{Captures, Regex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parser {
    point: usize,
    links: HashMap<String, String>,
    tokens: Vec<Token>,
    token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let tokens = tokens;
        let token = tokens.clone().iter().nth(0).unwrap().to_owned();
        Parser {
            point: 0usize,
            links: HashMap::new(),
            tokens: tokens,
            token: Some(token),
        }
    }

    fn consume(&mut self) {
        self.point += 1usize;
        match self.tokens.iter().nth(self.point) {
            Some(token) => {
                self.token = Some(token.to_owned());
            }
            None => {
                self.token = None;
            }
        }
    }

    pub fn exec(&mut self) -> String {
        self.links = self.collecte_link();
        let mut output = String::new();
        while let Some(token) = self.token.clone() {
            match token {
                Token::Heading { depth, text } => {
                    let text = self.clone().inline_parser(text.to_owned());
                    let s = format!("<h{}>{}</h{}>\n", depth, text.trim(), depth);
                    output += s.as_str();
                    self.consume();
                }
                Token::Paragraph { text } => {
                    let text = self
                        .clone()
                        .inline_parser(text.to_owned())
                        .trim()
                        .to_string();
                    let s = format!("<p>{}</p>\n", text);
                    output += s.as_str();
                    self.consume();
                }
                Token::Code { lang, text } => {
                    let mut s = if lang.to_owned() == String::from("") {
                        String::from("")
                    } else {
                        format!(" class=\"language-{}\"", lang)
                    };
                    s = format!("<pre><code{}>\n{}</code></pre>\n", s, text);
                    output += s.as_str();
                    self.consume();
                }
                Token::ListStart { ordered } => {
                    let list_parent: String = if ordered.to_owned() {
                        String::from("ol")
                    } else {
                        String::from("ul")
                    };
                    self.consume();
                    output += format!("<{}>\n", &list_parent).as_str();
                    let mut is_in_list: bool = true;
                    while is_in_list {
                        match self.token.clone().unwrap() {
                            Token::ListItem {
                                text,
                                task: _,
                                checked: _,
                            } => {
                                output +=
                                    format!("<li>{}</li>\n", self.clone().inline_parser(text))
                                        .as_str();
                                self.consume();
                            }
                            Token::ListEnd => {
                                output += format!("</{}>\n", list_parent).as_str();
                                is_in_list = false;
                                self.consume();
                            }
                            _ => is_in_list = false,
                        }
                    }
                }
                Token::BlockquoteStart => {
                    self.consume();
                    match self.token.clone().unwrap() {
                        Token::Paragraph { text } => {
                            output +=
                                format!("<blockquote>\n<p>{}</p>\n</blockquote>\n", text).as_str();
                        }
                        _ => {}
                    };
                    self.consume();
                    self.consume();
                }
                _ => {
                    self.consume();
                }
            }
        }
        output.trim().to_string()
    }

    fn collecte_link(&mut self) -> HashMap<String, String> {
        let mut links: HashMap<String, String> = HashMap::new();
        let mut link_len = vec![];
        for (nums, token) in self.tokens.iter().enumerate() {
            match token {
                Token::Paragraph { text } => {
                    if Regex::new(r"^\[(\w+)\]:\s*([\w\W[[:punct:]]]+)\s*\u0022([\w\W\s[[:punct:]]]+)\u0022\n?$").unwrap().is_match(text.as_str()){
                        link_len.push(nums as usize);
                        let mut linkname: String = String::new();
                        let text:String = Regex::new(r"^\[(\w+)\]:\s*([\w\W[[:punct:]]]+)\s+\u0022([\w\W\s[[:punct:]]]+)\u0022\n?$").unwrap().replace(text.as_str(), |caps: &Captures| {
                            linkname = caps.get(1).map_or("", |m| m.as_str()).to_string().to_lowercase();
                            return format!("<a href=\"{}\" title=\"{}\">{}</a>", &caps[2].trim().to_string(), &caps[3].to_string(), &caps[1].to_string());
                        }).to_string();
                        links.insert(linkname, text);
                    }else if Regex::new(r"^\[(\w+)\]:\s*([\w\W[[:punct:]]]+)\s*\n?$").unwrap().is_match(text.as_str()){
                        link_len.push(nums as usize);
                        let mut linkname: String = String::new();
                        let text:String = Regex::new(r"^\[(\w+)\]:\s*([\w\W[[:punct:]]]+)\s*\n?$").unwrap().replace(text.as_str(), |caps: &Captures| {
                            linkname = caps.get(1).map_or("", |m| m.as_str()).to_string().to_lowercase();
                            return format!("<a href=\"{}\">{}</a>", &caps[2].trim().to_string(), &caps[1].to_string());
                        }).to_string();
                        links.insert(linkname, text);
                    }
                }
                _ => {}
            };
        }
        link_len.reverse();
        for num in link_len.iter() {
            self.tokens.remove(*num);
        }
        links
    }

    fn inline_parser(self, text: String) -> String {
        // Code
        if Regex::new(r"(`+)\B").unwrap().is_match(text.as_str()) {
            if Regex::new(r"`<\w+>`").unwrap().is_match(text.as_str()) {
                let e = Regex::new(r"`<\w+>`")
                    .unwrap()
                    .captures(text.as_str())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();
                let pat = Regex::new(r"\w+")
                    .unwrap()
                    .captures(e)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();
                let replace_string: String = format!("&lt;{}&gt;", pat);
                self.inline_parser(
                    Regex::new(r"<\w+>")
                        .unwrap()
                        .replace(text.as_str(), replace_string.as_str())
                        .to_string(),
                )
            } else {
                let f = Regex::new(r"\B(`+)")
                    .unwrap()
                    .replace(text.as_str(), "<code>")
                    .to_string();
                self.inline_parser(
                    Regex::new(r"(`+)\B")
                        .unwrap()
                        .replace(f.as_str(), "</code>")
                        .to_string(),
                )
            }
        }
        // Italic
        else if Regex::new(r"[_*][[:word:]]+[_*]")
            .unwrap()
            .is_match(text.as_str())
        {
            let f = Regex::new(r"[_*]")
                .unwrap()
                .replace(text.as_str(), "<em>")
                .to_string();
            self.inline_parser(
                Regex::new(r"[_*]")
                    .unwrap()
                    .replace(f.as_str(), "</em>")
                    .to_string(),
            )
        }
        // Bold
        else if Regex::new(r"([_*]{2})[[[:punct:]]\w\W]+([_*]{2})")
            .unwrap()
            .is_match(text.as_str())
        {
            let f = Regex::new(r"[_*]{2}")
                .unwrap()
                .replace(text.as_str(), "<strong>")
                .to_string();
            self.inline_parser(
                Regex::new(r"[_*]{2}")
                    .unwrap()
                    .replace(f.as_str(), "</strong>")
                    .to_string(),
            )
        }
        // Image
        else if Regex::new(r"!\[(\w+)\]\(\s*([\w\W[[:punct:]]]+)[\s\w\W[[:punct:]]]*\)")
            .unwrap()
            .is_match(text.as_str())
        {
            let f: String;
            if Regex::new(
                r"!\[(\w+)\]\(\s*([\w\W[[:punct:]]]+)\s+(\u0022[\w\W\s[[:punct:]]]+\u0022)?\s*\)",
            )
            .unwrap()
            .is_match(text.as_str())
            {
                f = Regex::new(r"!\[(\w+)\]\(\s*([\w\W[[:punct:]]]+)\s+(\u0022[\w\W\s[[:punct:]]]+\u0022)\s*\)").unwrap().replace(text.as_str(), |caps: &Captures|{
                        return format!("<img src=\"{}\" alt=\"{}\" title={}>", &caps[2].trim().to_string(), &caps[1].to_string(), &caps[3].to_string());
                }).to_string();
            } else {
                f = Regex::new(r"!\[(\w+)\]\(\s*([\w\W[[:punct:]]]+)\s*\)")
                    .unwrap()
                    .replace(text.as_str(), |caps: &Captures| {
                        return format!(
                            "<img src=\"{}\" alt=\"{}\">",
                            &caps[2].trim().to_string(),
                            &caps[1].to_string()
                        );
                    })
                    .to_string();
            }
            self.inline_parser(f.to_string())
        }
        // Link label :defined at Inline
        else if Regex::new(r"\[(\w+)\]\(([\w[[:punct:]]]+)\)")
            .unwrap()
            .is_match(text.as_str())
        {
            let f = Regex::new(r"\[(\w+)\]\(([\w[[:punct:]]]+)\)")
                .unwrap()
                .replace(text.as_str(), |caps: &Captures| {
                    return format!(
                        "<a href=\"{}\">{}</a>",
                        &caps[2].to_string(),
                        &caps[1].to_string()
                    );
                });
            self.inline_parser(f.to_string())
        }
        // Link label :refarence
        else if Regex::new(r"\[(\w+)\]").unwrap().is_match(text.as_str()) {
            let f = Regex::new(r"\[(\w+)\]")
                .unwrap()
                .replace(text.as_str(), |caps: &Captures| {
                    return format!(
                        "{}",
                        self.links.get(&caps[1].to_string().to_lowercase()).unwrap()
                    );
                });
            self.inline_parser(f.to_string())
        } else {
            text
        }
    }
}
