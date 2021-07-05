# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2] - 2021-07-05
## Added
- Substitutions: `"`, `'`, `...`, `->`, `<-`, `--`, `(C)`, `(R)`, `(TM)` are now all substituted
  with the appropriate Unicode character
- `@BLANK` macro to make links open in a new tab
- `@CONFIG` macro for global configuration
- `heading_anchor` configuration option which makes it possible to add an anchor to all headings
- `lang` configuration option to specify the quote style
- A work-in-progress API to get token spans, which can be used for syntax highlighting

## Changed
- Grammar change: Block macros don't need to have content. If a macro is followed by a blank line, it is empty.
- API changes: Lots of types were renamed, refactored or expanded with new fields. The API is still unstable, so the changes aren't listed here


## Fixed
- `@PASS` and `@NOPASS` should now work correctly in all positions

## [0.1] - 2021-05-17
Initial release

[Unreleased]: https://github.com/Aloso/unidok/compare/v0.2...HEAD
[0.2]: https://github.com/Aloso/unidok/compare/v0.1...v0.2
[0.1]: https://github.com/Aloso/unidok/releases/tag/v0.1
