#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    N_A,
    EOF,
    Heading{depth: i8, text: String},
    Paragraph{text: String},
    Code{lang: String, text: String},
    Text{text: String},
    ListStart{ordered: bool},
    ListItem{text: String, task: bool, checked: bool},
    ListEnd
}
