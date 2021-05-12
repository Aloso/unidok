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

* Code blocks
  * Syntax highlighting

* Math
  * Need a way to automatically include the MathJax script (perhaps with a macro)

* Replacements
  * Smart punctuation: `"`, `'`, `--`, `...`
  * Arrows: `->`, `=>`, `<-`, `<=`, `<->`, `<=>`
  * Symbols: `(C)`, `(TM)`, `(R)`

* Macros
  * Table of contents (`@TOC`)
  * Table column styling (`@COLS`)
  * Footnotes section (`@FOOTNOTES`)

* Change behaviour of `$`: Only parse limiter directly next to a delimiter run, after a link reference or in an otherwise empty line

### Lower Priority

* Lists
  * Checkbox list items (`- [x] done`)

* Tables
  * Allow table cells to contain block elements

* HTML
  * Allow uppercase HTML tags
  * Allow numeric HTML entities
  * Warn when an element is in an element where it is illegal as of HTML5
  * Warn when a block HTML element isn't followed by a line break

* Links
  * Auto-links
  * Forbid nested links
  * URLs in angle brackets

* Replacement plugins
  * Math:
    `!=`, `<=`, `>=`, `~=`, `===`, `~~`, `+-`, `-+`, `|->`, `_|_`, `|-`,
    `\AA`, `\EE`, `\N`, `\Z`, `\Q`, `\R`, `\C`, `\H`, `\F`, `\oo` ...
    * <https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols>
    * <http://asciimath.org/>
    * Note that greek letters and many operators can be entered with HTML entities, e.g. `&pi;` = &pi;, `&notin;` = &notin;
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

### Random Ideas

* `@()` could have a shortcut for `style=...`

* Metadata section at the top, like in Liquid:
  ````
  ---
  author: John Doe
  date: 2025-01-01
  ---
  ````

* Stolen from Asciidoctor:
  * Admonition blocks (e.g. `@TIP`)
  * Sidebar blocks (e.g. `@SIDEBAR`)
  * Example blocks (e.g. `@EXAMPLE(title)`)
  * Labeled lists (e.g. `Label:: content`), Q&A lists, glossary lists, bibliography lists

* Emojis (e.g. `:makeup:`)

* Custom inline formatting delimiters (e.g. `=Foo= ´bar´`)
  * The following symbols are potentially available: `+`, `?`, `´`, `=`, `:`, `;`

* Support XML (e.g. for `<svg>` elements)



### Design Discussion

* The `#` character is useful, e.g. for GitHub issues and pull requests, Twitter hashtags, etc. Should we use `~` for subscript and require two tildes for strikethrough?

* In Markdown, numbered lists can only interrupt a paragraph if they start with the number 1. On one hand, this is an inconsistency and an edge case that probably very few people know about. On the other hand, it is rarely a problem, therefore we gain little by diverging from the CommonMark spec.

  Comparison (the number 4 is used as an example for any non-negative number other than 1):

  <table>
    <tr>
      <th rowspan="2">Input</th>
      <th rowspan="2">Expected</th>
      <th colspan="2">Solution for</th>
      <th rowspan="2">Likeliness</th>
    </tr>
    <tr>
      <th>CommonMark</th>
      <th>Unidoc</th>
    </tr>
    <tr>
      <td>
      <pre style="margin:0">Text<br/>1. Text</pre>
      </td>
      <td align="center">list</td>
      <td colspan="2" align="center"><strong>it works!</strong></td>
      <td align="center">High</td>
    </tr>
    <tr>
      <td>
      <pre style="margin:0">Text<br/>1. Text</pre>
      </td>
      <td align="center">paragraph</td>
      <td colspan="2" align="center">escape the dot</td>
      <td align="center">Very low * †</td>
    </tr>
    <tr>
      <td>
      <pre style="margin:0">Text<br/>4. Text</pre>
      </td>
      <td align="center">list</td>
      <td align="center">insert blank line</td>
      <td align="center"><strong>it works!</strong></td>
      <td align="center">Low ‡</td>
    </tr>
    <tr>
      <td>
      <pre style="margin:0">Text<br/>4. Text</pre>
      </td>
      <td align="center">paragraph</td>
      <td align="center"><strong>it works!</strong></td>
      <td align="center">escape the dot</td>
      <td align="center">Low *</td>
    </tr>
  </table>

  \* This is unlikely because there's no reason to add a line break before the number. The line break would more likely be _after_ the number.

  † This is unlikely because it only applies to the number 1.

  ‡ This is unlikely because lists rarely start with a number other than 1.

* Should blockquotes allow empty lines in between? This is different than in Markdown, where a blank line terminates a blockquote. This might be unexpected, and an inexperienced writer might not know what to do to fix (the fix is to insert a `$`).

* It's odd that macros can attach to `inline code`, but not to other types of inline formatting. This is because inline code has a different parsing strategy than other formatting. Specifically, backticks that surround inline code don't need to be left- or right-flanking; for example, this is valid inline code: `` ` text ` `` but this is not a valid emphasis: `* text *`

  When a macro appears before a formatting delimiter, the parsing strategy could be changed, but is it worth the added complexity? Note that this already works: `@MACRO{**bold text**}`

* It's unfortunate that a sequence of dashes can be either a thematic break or a heading underline. I would prefer if this ambiguity didn't exist, even though it is easy to resolve. However, deprecating thematic breaks made of dashes would break a lot of Markdown documents; one current advantage of Unidoc is that many Markdown documents need no or very few changes to become an equivalent Unidoc document.

* Unidoc tries to behave the same as CommonMark, unless there's a good reason to break compatibility. One interesting case are tables: Tables aren't part of the CommonMark specification, only the GFM (GitHub-flavored Markdown) specification. However, GFM-style tables are supported in many Markdown implementations. Therefore it would make sense if Unidoc tables were backwards-compatible with GFM-style tables.

  I decided against that because I find them inflexible and cumbersome to type. Furthermore, they are only readable when the content fits in a single line, and the line that separates the table header from the body seems out of place when there is no table header. Lastly, GFM-style tables are difficult to parse.

* The Rust AsciiMath implementation used by Unidoc behaves slightly different than the official implementation and might also contain a few bugs. Possible solutions:

  * Use the [official implementation](https://github.com/asciimath/asciimathml/blob/master/ASCIIMathML.js). This requires that NodeJS is installed on the build machine.

  * Don't convert AsciiMath to MathML, and include MathJax with AsciiMath support. Note that the recommended way to use MathJax searches the entire document for text enclosed in `\(`...`\)`, so this is less performant and might also interpret text as Math that isn't supposed to be.

  * Improve `asciimath-rs`, or translate the official JavaScript implementation into Rust

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
