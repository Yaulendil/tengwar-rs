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
pub enum Mode {
    /// The Classical Mode, used for Quenya.
    #[value(alias = "c")]
    Classical,
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
    pub const DEFAULT: Self = Self::Classical;

    pub fn convert<T: FromIterator<Token>>(
        &self,
        input: impl ToTengwar,
        settings: TranscriberSettings,
    ) -> T {
        match self {
            Self::Classical => convert::<Quenya, T>(input, settings),
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
