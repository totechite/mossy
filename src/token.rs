#[derive(Debug, PartialEq)]
pub enum Token{
    N_A,
    EOF,
    HEADING{depth: i8, text: String},
    PARAGRAPH{text: String},
    CODE{lang: String, text: String},
    TEXT{text: String},
    LIST_START{ordered: bool},
    LIST_ITEM_START,
    LIST_ITEM_END,
    LIST_END
}
