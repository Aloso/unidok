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

### MVP

* Bug: Excess spaces/tabs in empty line can interrupt blockquote

* Code blocks
  * Syntax highlighting

* HTML
  * HTML entities

* Math
  * Need a way to automatically include the MathJax script (perhaps with a macro)

* Replacements
  * Smart punctuation: `"`, `'`, `--`, `...`
  * Arrows: `->`, `=>`, `<-`, `<=`, `<->`, `<=>`
  * Symbols: `(C)`, `(TM)`, `(R)`

* Macros
  * Table of contents (`@TOC`)
  * Table column styling (`@COLS`)

### Lower Priority

* Lists
  * Checkbox list items (`- [x] done`)

* Tables
  * Allow table cells to contain block elements

* HTML
  * Allow uppercase HTML tags
  * Warn when an element is in an element where it is illegal as of HTML5
  * Warn when a block HTML element isn't followed by a line break

* Links
  * Auto-links
  * Forbid nested links
  * URLs in angle brackets

* Replacement plugins
  * Math:
    `!=`, `<=`, `>=`, `~=`, `===`, `~~`, `+-`, `-+`, `|->`, `_|_`, `|-`,
    `\alpha`, `\Alpha`, `\beta`, ..., `\pi`, ..., `\Omega`,
    `\in`, `\notin`, `\AA`, `\EE`, `\N`, `\Z`, `\Q`, `\R`, `\C`, `\H`, `\F`, `\oo`, `\aleph` ...
    * <https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols>
    * <http://asciimath.org/>
  * Other, e.g.: `?!`, `!?`

* Footnotes

* Macros
  * Include file (`@INCLUDE`)
  * Rewrite URLs (`@REWRITE_URLS`)
    * Important when using webpack, jekyll or a similar system where assets move while building the site
    * Convenient if URLs can be abbreviated, e.g. `wiki:en/*` -> `https://en.wikipedia.org/wiki/*`
  * Don't wrap text in a paragraph (`@PURE`)
  * Open link in new tabl (`@BLANK`)
  * Load plugin (`@LOAD`)
  * Exclude heading from table of contents (`@NO_TOC`)
  * Details that can be opened with summary (`@DETAILS`)
  * Image caption (`@CAPTION`)
  * Metadata (`@META`)
  * `@()` should have a shortcut for `style=...`

### Version 1.0

* Security protocol
  * There should be two operating modes, "safe mode" and "unsafe mode". In safe mode, when untrusted input is converted to HTML and displayed, the following should apply:
    * The environment converting the input to HTML (from now on called the "build environment") can't be altered from within Unidoc; for example, no files in the build environment can't be modified.
    * Information from the build environment can't be obtained from within Unidoc, unless explicitly permitted; for example, file access is restricted to an allowlisted set of directories
    * The build environment can't be attacked or compromised in any way, including hacking, crypto-mining, sending spam mail, running a Denial-of-Service attack (e.g. by providing input with exponential parsing complexity), communicating with 3rd-party services using the build environment's credentials, performing illegal activity using the build environment, circumventing paywalls or other restrictions (e.g. to access services only available in certain countries), etc.
  * The safe mode should be the default. Unsafe mode can be enabled in the API, the command line, and nowhere else.
  * Further restrictions can be put in place, e.g. to forbid embedding iframes or untrusted JavaScripts in the document.

* Plugins
  * Plugins could add new macros, text substitutions, URL schemas, HTML tags, change the configuration, read and modify the IR, add document metadata, provide syntax highlighting, provide a file format parser (e.g. to include `*.md` or `*.adoc` files), start a development server, ...
  * Unidoc could communicate with plugins over stdin/out via JSON
  * Plugins probably won't work in the playground, so implement as much as possible in Unidoc directly; this simplifies distribution, as a single executable is sufficient for most purposes.

* IDE support

### Vague Ideas for the Distant Future

* Stolen from Asciidoctor:
  * Admonition blocks (e.g. `@TIP`)
  * Sidebar blocks (e.g. `@SIDEBAR`)
  * Example blocks (e.g. `@EXAMPLE(title)`)
  * Labeled lists (e.g. `Label:: content`), Q&A lists, glossary lists, bibliography lists
