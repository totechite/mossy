use lexer::Lexer;
use parser::Parser;

/// Mini markdown parser.   
/// Based specification is [CommonMark](https://spec.commonmark.org/0.28/).

#[derive(Debug)]
pub struct Mossy {}

///
/// ## Examples
///
/// ```
/// extern crate mossy;
/// use mossy::Mossy;
///
/// let md_text: String = r"
/// mossy
/// ================
/// Mini markdown parser
///
/// ## Specification
/// It's based [CommonMark].   
///
/// ## Supported Markdown syntaxes
/// - Heading
/// - List
/// - Code block
/// - Link label
///
/// and some inline-elements.
///
/// LICENSE
/// ----------------
/// MIT
///
/// [CommonMark]: https://spec.commonmark.org/0.28/
/// ";
///
/// let html: String = Mossy::new(md_text);
/// 
/// ```
///

impl Mossy {
    pub fn new(markdown: String) -> String {
	    let mut lexer: Lexer = Lexer::new(markdown);//gen tokens.
		Parser::new(lexer.exec()).exec()	//parse markdown_tokens to HTML.
    }
}




#[test]
fn sharp_heading() {
    assert_eq!(Mossy::new(String::from("# heading")), String::from("<h1>heading</h1>"))
}

#[test]
fn line_heading() {
	let md = 
r#"
heading
========
"#.to_string();
	let html =
	 "<h1>heading</h1>";
    assert_eq!(Mossy::new(md), html)
}

#[test]
fn code() {
	let md =
r#"
``` js
console.log("test");
```
"#.to_string();
	let html = 
r#"<pre><code class="language-js">
console.log("test");
</code></pre>"#;
    assert_eq!(Mossy::new(md), html)
}

#[test]
fn inline_link() {
	let md =
r#"[Rust](https://www.rust-lang.org)is a system programming language."#.to_string();
	let html = 
r#"<p><a href="https://www.rust-lang.org">Rust</a>is a system programming language.</p>"#;
    assert_eq!(Mossy::new(md), html)
}