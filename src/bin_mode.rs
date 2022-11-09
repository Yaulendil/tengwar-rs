//! Module defining the Mode enum used by the executable binary. Kept separate
//!     from `main.rs` since it is more code than definition.

use tengwar::*;
// use super::LongVowels;


pub struct Runner {
    pub mode: Mode,

    pub alt_a: bool,
    // pub alt_rince: bool,
    pub ligate_short: bool,
    pub ligate_zwj: bool,
    pub nuquerna: bool,
    // pub vowels: LongVowels,
}

impl Runner {
    pub const fn new(mode: Mode) -> Self {
        Self {
            mode,
            alt_a: false,
            // alt_rince: false,
            ligate_short: false,
            ligate_zwj: false,
            nuquerna: false,
            // vowels: LongVowels::Doubled,
        }
    }

    pub fn convert<T: FromIterator<Token>>(&self, input: impl ToTengwar) -> T {
        macro_rules! run {
            ($mode:ty, $input:expr) => {{
                let mut transcriber = $input.transcriber::<$mode>();
                transcriber.alt_a = self.alt_a;
                // transcriber.alt_rince = self.alt_rince;
                transcriber.ligate_short = self.ligate_short;
                transcriber.ligate_zwj = self.ligate_zwj;
                transcriber.nuquerna = self.nuquerna;
                // transcriber.vowels = self.vowels;
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
