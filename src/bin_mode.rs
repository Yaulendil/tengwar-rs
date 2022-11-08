//! Module defining the Mode enum used by the executable binary. Kept separate
//!     from `main.rs` since it is more code than definition.

use tengwar::*;


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

    pub fn convert(&self, input: impl AsRef<str>, short: bool, zwj: bool) -> String {
        match self {
            Self::Quenya => {
                let mut iter = input.transcriber::<Quenya>();
                iter.ligate_short = short;
                iter.ligate_zwj = zwj;
                iter.collect()
            }
            Self::Gondor => {
                let mut iter = input.transcriber::<Gondor>();
                iter.ligate_short = short;
                iter.ligate_zwj = zwj;
                iter.collect()
            }
            Self::Beleriand => {
                let mut iter = input.transcriber::<Beleriand>();
                iter.ligate_short = short;
                iter.ligate_zwj = zwj;
                iter.collect()
            }
            /*Self::English => {
                let mut iter = input.tengwar_iter::<English>();
                iter.ligate_short = short;
                iter.ligate_zwj = lig;
                iter.collect()
            }*/
        }
    }
}
