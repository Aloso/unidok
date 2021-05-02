use std::fmt;

use crate::utils::While;
use crate::{Parse, StrSlice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElemName {
    // region: Base document structure
    Html,
    Head,
    Body,
    Title,
    // endregion

    // region: Self-closing tags
    Area,
    Base, // in head
    Br,
    Col,
    Embed,
    Hr,
    Img,
    Input,
    Keygen,
    Link,
    Menuitem,
    Meta, // in head
    Param,
    Source,
    Track,
    Wbr,
    // endregion

    // region: Content sectioning
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Main,
    Nav,
    Section,
    // endregion

    // region: Text content
    Blockquote,
    Dd,
    Div,
    Dl,
    Dt,
    Figcaption,
    Figure,
    // Hr
    Li,
    Ol,
    P,
    Pre,
    Ul,
    // endregion

    // region: Inline text
    A,
    Abbr,
    B,
    Bdi,
    Bdo,
    Cite,
    Code,
    Data,
    Dfn,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rb,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    // endregion

    // region: Table content
    Caption,
    // Col
    Colgroup,
    Table,
    Tbody,
    Td,
    Tfoot,
    Th,
    Thead,
    Tr,
    // endregion

    // region: Forms
    Button,
    Datalist,
    Fieldset,
    Form,
    // Input
    Label,
    Legend,
    Meter,
    Optgroup,
    Option,
    Output,
    Progress,
    Select,
    Textarea,
    // endregion

    // region: Image and multimedia
    // Area
    Audio,
    // Img
    Map,
    // Track
    Video,
    // endregion

    // region: Embedded content
    // Embed
    Iframe,
    Object,
    // Param
    Picture,
    Portal,
    // Source,
    // endregion

    // region: Special content
    Script, // JS
    Svg,    // XML
    Style,  // CSS
    Math,   // MathML
    // endregion

    // region: Deprecated
    Acronym,
    Applet,
    Basefont,
    Bgsound,
    Big,
    Center,
    Content,
    Dir,
    Font,
    Frame,
    Frameset,
    Hgroup,
    Image,
    Isindex,
    // Keygen,
    Listing,
    Marquee,
    // Menuitem,
    Multicol,
    Nextid,
    Nobr,
    Noembed,
    Noframes,
    Plaintext,
    // Rb,
    // Rtc,
    Shadow,
    Spacer,
    Strike,
    Tt,
    Xmp,
    // endregion

    // region: Interactive
    Details,
    Dialog,
    Menu,
    Summary,
    // endregion

    // region: Other
    Canvas,
    Command,
    Del,
    Ins,
    Noscript,
    Template,
    // endregion

    // Custom elements
    Custom(StrSlice),
}

impl ElemName {
    pub(crate) fn parser() -> ParseElemName {
        ParseElemName
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ElemName::Html => "html",
            ElemName::Head => "head",
            ElemName::Body => "body",
            ElemName::Title => "title",
            ElemName::Area => "area",
            ElemName::Base => "base",
            ElemName::Br => "br",
            ElemName::Col => "col",
            ElemName::Embed => "embed",
            ElemName::Hr => "hr",
            ElemName::Img => "img",
            ElemName::Input => "input",
            ElemName::Keygen => "keygen",
            ElemName::Link => "link",
            ElemName::Menuitem => "menuitem",
            ElemName::Meta => "meta",
            ElemName::Param => "param",
            ElemName::Source => "source",
            ElemName::Track => "track",
            ElemName::Wbr => "wbr",
            ElemName::Address => "address",
            ElemName::Article => "article",
            ElemName::Aside => "aside",
            ElemName::Footer => "footer",
            ElemName::Header => "header",
            ElemName::H1 => "h1",
            ElemName::H2 => "h2",
            ElemName::H3 => "h3",
            ElemName::H4 => "h4",
            ElemName::H5 => "h5",
            ElemName::H6 => "h6",
            ElemName::Main => "main",
            ElemName::Nav => "nav",
            ElemName::Section => "section",
            ElemName::Blockquote => "blockquote",
            ElemName::Dd => "dd",
            ElemName::Div => "div",
            ElemName::Dl => "dl",
            ElemName::Dt => "dt",
            ElemName::Figcaption => "figcaption",
            ElemName::Figure => "figure",
            ElemName::Li => "li",
            ElemName::Ol => "ol",
            ElemName::P => "p",
            ElemName::Pre => "pre",
            ElemName::Ul => "ul",
            ElemName::A => "a",
            ElemName::Abbr => "abbr",
            ElemName::B => "b",
            ElemName::Bdi => "bdi",
            ElemName::Bdo => "bdo",
            ElemName::Cite => "cite",
            ElemName::Code => "code",
            ElemName::Data => "data",
            ElemName::Dfn => "dfn",
            ElemName::Em => "em",
            ElemName::I => "i",
            ElemName::Kbd => "kbd",
            ElemName::Mark => "mark",
            ElemName::Q => "q",
            ElemName::Rb => "rb",
            ElemName::Rp => "rp",
            ElemName::Rt => "rt",
            ElemName::Rtc => "rtc",
            ElemName::Ruby => "ruby",
            ElemName::S => "s",
            ElemName::Samp => "samp",
            ElemName::Small => "small",
            ElemName::Span => "span",
            ElemName::Strong => "strong",
            ElemName::Sub => "sub",
            ElemName::Sup => "sup",
            ElemName::Time => "time",
            ElemName::U => "u",
            ElemName::Var => "var",
            ElemName::Caption => "caption",
            ElemName::Colgroup => "colgroup",
            ElemName::Table => "table",
            ElemName::Tbody => "tbody",
            ElemName::Td => "td",
            ElemName::Tfoot => "tfoot",
            ElemName::Th => "th",
            ElemName::Thead => "thead",
            ElemName::Tr => "tr",
            ElemName::Button => "button",
            ElemName::Datalist => "datalist",
            ElemName::Fieldset => "fieldset",
            ElemName::Form => "form",
            ElemName::Label => "label",
            ElemName::Legend => "legend",
            ElemName::Meter => "meter",
            ElemName::Optgroup => "optgroup",
            ElemName::Option => "option",
            ElemName::Output => "output",
            ElemName::Progress => "progress",
            ElemName::Select => "select",
            ElemName::Textarea => "textarea",
            ElemName::Audio => "audio",
            ElemName::Map => "map",
            ElemName::Video => "video",
            ElemName::Iframe => "iframe",
            ElemName::Object => "object",
            ElemName::Picture => "picture",
            ElemName::Portal => "portal",
            ElemName::Script => "script",
            ElemName::Svg => "svg",
            ElemName::Style => "style",
            ElemName::Math => "math",
            ElemName::Acronym => "acronym",
            ElemName::Applet => "applet",
            ElemName::Basefont => "basefont",
            ElemName::Bgsound => "bgsound",
            ElemName::Big => "big",
            ElemName::Center => "center",
            ElemName::Content => "content",
            ElemName::Dir => "dir",
            ElemName::Font => "font",
            ElemName::Frame => "frame",
            ElemName::Frameset => "frameset",
            ElemName::Hgroup => "hgroup",
            ElemName::Image => "image",
            ElemName::Isindex => "isindex",
            ElemName::Listing => "listing",
            ElemName::Marquee => "marquee",
            ElemName::Multicol => "multicol",
            ElemName::Nextid => "nextid",
            ElemName::Nobr => "nobr",
            ElemName::Noembed => "noembed",
            ElemName::Noframes => "noframes",
            ElemName::Plaintext => "plaintext",
            ElemName::Shadow => "shadow",
            ElemName::Spacer => "spacer",
            ElemName::Strike => "strike",
            ElemName::Tt => "tt",
            ElemName::Xmp => "xmp",
            ElemName::Details => "details",
            ElemName::Dialog => "dialog",
            ElemName::Menu => "menu",
            ElemName::Summary => "summary",
            ElemName::Canvas => "canvas",
            ElemName::Command => "command",
            ElemName::Del => "del",
            ElemName::Ins => "ins",
            ElemName::Noscript => "noscript",
            ElemName::Template => "template",
            ElemName::Custom(_) => "(custom)",
        }
    }

    #[rustfmt::skip]
    pub fn is_self_closing(&self) -> bool {
        use ElemName::*;

        matches!(self, Area | Base | Br | Col | Embed | Hr | Img | Input | Keygen
            | Link | Menuitem | Meta | Param | Source | Track | Wbr)
    }

    #[rustfmt::skip]
    pub fn is_block_level(&self) -> bool {
        use ElemName::*;

        !matches!(self, A | Abbr | B | Bdi | Bdo | Cite | Code | Data | Dfn | Em
            | I | Kbd | Mark | Q | Rb | Rp | Rt | Rtc | Ruby | S | Samp | Small
            | Span | Strong | Sub | Sup | Time | U | Var)
    }

    #[rustfmt::skip]
    pub fn is_deprecated(&self) -> bool {
        use ElemName::*;

        matches!(self, Acronym | Applet | Basefont | Bgsound | Big | Center | Content | Dir
            | Font | Frame | Frameset | Hgroup | Image | Isindex | Keygen | Listing
            | Marquee | Menuitem | Multicol | Nextid | Nobr | Noembed | Noframes | Plaintext
            | Rb | Rtc | Shadow | Spacer | Strike | Tt | Xmp)
    }

    #[rustfmt::skip]
    pub fn can_contain_blocks(&self) -> bool {
        use ElemName::*;

        matches!(self, Html | Head | Body | Address | Article | Aside | Header | Footer | Main | Nav
            | Section | Pre | Td | Th | Div | Blockquote | Li | Form | Details | Canvas | Noscript | Custom(_))
    }

    #[rustfmt::skip]
    pub fn contains_plaintext(&self) -> bool {
        use ElemName::*;

        matches!(self, Script | Style)
    }

    #[rustfmt::skip]
    pub fn must_contain_blocks(&self) -> bool {
        use ElemName::*;

        matches!(self, Html | Head | Ul | Ol | Table | Tbody | Thead | Tfoot | Tr | Colgroup
            | Audio | Video)
    }

    fn from(s: &'_ str, slice: StrSlice) -> Self {
        if let Some(c) = s.chars().next() {
            match c {
                'a' => match s {
                    "a" => ElemName::A,
                    "abbr" => ElemName::Abbr,
                    "acronym" => ElemName::Acronym,
                    "address" => ElemName::Address,
                    "applet" => ElemName::Applet,
                    "area" => ElemName::Area,
                    "article" => ElemName::Article,
                    "aside" => ElemName::Aside,
                    "audio" => ElemName::Audio,
                    _ => ElemName::custom(slice),
                },
                'b' => match s {
                    "b" => ElemName::B,
                    "base" => ElemName::Base,
                    "basefont" => ElemName::Basefont,
                    "bdi" => ElemName::Bdi,
                    "bdo" => ElemName::Bdo,
                    "bgsound" => ElemName::Bgsound,
                    "big" => ElemName::Big,
                    "blockquote" => ElemName::Blockquote,
                    "body" => ElemName::Body,
                    "br" => ElemName::Br,
                    "button" => ElemName::Button,
                    _ => ElemName::custom(slice),
                },
                'c' => match s {
                    "canvas" => ElemName::Canvas,
                    "caption" => ElemName::Caption,
                    "center" => ElemName::Center,
                    "cite" => ElemName::Cite,
                    "code" => ElemName::Code,
                    "col" => ElemName::Col,
                    "colgroup" => ElemName::Colgroup,
                    "command" => ElemName::Command,
                    "content" => ElemName::Content,
                    _ => ElemName::custom(slice),
                },
                'd' => match s {
                    "data" => ElemName::Data,
                    "datalist" => ElemName::Datalist,
                    "dd" => ElemName::Dd,
                    "del" => ElemName::Del,
                    "details" => ElemName::Details,
                    "dfn" => ElemName::Dfn,
                    "dialog" => ElemName::Dialog,
                    "dir" => ElemName::Dir,
                    "div" => ElemName::Div,
                    "dl" => ElemName::Dl,
                    "dt" => ElemName::Dt,
                    _ => ElemName::custom(slice),
                },
                'e' => match s {
                    "em" => ElemName::Em,
                    "embed" => ElemName::Embed,
                    _ => ElemName::custom(slice),
                },
                'f' => match s {
                    "fieldset" => ElemName::Fieldset,
                    "figcaption" => ElemName::Figcaption,
                    "figure" => ElemName::Figure,
                    "font" => ElemName::Font,
                    "footer" => ElemName::Footer,
                    "form" => ElemName::Form,
                    "frame" => ElemName::Frame,
                    "frameset" => ElemName::Frameset,
                    _ => ElemName::custom(slice),
                },
                'h' => match s {
                    "h1" => ElemName::H1,
                    "h2" => ElemName::H2,
                    "h3" => ElemName::H3,
                    "h4" => ElemName::H4,
                    "h5" => ElemName::H5,
                    "h6" => ElemName::H6,
                    "head" => ElemName::Head,
                    "header" => ElemName::Header,
                    "hgroup" => ElemName::Hgroup,
                    "hr" => ElemName::Hr,
                    "html" => ElemName::Html,
                    _ => ElemName::custom(slice),
                },
                'i' => match s {
                    "i" => ElemName::I,
                    "iframe" => ElemName::Iframe,
                    "img" => ElemName::Img,
                    "image" => ElemName::Image,
                    "input" => ElemName::Input,
                    "ins" => ElemName::Ins,
                    "isindex" => ElemName::Isindex,
                    _ => ElemName::custom(slice),
                },
                'k' => match s {
                    "kbd" => ElemName::Kbd,
                    "keygen" => ElemName::Keygen,
                    _ => ElemName::custom(slice),
                },
                'l' => match s {
                    "label" => ElemName::Label,
                    "legend" => ElemName::Legend,
                    "li" => ElemName::Li,
                    "link" => ElemName::Link,
                    "listing" => ElemName::Listing,
                    _ => ElemName::custom(slice),
                },
                'm' => match s {
                    "main" => ElemName::Main,
                    "map" => ElemName::Map,
                    "mark" => ElemName::Mark,
                    "marquee" => ElemName::Marquee,
                    "math" => ElemName::Math,
                    "menu" => ElemName::Menu,
                    "menuitem" => ElemName::Menuitem,
                    "meta" => ElemName::Meta,
                    "meter" => ElemName::Meter,
                    "multicol" => ElemName::Multicol,
                    _ => ElemName::custom(slice),
                },
                'n' => match s {
                    "nav" => ElemName::Nav,
                    "nextid" => ElemName::Nextid,
                    "nobr" => ElemName::Nobr,
                    "noembed" => ElemName::Noembed,
                    "noframes" => ElemName::Noframes,
                    "noscript" => ElemName::Noscript,
                    _ => ElemName::custom(slice),
                },
                'o' => match s {
                    "object" => ElemName::Object,
                    "ol" => ElemName::Ol,
                    "optgroup" => ElemName::Optgroup,
                    "option" => ElemName::Option,
                    "output" => ElemName::Output,
                    _ => ElemName::custom(slice),
                },
                'p' => match s {
                    "p" => ElemName::P,
                    "param" => ElemName::Param,
                    "picture" => ElemName::Picture,
                    "plaintext" => ElemName::Plaintext,
                    "portal" => ElemName::Portal,
                    "pre" => ElemName::Pre,
                    "progress" => ElemName::Progress,
                    _ => ElemName::custom(slice),
                },
                'q' => match s {
                    "q" => ElemName::Q,
                    _ => ElemName::custom(slice),
                },
                'r' => match s {
                    "rb" => ElemName::Rb,
                    "rp" => ElemName::Rp,
                    "rt" => ElemName::Rt,
                    "rtc" => ElemName::Rtc,
                    "ruby" => ElemName::Ruby,
                    _ => ElemName::custom(slice),
                },
                's' => match s {
                    "s" => ElemName::S,
                    "samp" => ElemName::Samp,
                    "script" => ElemName::Script,
                    "section" => ElemName::Section,
                    "select" => ElemName::Select,
                    "shadow" => ElemName::Shadow,
                    "small" => ElemName::Small,
                    "source" => ElemName::Source,
                    "spacer" => ElemName::Spacer,
                    "span" => ElemName::Span,
                    "strike" => ElemName::Strike,
                    "strong" => ElemName::Strong,
                    "style" => ElemName::Style,
                    "sub" => ElemName::Sub,
                    "summary" => ElemName::Summary,
                    "sup" => ElemName::Sup,
                    "svg" => ElemName::Svg,
                    _ => ElemName::custom(slice),
                },
                't' => match s {
                    "table" => ElemName::Table,
                    "tbody" => ElemName::Tbody,
                    "td" => ElemName::Td,
                    "template" => ElemName::Template,
                    "textarea" => ElemName::Textarea,
                    "tfoot" => ElemName::Tfoot,
                    "th" => ElemName::Th,
                    "thead" => ElemName::Thead,
                    "time" => ElemName::Time,
                    "title" => ElemName::Title,
                    "tr" => ElemName::Tr,
                    "track" => ElemName::Track,
                    "tt" => ElemName::Tt,
                    _ => ElemName::custom(slice),
                },
                'u' => match s {
                    "u" => ElemName::U,
                    "ul" => ElemName::Ul,
                    _ => ElemName::custom(slice),
                },
                _ => match s {
                    "var" => ElemName::Var,
                    "video" => ElemName::Video,
                    "wbr" => ElemName::Wbr,
                    "xmp" => ElemName::Xmp,
                    _ => ElemName::custom(slice),
                },
            }
        } else {
            ElemName::custom(Default::default())
        }
    }

    fn custom(s: StrSlice) -> Self {
        ElemName::Custom(s)
    }
}

impl fmt::Display for ElemName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub(crate) struct ParseElemName;

impl Parse for ParseElemName {
    type Output = ElemName;

    fn parse(&self, input: &mut crate::input::Input) -> Option<Self::Output> {
        let mut input2 = input.start();

        let name =
            input2.parse_i(While(|c: char| c.is_ascii_alphanumeric() || matches!(c, '-' | '_')));
        if name.is_empty() {
            return None;
        }
        let next = input2.peek_char();
        if !matches!(next, Some(' ' | '\t' | '\r' | '\n' | '/' | '>')) {
            return None;
        }

        input2.apply();
        let elem = ElemName::from(name.to_str(input.text()), name);
        if let ElemName::Custom(_) = elem {
            return None;
        }
        Some(elem)
    }
}
