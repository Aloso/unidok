# Unidoc: A powerful, readable, easy-to-learn markup language

This language is inspired by AsciiDoc, but has a syntax that resembles Markdown. It tries to be as simple as possible while offering powerful features such as macros.

## Principles

Unidoc should look familiar if you're familiar with Markdown. It follows the CommonMark specification closely where it makes sense, but it also omits some Markdown features that I feel are unhelpful, and adds new features. Notable examples are:

* It supports `^superscript^`, `#subscript#`, `~line-through~` text, tables, math formulas, and macros

* It allows mixing HTML with other formatting types (e.g. `<kbd>**bold**</kbd>`)

* It does not support indented code blocks, only fenced code blocks;
  this greatly simplifies the rules about indenting

* It does not support _laziness_, i.e. in a list item or blockquote,
  all lines must be indented or preceded with `>` respectively, not just the first line

* It does not support URLs in angle brackets

* Headings (e.g. `## Heading`) can't have trailing number signs

* It doesn't have hard line breaks (use `<br>` instead)

* Thematic breaks (e.g. `----`) can't have internal spaces

* Links can be nested

Unidoc tries to only deprecate CommonJS features that are rarely used in practice, and/or that greatly simplify the parsing rules.

Less important:

* Numbered lists can interrupt a paragraph, even if it doesn't start with 1

## Specification

### Paragraphs

A paragraph is a section of text. Paragraphs are separated with a blank<sup>[1]</sup> line. When a paragraph is followed by a list, quote, code block or table, no blank line is needed, just a line break.

### Thematic breaks

A thematic break is rendered as a horizontal line (`<hr>` tag). A thematic break is a line that consists at least three consecutive stars, dashes or underscores. They can't be mixed, and the line can't contain anything else apart from whitespace. For example, these are thematic breaks:

```markdown
---
  *******
```

A thematic break can't come directly after a paragraph, a blank<sup>[1]</sup> line is needed.

### Headings

There are two kinds of headings: ATX headings and setext headings.

#### ATX headings

ATX headings start on a new line with 1 to 6 number signs (`#`), followed by a space or tab. The number of number signs indicates the heading level: If a heading starts with 3 number signs, it's a level-3 heading. For example:

```markdown
# Heading level 1
## Heading level 2
```

#### Setext headings

Setext headings can only be level-1 or level-2. Level-1 headings are underlined with equals (`=`) characters, level-2 headings are underlined with dashes (`-`). They can't be empty. If they appear after a paragraph, that paragraph must be followed by a blank<sup>[1]</sup> line:

```markdown
Heading level 1
===========
Some text

Heading level 2
-----------
More text
```

The underline can be any length, it can even be as little as 1 character long:

```markdown
Heading level 1
=

Heading level 2
----------------------
```

Contrary to ATX headings, setext headings can contain line breaks (but no blank<sup>[1]</sup> lines):

```markdown
This is a
setext heading
--------------
```

### Fenced code blocks

Fenced code blocks are used to display code-like text, usually in a monospace. It is rendered in HTML as a `<pre>` element containing a `<code>` element. By default, no formatting is performed in the code block. For example:

````markdown
```rust
fn foo() -> &'static str {
    "Hello world!"
}
```
````

A code fence is a sequence of at least three consecutive backtick characters (`` ` ``) or tildes (`~`). A fenced code block begins and ends with a code fence. The code fences can't have any text besides whitespace before them in the line. The closing code fence must use the same character as the opening code fence and must be at least as long as the opening code fence.

The line with the opening code fence may optionally contain some text following the code fence; this is trimmed of leading and trailing whitespace and called the info string. If the info string comes after a backtick fence, it may not contain any backtick characters. If the first word in the info string is a programming language, the code is syntax-highlighted.

If the closing code fence is missing, the rest of the document is treated as a fenced code block regardless.

### Quotes

A quote is a text where each line is preceded by a quote marker, i.e. the `>` character. The quote marker can be omitted in blank<sup>[1]</sup> lines. Example:

```markdown
> Unidoc should look familiar if you're familiar
> with Markdown.

> It follows the CommonMark specification closely
> where it makes sense, but it also omits some
> Markdown features that I feel are unhelpful,
> and adds new features.
```

Which is rendered like this:

> Unidoc should look familiar if you're familiar
> with Markdown.
>
> It follows the CommonMark specification closely
> where it makes sense, but it also omits some
> Markdown features that I feel are unhelpful,
> and adds new features.

Quotes can contain other block elements, such as quotes, lists, paragraphs, thematic breaks, and so on.

### Lists

A list is a sequence of list items.

A list item is a line starting with a list marker. It may contain multiple block elements, but all lines after the first one must be indented by the same number of spaces as there are characters in the list marker. Tabs can be used instead of spaces, where one tab corresponds to 4 spaces.

A list marker is either a dash (`-`), a star (`*`), a plus (`+`) or a numeric marker. A numeric marker is a decimal number followed by a dot (`.`) or closing parenthesis (`)`). A list marker must be followed by a space, a tab, or a line ending.

In a list, all list items must be of the same list marker type (e.g. all stars, or all numbers with a dot). If different types of list markers are mixed, the list is divided into several lists:

```markdown
* List item
* List item
- List item
1. List item
2. List item
```

This is rendered as three lists:

* List item
* List item
- List item
1. List item
2. List item

A numbered list can start with any non-negative integer with at most 9 digits. The number in subsequent list items is ignored, since the numbers in HTML lists are always continuously increasing:

```markdown
  4. List item
  6. List item
