# ![Logo](doc/ud.svg)&nbsp; Unidok

A powerful, readable, easy-to-learn markup language

This language is inspired by AsciiDoc, but has a syntax that resembles Markdown. It tries to be as simple as possible while offering powerful features such as macros.

<div align="center">
  <hr>
  <a href="https://aloso.github.io/unidok/">Check out the website!</a>
  <hr>
</div>

## Installation

Installation currently requires that you have the Rust toolchain installed (Rust Beta/1.53 or higher, including Cargo).

Clone the repository and run `cargo install`.

## Usage

Unidok doesn't yet have a proper command-line interface. You can pass a string to it, and it will print the corresponding HTML to stdout. That's it (for now).

```shell
> unidok '# Hello world'
<h1>Hello world</h1>
```

## Principles

Unidok should look familiar if you're familiar with Markdown. It follows the CommonMark specification closely where it makes sense, but it also omits some Markdown features that I feel are unhelpful, and adds new features. Notable examples are:

* It supports `^superscript^`, `#subscript#`, `~line-through~` text, tables, math formulas, and macros

* It allows mixing HTML with other formatting types (e.g. `<kbd>**bold**</kbd>`)

* It does not support indented code blocks, only fenced code blocks

* It does not support _laziness_, i.e. in a list item or blockquote,
  all lines must be indented or preceded with `>` respectively, not just the first line


## Roadmap

[See here](https://aloso.github.io/unidok/?design#upcoming-features).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
