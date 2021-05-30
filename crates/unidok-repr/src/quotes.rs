use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpeningQuotes {
    AsciiDouble, // "
    AsciiSingle, // '

    EnglishDouble, // “
    EnglishSingle, // ‘

    GermanDouble, // „
    GermanSingle, // ‚

    SwedishDouble, // ”
    SwedishSingle, // ’

    GuillementsDouble, // «
    GuillementsSingle, // ‹

    FrenchDouble, // « with space
    FrenchSingle, // ‹ with space

    DanishDouble, // »
    DanishSingle, // ›

    JapaneseFilled, // 「
    JapaneseEmpty,  // 『

    TibetanDouble, // 《
    TibetanSingle, //〈
}

impl OpeningQuotes {
    pub fn to_str(self) -> &'static str {
        use OpeningQuotes::*;
        match self {
            AsciiDouble => "\"",
            AsciiSingle => "'",
            EnglishDouble => "“",
            EnglishSingle => "‘",
            GermanDouble => "„",
            GermanSingle => "‚",
            SwedishDouble => "”",
            SwedishSingle => "’",
            GuillementsDouble => "«",
            GuillementsSingle => "‹",
            FrenchDouble => "«\u{A0}",
            FrenchSingle => "‹\u{A0}",
            DanishDouble => "»",
            DanishSingle => "›",
            JapaneseFilled => "「",
            JapaneseEmpty => "『",
            TibetanDouble => "《",
            TibetanSingle => "〈",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClosingQuotes {
    AsciiDouble, // "
    AsciiSingle, // '

    EnglishDouble, // ”
    EnglishSingle, // ’

    GermanDouble, // “
    GermanSingle, // ‘

    GuillementsDouble, // »
    GuillementsSingle, // ›

    FrenchDouble, // » with space
    FrenchSingle, // › with space

    DanishDouble, // «
    DanishSingle, // ‹

    AlbanianAltDouble, // „
    AlbanianAltSingle, // ‚

    JapaneseFilled, // 」
    JapaneseEmpty,  // 』

    TibetanDouble, // 》
    TibetanSingle, // 〉
}

impl ClosingQuotes {
    pub fn to_str(self) -> &'static str {
        use ClosingQuotes::*;
        match self {
            AsciiDouble => "\"",
            AsciiSingle => "'",
            EnglishDouble => "”",
            EnglishSingle => "’",
            GermanDouble => "“",
            GermanSingle => "‘",
            GuillementsDouble => "»",
            GuillementsSingle => "›",
            FrenchDouble => "\u{A0}»",
            FrenchSingle => "\u{A0}›",
            DanishDouble => "«",
            DanishSingle => "‹",
            AlbanianAltDouble => "„",
            AlbanianAltSingle => "‚",
            JapaneseFilled => "」",
            JapaneseEmpty => "』",
            TibetanDouble => "》",
            TibetanSingle => "〉",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuoteStyle {
    pub double_start: OpeningQuotes,
    pub double_end: ClosingQuotes,
    pub single_start: OpeningQuotes,
    pub single_end: ClosingQuotes,
}

impl QuoteStyle {
    pub fn is_english(&self) -> bool {
        matches!(
            self,
            QuoteStyle {
                double_start: OpeningQuotes::EnglishDouble,
                double_end: ClosingQuotes::EnglishDouble,
                single_start: OpeningQuotes::EnglishSingle,
                single_end: ClosingQuotes::EnglishSingle,
            }
        )
    }

    pub fn ascii() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::AsciiDouble,
            double_end: ClosingQuotes::AsciiDouble,
            single_start: OpeningQuotes::AsciiSingle,
            single_end: ClosingQuotes::AsciiSingle,
        }
    }

    pub fn english() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::EnglishDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::EnglishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    pub fn german() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::GermanDouble,
            single_start: OpeningQuotes::GermanSingle,
            single_end: ClosingQuotes::GermanSingle,
        }
    }

