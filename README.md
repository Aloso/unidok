# ![Logo](doc/ud.svg)&nbsp; Unidoc

A powerful, readable, easy-to-learn markup language

This language is inspired by AsciiDoc, but has a syntax that resembles Markdown. It tries to be as simple as possible while offering powerful features such as macros.

<div align="center">
  <hr>
  <a href="https://aloso.github.io/unidoc/">Check out the website!</a>
  <hr>
</div>

## Installation

Installation currently requires that you have the Rust toolchain installed (Rust Beta/1.53 or higher, including Cargo).

Clone the repository and run `cargo install`.

## Usage

Unidoc doesn't yet have a proper command-line interface. You can pass a string to it, and it will print the corresponding HTML to stdout. That's it (for now).

```shell
> unidoc '# Hello world'
<h1>Hello world</h1>
```

## Principles

Unidoc should look familiar if you're familiar with Markdown. It follows the CommonMark specification closely where it makes sense, but it also omits some Markdown features that I feel are unhelpful, and adds new features. Notable examples are:

* It supports `^superscript^`, `#subscript#`, `~line-through~` text, tables, math formulas, and macros

* It allows mixing HTML with other formatting types (e.g. `<kbd>**bold**</kbd>`)

* It does not support indented code blocks, only fenced code blocks

* It does not support _laziness_, i.e. in a list item or blockquote,
  all lines must be indented or preceded with `>` respectively, not just the first line

## Roadmap

* Code blocks
  * Syntax highlighting

* HTML
  * HTML entities
  * Allow uppercase HTML tags
  * Warn when an element is in an element where it is illegal as of HTML5
  * Warn when a block HTML element isn't followed by a line break

* Math
  * Need a way to automatically include the MathJax script (perhaps with a macro)

* Links
  * Link reference definitions for images
  * Auto-links?
  * Forbid nested links?
  * URLs in angle brackets?

* Replacements
  * Enabled by default:
    * Quotes, apostrophe: `"`, `'` ("smart quotes")
    * Arrows: `->`, `=>`, `<-`, `<=`, `<->`, `<=>`
    * Em dash, ellipsis: `--`, `...`
    * Symbols: `(C)`, `(TM)`, `(R)`
  * Math:
    `!=`, `<=`, `>=`, `~=`, `===`, `~~`, `+-`, `-+`, `|->`, `_|_`, `|-`,
    `\alpha`, `\Alpha`, `\beta`, ..., `\pi`, ..., `\Omega`,
    `\in`, `\notin`, `\AA`, `\EE`, `\N`, `\Z`, `\Q`, `\R`, `\C`, `\H`, `\F`, `\oo`, `\aleph` ...
    * <https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols>
  * Other, e.g.: `?!`, `!?`

* Macros
  * Table of contents (`@TOC`)
  * Include file (`@INCLUDE`)
  * Table column styling (`@COLS`)
  * Rewrite URLs (`@REWRITE_URLS`)
    * Important when using webpack, jekyll or a similar system where assets move while building the site
    * Convenient if URLs can be abbreviated, e.g. `wiki:en/*` -> `https://en.wikipedia.org/wiki/*`
  * Footnotes, e.g.
    * `@FN{^1} ... @FN(1){This is the footnote text}`
    * `@FN{This is the footnote text} ... @FOOTNOTES{}`
  * Define custom macros in a plugin

* Plugins
  * Maintain some plugins in-tree in the same workspace, which can share dependencies and are safe to use
  * To check if a plugin is safe to use, it
  * Asciidoc communicates with plugins over stdin/out via JSON.

    <details><summary>Example</summary>

    ```json
    {
      "status": "connect",
      "api_version": "1.4",
      "auth_challenge": "f73d287a",
    }
    {
      "status": "connect",
      "plugin_name": "hello-world",
      "plugin_version": "1.1",
      "safe": true,
      "auth_token": "7245a74b57e57c5",
    }
    {
      "status": "ok"
    }
    {
      "status": "ok",
      "actions": [
        {
          "type": "register substitution",
          "name": "coypright",
          "find": "(C)",
          "replace": "&copy;",
        },
        {
          "type": "register substitution",
          "name": "ellipsis",
          "find": "...",
          "replace": "&hellip;",
          "validate": true
        },
        {
          "type": "register macro",
          "name": "FOO"
        }
      ]
    }
    {
      "status": "close"
    }
    ```

    </details>

* Vague ideas:
  * Stolen from Asciidoctor: Admonition blocks (e.g. `@TIP`), sidebar blocks (e.g. `@SIDEBAR`), example blocks (e.g. `@EXAMPLE(title)`), labeled lists (e.g. `Label:: content`), Q&A lists, glossary lists, bibliography lists
  * Emojis (e.g. `:makeup:`)
  * Custom inline formatting delimiters (e.g. `=Keyboard-shortcut=`)
    * The following symbols are potentially available: `+`, `?`, `´`, `=`, `:`, `;`
  * Globally disable some parsers
    * HTML: Either accept only allowlisted tags, or don't accept forbidden tags
      * Enabled by default:
        - [x] Base document structure
        - [x] Content sectioning
        - [x] Text content
        - [x] Inline text
        - [x] Table content
        - [x] Images (`<img>`, `<svg>`)
        - [x] Other (`<canvas>`, `<command>`, `<del>`, `<ins>`, `<noscript>`, `<template>`)
        - [x] Math
        - [ ] Audio/video
        - [ ] Forms
        - [ ] Embedded content
        - [ ] Stylesheets (`<style>`, `<link rel="stylesheet">`)
        - [ ] Scripts
        - [ ] Deprecated elements
        - [ ] Custom
    * Insecure macros
    * Replacements
    * Math formulas
    * Limiter
    * Checkboxes
    * Autolinks
    * ATX headings
    * Setext headings

    Less important:

    - Lists
    - Blockquotes
    - Thematic breaks
    - Fenced code blocks
    - Link reference definitions
    - Code spans
    - Emphasis, strong emphasis
    - Links, images

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