* Emojis (e.g. `:makeup:`)
* Custom inline formatting delimiters (e.g. `=Keyboard-shortcut=`)
  * The following symbols are potentially available: `+`, `?`, `´`, `=`, `:`, `;`

### Design Discussion

* The `#` character is useful, e.g. for GitHub issues and pull requests, Twitter hashtags, etc. We could use `~` for subscript and require two tildes for strikethrough (as in GFM).

* Backslash and dollar sign (limiter) have overlapping purposes. Furthermore, the dollar sign must _always_ be escaped, which is not only annoying, but can cause problems, since an unescaped dollar sign that should be escaped is easy to miss.

* In Markdown, numbered lists can only interrupt a paragraph if they start with the number 1. On one hand, this is an inconsistency and an edge case that probably very few people know about. On the other hand, it is useful (99% of the time, it "just works", without you having to know how). Also, when it doesn't work, the solution is simple and intuitive: Add another line break. In the other direction, the solution is less intuitive: Either remove the line break before the number, or escape the dot or parenthesis of the number list marker:

  ```markdown
  Hope's favourite number is
  4. This is a list item in Unidoc, but not in Markdown.

  Marcus' favourite number is
  1. This is a list item in both Unidoc and Markdown.
  ```

  Disambiguation im Markdown:

  ```markdown
  Hope's favourite number is

  4. This is a list item.

  Marcus' favourite number is
  1\. Not a list item
  ```

* Blockquotes can have empty lines in between. This is different than in Markdown, where a blank line terminates a blockquote. This might be unexpected, and an inexperienced writer might not know what to do to fix (the fix is to insert a `$`).

* It's odd that macros can attach to `inline code`, but not to other types of inline formatting. This is due to how formatting is parsed. While parsing a paragraph, it is split into _items_; an item can be

  * a formatting delimiter
  * a link, image, HTML node, math element, escape sequence, limiter or inline code
  * a line break
  * a string of text that doesn't contain any of the above

  In the next step, Unidoc determines what delimiters open a formatting range, which close a formatting range, and which do neither (and are therefore printed in the output as-is). However, to attach macros to any formatting range, that formatting range must be parsable in one go, which is not possible, or would require profound changes to how inline formatting works in the presence of macros.

  Note that inline code works very different than other kinds of formatting. This is because, according to the CommonMark specification, inline code binds "more tightly" than emphasis and strong emphasis (`*italic*` and `**bold**` text). This means that `` **Hello `world**!` `` is rendered as \*\*Hello `world**!`, and not **Hello \`world**!\`. Generally speaking, inline code can be parsed in one go: When the first backtick sequence is encountered, it always opens an inline code element, which is closed by the next backtick sequence of equal length.

* It's unfortunate that a sequence of dashes can be either a thematic break or a heading underline. I would prefer if this ambiguity didn't exist, even though it is easy to resolve. However, deprecating thematic breaks made of dashes would break a lot of Markdown documents; one current advantage of Unidoc is that most Markdown documents need no or very few changes to become an equivalent Unidoc document.

* Unidoc tries to behave the same as CommonMark, unless there's a good reason to break compatibility. One interesting case are tables: Tables aren't part of the CommonMark specification, only the GFM (GitHub-flavored Markdown) specification. However, GFM-style tables are supported in many Markdown implementations. Therefore it would make sense if Unidoc tables were backwards-compatible with GFM-style tables.

  I decided against that because I find them inflexible and cumbersome to type. Furthermore, they are only readable when the content fits in a single line, and the line that separates the table header from the body seems out of place when there is no table header. Lastly, GFM-style tables are difficult to parse.

* Math support isn't strictly required, since MathJax has AsciiMath support. However, I believe that doing some of the work up front (converting AsciiMath to MathML) makes the website faster, although I haven't measured this. Also, it means that special characters in math formulas (`_ * ~ [] |`) aren't accidentally interpreted as Unidoc. Not having to escape them makes the formulas easier to read and to write.

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
