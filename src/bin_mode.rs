//! Module defining the Mode enum used by the executable binary. Kept separate
//!     from `main.rs` since it is more code than definition.

use tengwar::*;


fn convert<M: TengwarMode, T: FromIterator<Token>>(
    input: impl ToTengwar,
    settings: TranscriberSettings,
) -> T {
    input.transcriber::<M>().with_settings(settings).collect()
}


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

    pub fn convert<T: FromIterator<Token>>(
        &self,
        input: impl ToTengwar,
        settings: TranscriberSettings,
    ) -> T {
        match self {
            Self::Quenya => convert::<Quenya, T>(input, settings),
            Self::Gondor => convert::<Gondor, T>(input, settings),
            Self::Beleriand => convert::<Beleriand, T>(input, settings),
            /*Self::English => convert::<English, T>(input, settings),*/
        }
    }
}


pub struct Runner {
    pub mode: Mode,
    pub settings: TranscriberSettings,
}

impl Runner {
    pub const fn new(mode: Mode, settings: TranscriberSettings) -> Self {
        Self { mode, settings }
    }

    pub fn convert<T: FromIterator<Token>>(&self, input: impl ToTengwar) -> T {
        self.mode.convert::<T>(input, self.settings)
    }
}
