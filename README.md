# Unidoc: A powerful, readable, easy-to-learn markup language

This language is inspired by AsciiDoc, but has a syntax that resembles Markdown. It tries to be as simple as possible while offering powerful features such as macros.

## Principles

Unidoc should look familiar if you're familiar with Markdown. It follows the CommonMark specification closely where it makes sense, but it also omits some Markdown features that I feel are unhelpful, and adds new features. Notable examples are:

* It supports `^superscript^`, `#subscript#`, `~line-through~` text, tables, math formulas, and macros
* It allows mixing HTML with other formatting types (e.g. `<kbd>**bold**</kbd>`)
* It does not support indented code blocks, only fenced code blocks;
  this greatly simplifies the rules about indenting
* It does not support _laziness_, i.e. in a list item or blockquote, all lines must be indented or preceded with `>` respectively, not just the first line
* It does not support URLs in angle brackets
* It parses `` `code spans` `` differently: They can contain backslash escapes, and always start and end with exactly 1 backtick.
* Headings (e.g. `## Heading`) can't have trailing number signs
* It doesn't have hard line breaks (use `<br>` instead)
* Thematic breaks (e.g. `----`) can't have internal spaces
