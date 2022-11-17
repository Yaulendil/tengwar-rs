//! Module defining the Mode enum used by the executable binary. Kept separate
//!     from `main.rs` since it is more code than definition.

use tengwar::*;


#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LongVowels {
    /// Always use the separate extended carrier mark.
    //  0: No tehta `char`s will ever follow a base tengwa.
    #[value(alias = "s", alias = "0")]
    Separate,
    /// Where possible, use doubled diacritics.
    //  2: Up to two tehta `char`s may follow a base tengwa.
    #[value(alias = "d", alias = "2")]
    Doubled,
    /// Where possible, use unique diacritics.
    //  1: Up to one tehta `char` may follow a base tengwa.
    #[value(alias = "u", alias = "1")]
    Unique,
}

impl LongVowels {
    pub const DEFAULT: Self = Self::from_lib(VowelStyle::DEFAULT);

    pub const fn from_lib(style: VowelStyle) -> Self {
        match style {
            VowelStyle::Separate => Self::Separate,
            VowelStyle::Doubled => Self::Doubled,
            VowelStyle::Unique => Self::Unique,
        }
    }

    pub const fn style(&self) -> VowelStyle {
        match self {
            Self::Separate => VowelStyle::Separate,
            Self::Doubled => VowelStyle::Doubled,
            Self::Unique => VowelStyle::Unique,
        }
    }
}


#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Mode {
    /// The Classical Mode, used for Quenya.
    #[value(alias = "q")]
    Quenya,
    /// The Mode of Gondor, used for Sindarin.
    #[value(alias = "g")]
    Gondor,
    /// The Mode of Beleriand, used for Sindarin.
    #[value(alias = "b")]
    Beleriand,
    /*/// An orthographic "general" mode for English.
    #[value(alias = "e")]
    English,*/
}

impl Mode {
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self::Quenya;
}


pub struct Runner {
    pub mode: Mode,

    pub alt_a: bool,
    pub alt_rince: bool,
    pub ligate_short: bool,
    pub ligate_zwj: bool,
    pub nuquerna: bool,
    pub vowels: VowelStyle,
}

impl Runner {
    pub const fn new(mode: Mode) -> Self {
        Self {
            mode,
            alt_a: false,
            alt_rince: false,
            ligate_short: false,
            ligate_zwj: false,
            nuquerna: false,
            vowels: VowelStyle::DEFAULT,
        }
    }

    pub fn convert<T: FromIterator<Token>>(&self, input: impl ToTengwar) -> T {
        macro_rules! run {
            ($mode:ty, $input:expr) => {{
                let mut transcriber = $input.transcriber::<$mode>();
                transcriber.settings.alt_a = self.alt_a;
                transcriber.settings.alt_rince = self.alt_rince;
                transcriber.settings.ligate_short = self.ligate_short;
                transcriber.settings.ligate_zwj = self.ligate_zwj;
                transcriber.settings.nuquerna = self.nuquerna;
                transcriber.settings.vowels = self.vowels;
                transcriber.collect()
            }};
        }

        match self.mode {
            Mode::Quenya => run!(Quenya, input),
            Mode::Gondor => run!(Gondor, input),
            Mode::Beleriand => run!(Beleriand, input),
            /*Mode::English => run!(English, input),*/
        }
    }
}