    pub fn guillements() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GuillementsDouble,
            double_end: ClosingQuotes::GuillementsDouble,
            single_start: OpeningQuotes::GuillementsSingle,
            single_end: ClosingQuotes::GuillementsSingle,
        }
    }

    // //////////////////  Special cases  ////////////////// //

    fn albanian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::GermanDouble,
            single_start: OpeningQuotes::EnglishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn arabic_armenian_kazakh_khmer_pashto_persian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GuillementsDouble,
            double_end: ClosingQuotes::GuillementsDouble,
            single_start: OpeningQuotes::AsciiSingle,
            single_end: ClosingQuotes::AsciiSingle,
        }
    }

    fn azerbaijani_belarusian_mongolian_russian_uzbek() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GuillementsDouble,
            double_end: ClosingQuotes::GuillementsDouble,
            single_start: OpeningQuotes::GermanDouble,
            single_end: ClosingQuotes::GermanDouble,
        }
    }

    fn guillements_and_english() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GuillementsDouble,
            double_end: ClosingQuotes::GuillementsDouble,
            single_start: OpeningQuotes::EnglishDouble,
            single_end: ClosingQuotes::EnglishDouble,
        }
    }

    fn bosnian_finnish_hebrew_swedish() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::SwedishDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::SwedishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn bulgarian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::GermanDouble,
            single_start: OpeningQuotes::SwedishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn croatian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::EnglishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn danish() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::DanishDouble,
            double_end: ClosingQuotes::DanishDouble,
            single_start: OpeningQuotes::DanishSingle,
            single_end: ClosingQuotes::DanishSingle,
        }
    }

    fn estonian_georgian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::GermanDouble,
            single_start: OpeningQuotes::AsciiSingle,
            single_end: ClosingQuotes::AsciiSingle,
        }
    }

    fn french() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::FrenchDouble,
            double_end: ClosingQuotes::FrenchDouble,
            single_start: OpeningQuotes::EnglishDouble,
            single_end: ClosingQuotes::EnglishDouble,
        }
    }

    fn hungarian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::DanishDouble,
            single_end: ClosingQuotes::DanishDouble,
        }
    }

    fn ido() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::FrenchDouble,
            double_end: ClosingQuotes::FrenchDouble,
            single_start: OpeningQuotes::EnglishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn japanese_taiwanese_traditional_chinese_new_tai_lue() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::JapaneseFilled,
            double_end: ClosingQuotes::JapaneseFilled,
            single_start: OpeningQuotes::JapaneseEmpty,
            single_end: ClosingQuotes::JapaneseEmpty,
        }
    }

    fn north_korean() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::TibetanDouble,
            double_end: ClosingQuotes::TibetanDouble,
            single_start: OpeningQuotes::TibetanSingle,
            single_end: ClosingQuotes::TibetanSingle,
        }
    }

    fn lao_latvian_vietnamese() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::EnglishDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::AsciiSingle,
            single_end: ClosingQuotes::AsciiSingle,
        }
    }

    fn macedonian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::GermanDouble,
            single_start: OpeningQuotes::SwedishSingle,
            single_end: ClosingQuotes::GermanSingle,
        }
    }

    fn norwegian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GuillementsDouble,
            double_end: ClosingQuotes::GuillementsDouble,
            single_start: OpeningQuotes::EnglishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn polish_romanian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::GuillementsDouble,
            single_end: ClosingQuotes::GuillementsDouble,
        }
    }

    fn serbian() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::GermanDouble,
            double_end: ClosingQuotes::EnglishDouble,
            single_start: OpeningQuotes::SwedishSingle,
            single_end: ClosingQuotes::EnglishSingle,
        }
    }

    fn tai_le_tibetan() -> QuoteStyle {
        QuoteStyle {
            double_start: OpeningQuotes::TibetanDouble,
            double_end: ClosingQuotes::TibetanDouble,
            single_start: OpeningQuotes::TibetanSingle,
            single_end: ClosingQuotes::TibetanSingle,
        }
    }
}

impl Default for QuoteStyle {
    fn default() -> Self {
        QuoteStyle::english()
    }
}

impl FromStr for QuoteStyle {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "afrikaans" | "chinese" | "dutch" | "english" | "esperanto" | "filipino" | "hindi"
            | "indonesian" | "interlingua" | "irish" | "scottish gaelic" | "south korean"
            | "maltese" | "brazilian" | "tamil" | "thai" | "turkish" | "urdu" | "welsh" => {
                QuoteStyle::english()
            }

            "czech" | "german" | "icelandic" | "lithuanian" | "slovak" | "slovene" | "sorbian" => {
                QuoteStyle::german()
            }

            "amharic" | "swiss" | "romansh" | "tigrinya" | "uyghur" => QuoteStyle::guillements(),

            "basque" | "catalan" | "galician" | "greek" | "italian" | "occitan" | "portuguese"
            | "spanish" | "ukrainian" => QuoteStyle::guillements_and_english(),

            "albanian" => QuoteStyle::albanian(),
            "arabic" | "armenian" | "kazakh" | "khmer" | "pashto" | "persian" => {
                QuoteStyle::arabic_armenian_kazakh_khmer_pashto_persian()
            }
            "azerbaijani" | "belarusian" | "mongolian" | "russian" | "uzbek" => {
                QuoteStyle::azerbaijani_belarusian_mongolian_russian_uzbek()
            }
            "bosnian" | "finnish" | "hebrew" | "swedish" => {
                QuoteStyle::bosnian_finnish_hebrew_swedish()
            }
            "bulgarian" => QuoteStyle::bulgarian(),
            "croatian" => QuoteStyle::croatian(),
            "danish" => QuoteStyle::danish(),
            "estonian" | "georgian" => QuoteStyle::estonian_georgian(),
            "french" => QuoteStyle::french(),
            "hungarian" => QuoteStyle::hungarian(),
            "ido" => QuoteStyle::ido(),
            "japanese" | "taiwanese" | "traditional chinese" | "new tai lue" => {
                QuoteStyle::japanese_taiwanese_traditional_chinese_new_tai_lue()
            }
            "north korean" => QuoteStyle::north_korean(),
            "lao" | "latvian" | "vietnamese" => QuoteStyle::lao_latvian_vietnamese(),
            "macedonian" => QuoteStyle::macedonian(),
            "norwegian" => QuoteStyle::norwegian(),
            "polish" | "romanian" => QuoteStyle::polish_romanian(),
            "serbian" => QuoteStyle::serbian(),
            "tai le" | "tibetan" => QuoteStyle::tai_le_tibetan(),

            _ => return Err(()),
        })
    }
}
