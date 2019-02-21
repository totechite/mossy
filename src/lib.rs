//! mossy
//! =====================
//! A toy library for parsing and compiling Markdown.
//!
//! Based specification is [CommonMark](https://spec.commonmark.org/0.28/).
//!
//! Usage
//! ---------------------
//! ```rust
//! extern crate mossy;
//! use mossy::App;
//!
//! let md_text = r"
//! mossy
//! ================
//! A toy library for parsing Markdown.
//!
//! # Specification
//! It's based [CommonMark].
//!
//! LICENSE
//! ----------------
//! MIT
//!
//! [CommonMark]: https://spec.commonmark.org/0.28/
//! ".to_string();
//!
//! let html: String = App::exec(md_text);
//!
//! ```


extern crate regex;

mod token;

mod lexer;

mod parser;

pub use crate::lexer::Lexer;
pub use crate::parser::Parser;

#[derive(Debug)]
pub struct App {}

impl App {

/// ```
/// # extern crate mossy;
/// # use mossy::App;
/// let md_text = "# I'm <h1> tag!";
/// let html: String = App::exec(String::from(md_text));
/// ```
    pub fn exec(markdown: String) -> String {
        let mut lexer: Lexer = Lexer::new(markdown); //gen tokens.
        Parser::new(lexer.exec()).exec() //parse markdown_tokens to HTML.
    }
}

#[test]
fn sharp_heading() {
    assert_eq!(
        App::exec(String::from("# heading")),
        String::from("<h1>heading</h1>")
    )
}

#[test]
fn line_heading() {
    let md = r#"
heading
========
"#
    .to_string();
    let html = "<h1>heading</h1>";
    assert_eq!(App::exec(md), html)
}

#[test]
fn code() {
    let md = r#"
``` js
console.log("test");
```
"#
    .to_string();
    let html = r#"<pre><code class="language-js">
console.log("test");
</code></pre>"#;
    assert_eq!(App::exec(md), html)
}

#[test]
fn inline_link() {
    let md = r#"[Rust](https://www.rust-lang.org)is a system programming language."#.to_string();
    let html =
        r#"<p><a href="https://www.rust-lang.org">Rust</a>is a system programming language.</p>"#;
    assert_eq!(App::exec(md), html)
}
