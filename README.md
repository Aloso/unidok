# ![Logo](doc/ud.svg)&nbsp; Unidok

![Crates.io][Crates] ![npm][Npm] [![Test][Test-image]][Test-url] ![License][License-image]

[Crates]: https://img.shields.io/crates/v/unidok
[Npm]: https://img.shields.io/npm/v/unidok
[Test-image]: https://github.com/Aloso/unidok/actions/workflows/test.yml/badge.svg
[Test-url]: https://github.com/Aloso/unidok/actions/workflows/test.yml
[License-image]: https://img.shields.io/badge/license-Apache%202%2FMIT-blue

A powerful, readable, easy-to-learn markup language

Unidok is a new Markup language, inspired by AsciiDoctor and Markdown. It is very easy to read and to learn while offering powerful features such as macros.

<div align="center">

|        |
| ------ |
| [Check out the website!](https://aloso.github.io/unidok/) |

</div>

## Installation

[Here](https://github.com/Aloso/unidok/releases) you can find pre-built binaries for Linux, Windows and macOS.

If you want to build it yourself, install the Rust toolchain with Rust Beta and run `cargo install`.

## Principles

Unidok should look familiar if you're familiar with Markdown. It follows the CommonMark specification closely where it makes sense, but it also omits some Markdown features that I feel are unhelpful, and adds new features. Notable examples are:

* Supports `^superscript^`, `#subscript#`, `~line-through~` text, tables, math formulas, and macros

* Does not support indented code blocks, only fenced code blocks

* Sane HTML parsing

* It does not support _laziness_, i.e. in a list item or blockquote,
  all lines must be indented or preceded with `>` respectively, not just the first line


## Roadmap

[See here](https://aloso.github.io/unidok/?roadmap).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE_MIT](LICENSE_MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
