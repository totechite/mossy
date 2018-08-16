use token::Token;

#[derive(Debug)]
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
        for token in self.tokens.unwrap() {
            match token{
                Token::Heading{depth, text} => {
                    let s = format!("<h{}>{}</h{}>", depth, text, depth);
                    self.parsed += &s;
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
}