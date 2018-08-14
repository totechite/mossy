#[derive(Debug, PartialEq)]
pub enum Token{
    N_A,
    EOF,
    Heading{depth: i8, text: String},
    Paragraph{text: String},
    Code{lang: String, text: String},
    Text{text: String},
    ListStart{ordered: bool},
    ListItemStart,
    ListItemEnd,
    ListEnd
}
