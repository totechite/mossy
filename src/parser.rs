use token::Token;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Parser {
    point: usize,
    tokens: Option<Vec<Token>>,
    parsed: String
}

impl Parser {

    pub fn new(tokens: Vec<Token>)-> Parser{
        Parser{
            point: 0usize,
            tokens: Some(tokens),
            parsed: String::new()
        }
    }

    pub fn exec(mut self) -> String{
        for token in self.clone().tokens.unwrap() {
            match token{
                Token::Heading{depth, text} => {
                    let text = self.clone().inline_parser(text);
                    let s = format!("<h{}>{}</h{}>", depth, text, depth);
                    self.parsed += s.as_str();
                },
                Token::Paragraph{text} => {
                    let s = format!("<p>{}</p>", text);
                    self.parsed += &s;
                },
                Token::Code{lang, text} => {
                    let s = if &lang==&String::from(""){ "".to_string() }else{ format!(" class=\"{}\"", lang)  };
                    let s = format!("<pre><code{}>{}</code></pre>", s, text);
                    self.parsed += &s;
                },
                _ => {}
            }
        }
        self.parsed
    }

    fn inline_parser(self, text: String) -> String{
        if Regex::new(r"(`+)\b").unwrap().is_match(text.as_str()){
            let f = Regex::new(r"\b?(`+)").unwrap().replace(text.as_str(), "<code>").to_string();
            self.inline_parser(Regex::new(r"(`+)\b").unwrap().replace(f.as_str(), "</code>").to_string())
        }else if Regex::new(r"[_*][[:alpha:]]+[_*]").unwrap().is_match(text.as_str()) {
            let f = Regex::new(r"[_*]").unwrap().replace(text.as_str(), "<em>").to_string();
            self.inline_parser(Regex::new(r"[_*]").unwrap().replace(f.as_str(), "</em>").to_string())
        }else if Regex::new(r"[_*]{2}\w+[_*]{2}").unwrap().is_match(text.as_str()) {
            let f = Regex::new(r"[(_*]{2}").unwrap().replace(text.as_str(), "<strong>").to_string();
            self.inline_parser(Regex::new(r"[(_*]{2}").unwrap().replace(f.as_str(), "</strong>").to_string())
        }else {
            text
        }
    }
}