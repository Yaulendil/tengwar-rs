//! Library for conversion of Latin UTF-8 text into Tengwar, using the unicode
//!     codepoints of the [Free Tengwar Font Project]. Specifically, but not
//!     exclusively, designed with [Tengwar Telcontar] in mind, for the purpose
//!     of use within LaTeX macros.
//!
//! [Free Tengwar Font Project]: http://freetengwar.sourceforge.net/mapping.html
//! [Tengwar Telcontar]: http://freetengwar.sourceforge.net/tengtelc.html
//!
//! # Modes
//!
//! Three modes are currently provided: [`Quenya`] ("Classical"), [`Beleriand`],
//!     and [`Gondor`]. Each mode is a zero-size singleton that implements the
//!     [`TengwarMode`] trait.
//!
//! # Examples
//!
//! [`AsRef<str>`]: AsRef
//! [`transcribe`]: TengwarMode::transcribe
//! [`to_tengwar`]: ToTengwar::to_tengwar
//!
//! The most basic way to convert text is the [`transcribe`] associated function
//!     on the [`TengwarMode`] trait. This function accepts any input type that
//!     implements [`AsRef<str>`].
//! ```
//! use tengwar::{Quenya, TengwarMode};
//!
//! let text: String = Quenya::transcribe("namárië:-");
//!
//! if cfg!(feature = "circumflex") {
//!     assert_eq!(text, "");
//! } else {
//!     assert_eq!(text, "");
//! }
//! ```
//!
//! With the use of the [`ToTengwar`] helper trait, a method is provided on the
//!     input type directly. This trait is automatically implemented for types
//!     that implement [`AsRef<str>`], where it is a simple passthrough to the
//!     [`TengwarMode::transcribe`] function.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let text: String = "namárië:-".to_tengwar::<Quenya, String>();
//!
//! if cfg!(feature = "circumflex") {
//!     assert_eq!(text, "");
//! } else {
//!     assert_eq!(text, "");
//! }
//! ```
//!
//! Also available, and likely the easiest to discover via code completion, is
//!     the crate-level [`transcribe`] function, which takes an implementor of
//!     [`TengwarMode`] as a generic parameter. This function accepts any input
//!     type that implements [`ToTengwar`], and is a passthrough to the
//!     [`to_tengwar`] method.
//! ```
//! use tengwar::{Quenya, transcribe};
//!
//! let text: String = transcribe::<Quenya>("namárië:-");
//!
//! if cfg!(feature = "circumflex") {
//!     assert_eq!(text, "");
//! } else {
//!     assert_eq!(text, "");
//! }
//! ```

pub mod characters;
pub mod mode;
mod policy;

pub use characters::{Glyph, Numeral};
pub use mode::{Beleriand, Gondor, Quenya, TengwarMode};

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write},
    iter::{FromIterator, Peekable},
};
use crate::mode::ModeIter;


/// Convert a compatible object (typically text) into the Tengwar.
///
/// This function merely calls a Trait method, but is likely the most readily
///     discoverable part of the library when using code completion tools.
pub fn transcribe<M: TengwarMode>(text: impl ToTengwar) -> String {
    text.to_tengwar::<M, String>()
}


/// A very small trait serving to implement ergonomic transcription methods
///     directly onto text objects.
pub trait ToTengwar {
    /// Create a [`TokenIter`] to progressively transcribe this text into the
    ///     Tengwar. The returned iterator will yield [`Token`]s.
    fn tengwar_iter<M: TengwarMode>(&self) -> TokenIter<ModeIter<M>>;

    /// Transcribe this object into the Tengwar.
    fn to_tengwar<M: TengwarMode, T: FromIterator<Token>>(&self) -> T {
        self.tengwar_iter::<M>().collect()
    }
}

impl<S: AsRef<str>> ToTengwar for S {
    fn tengwar_iter<M: TengwarMode>(&self) -> TokenIter<ModeIter<M>> {
        ModeIter::from_str(self).into_token_iter()
    }
}


/// A small container for either plain text or a [`Glyph`] specification. Serves
///     as the top level of throughput for the transliteration process.
#[derive(Clone, Debug)]
pub enum Token {
    /// A single Unicode codepoint.
    Char(char),
    /// A numeric value.
    Number(Numeral<isize>),
    /// UTF-8 text data.
    String(Cow<'static, str>),
    /// A specified base character and any extra tags it requires.
    Tengwa(Glyph),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Char(ch) => f.write_char(ch),
            Self::Number(n) => f.write_str(&n.render()),
            Self::String(s) => f.write_str(s),
            Self::Tengwa(t) => t.fmt(f),
        }
    }
}

impl FromIterator<Token> for String {
    fn from_iter<T: IntoIterator<Item=Token>>(iter: T) -> Self {
        let mut iter = iter.into_iter().peekable();
        let mut buf = String::new();

        while let Some(token) = iter.next() {
            write!(buf, "{token}").expect("Error writing String");

            if let Token::Tengwa(current) = token {
                if let Some(Token::Tengwa(next)) = iter.peek() {
                    if current.ligate_zwj && current.ligates_with(next) {
                        buf.push(characters::ZWJ);
                    }
                }
            }
        }

        buf
    }
}


pub struct TokenIter<I: Iterator<Item=Token>> {
    inner: Peekable<I>,
    pub ligate_short: bool,
    pub ligate_zwj: bool,
}

impl<I: Iterator<Item=Token>> TokenIter<I> {
    pub const fn ligated(mut self) -> Self {
        self.ligate_short = true;
        self.ligate_zwj = true;
        self
    }
}

impl<T: IntoIterator<Item=Token>> From<T> for TokenIter<T::IntoIter> {
    fn from(iter: T) -> Self {
        Self {
            inner: iter.into_iter().peekable(),
            ligate_short: false,
            ligate_zwj: false,
        }
    }
}

impl<I: Iterator<Item=Token>> Iterator for TokenIter<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: Token = self.inner.next()?;

        if let Token::Tengwa(glyph) = &mut token {
            glyph.ligate_zwj = self.ligate_zwj;

            match self.inner.peek() {
                Some(Token::Tengwa(next)) => {
                    glyph.is_final = false;
                    glyph.ligate_short = self.ligate_short
                        && glyph.is_short_carrier()
                        && next.telco_ligates();
                }
                _ => {
                    glyph.is_final = true;
                    glyph.ligate_short = false;
                }
            }
        }

        Some(token)
    }
}
