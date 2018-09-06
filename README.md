mossy
================
A toy library for parsing and compiling Markdown.   
[![Build Status](https://travis-ci.com/totechite/mossy.svg?branch=master)](https://travis-ci.com/totechite/mossy)

## Usage
Add this to your `Cargo.toml`:   
```toml
[dependencies]
mossy = "0.1.1"
```
and this to your crate root:   
```rust
extern crate mossy;
```

### Example
```rust
extern crate mossy;
use mossy::mossy::Mossy;

let md_text = r"
mossy
================
A toy library for parsing Markdown.

# Specification
It's based [CommonMark].   

LICENSE
----------------
MIT

[CommonMark]: https://spec.commonmark.org/0.28/
";

let html: String = Mossy::new(String::from(md_text));

```

## Specification
It's based [CommonMark](https://spec.commonmark.org/0.28/).   

## Supported Markdown syntaxes
- Heading
- List
- Code block
- Link label

and some inline-elements.

LICENSE
----------------
MIT