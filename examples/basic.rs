extern crate mossy;

use mossy::App;

fn main() {
    let markdown_text = r"
mossy
================
A toy library for parsing Markdown.

## Specification
It's based [CommonMark].

LICENSE
----------------
MIT

[CommonMark]: https://spec.commonmark.org/0.28/
"
    .to_string();

    println!("{}", App::exec(markdown_text));
}