500. List item
  1. List item
```

Is rendered as


  4. List item
  6. List item
500. List item
  1. List item

Here's an example how elements in a list must be indented:

````markdown
1. Nested list
2. 1) List item
      ```
      A code block
      ```
   2) List item
      - Another sublist
      - > containing a quote
        > * containing another list
````

1. Nested list
2. 1) List item
      ```
      A code block
      ```
   2) List item
      - Another sublist
      - > containing a quote
        > * containing another list

A list is either loose or tight. In a loose list, each list item's text is wrapped in an HTML paragraph (`<p>`), but not in a tight list.

A list is _loose_ if a blank line is anywhere between two list items, or any list item contains two block-level elements with a blank line between them. Otherwise, the list is _tight_:

````markdown
* This list
* is tight.

- This list

- is loose.

* This list is also loose.
* > A quote

  ```
  A code block
  ```
````

The difference is that in a tight list, the paragraphs aren't wrappend in HTML `<p>` elements.

### Tables

> TODO

### Backslash escapes

A backslash escape is an ASCII punctuation character preceded by a backslash (`\`). The backslash isn't displayed, and the escaped punctuation character is displayed verbatim even if it has a special meaning in Unidoc. For example:

```markdown
\# This is not a heading
```

This is rendered as:

\# This is not a heading

If it is followed by either `` ` ``, `*`, `_`, `^`, `~` or `#`, the backslash eagerly escapes as many of the same character as possible. For example:

```markdown
\^^^#test#
```

This escapes all three carets (`^`).

### Limiter

The limiter is the `$` character. It is used to disambiguate where one element ends and another element starts:

- It can be used to split quotes:
  ```markdown
  > This is a quote
  $
  > This is another quote
  ```
  Is rendered as
  > This is a quote

  > This is another quote

- It can be used to separate list items:

  ```markdown
  1. List item
  1. List item
  $
  1. List item
  1. List item
  ```
  Is rendered as
  1. List item
  1. List item

  1) List item
  1) List item

- It can be used to end an escape sequence early:
  ```markdown
  \^^$^test^
  ```
  Here, only the first two carets (`^`) are escaped.

- It can be used to turn a fenced code block into inline code:
  ````markdown
  $```this is
  inline code```
  ````
  Since the first code fence isn't at the line start, it isn't parsed as a fenced code block.

- It can be used to create two consecutive line breaks in a single paragraph:
  ````markdown
  Some text<br>
  $
  and more text
  ````
  Which is sometimes more readable than only one line break. This technique can also be used to make a loose list tight, without removing the blank lines:
  ```markdown
  - List item
    $
  - List item
  ```

### Emphasis and strong emphasis (italic, bold)

https://spec.commonmark.org/0.29/#emphasis-and-strong-emphasis

### Code spans

https://spec.commonmark.org/0.29/#code-spans

> TODO

### Links

https://spec.commonmark.org/0.29/#links

> TODO

### Images

https://spec.commonmark.org/0.29/#images

> TODO

### HTML

> TODO

### Character replacements

> TODO

## Roadmap

* Code blocks

  * Syntax highlighting
  * Support the `@PASS` macro
  * Support callouts
  * Allow closing fence to be indented more than the opening one

* HTML tags

* Lists with numbers

* Table column styling with `@COLS(col1 |col2 |col3 |col3)`

* Links

  * Auto-links
  * Link reference definitions
  * Forbid nested links
  * URLs in angle brackets?

* Expand macros, then lower IR to HTML

* Char replacements

* Handle indenting properly

* Handle multiple blank lines and leading and trailing whitespace properly

* Customize meaning of online formatting delimiters

  * `*`, `_` and `` ` `` are built-in, but the following chars can be used for anything: `~`, `^`, `#`, `+`, `!`, `?`, `´`, `=`, `:`, `;`
  * They only work when text is surrounded by a left-flanking delimiter on the left and a right-flanking delimiter on the right, e.g. the following isn't bold: `Hello ** world **!`

* Allow disabling some parsers:

  * Tables
  * Braces in table cell modifiers
  * HTML: Either accept only allowlisted tags, or don't accept forbidden tags
  * Insecure macros
  * Char replacements (insecure character U+0000 replacement can't be disabled)
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

--------
[1]: #fn1 "Footnote 1"
<a name="fn1">1</a>. A _blank line_ is a line that contains nothing except indentation, possibly including quote markers (`>`), and whitespace.
