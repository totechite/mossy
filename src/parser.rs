use lexer::Lexer;
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

    pub fn exec(self) -> String{
        let mut tokens = self.tokens.unwrap();
        for token in tokens {
            match token{
                Token::HEADING{depth, text} => {},
                Token::PARAGRAPH{text} => {},
                _ => {}
            }
        }
        String::from("complete fn exec().")
    }
}