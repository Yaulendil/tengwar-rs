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
//!     and [`Gondor`]. Each mode implements the [`TengwarMode`] trait.
//!
//! # Examples
//!
//! [`AsRef<str>`]: AsRef
//! [`collect`]: Iterator::collect
//! [`transcribe`]: TengwarMode::transcribe
//! [`transcriber`]: ToTengwar::transcriber
//! [`to_tengwar`]: ToTengwar::to_tengwar
//!
//! The most basic way to convert text is the [`transcribe`] associated function
//!     on the [`TengwarMode`] trait. This function accepts any input type that
//!     implements [`AsRef<str>`], and can return any type that implements
//!     `FromIterator<Token>`; This includes `Vec<Token>` and [`String`].
//! ```
//! use tengwar::{Quenya, TengwarMode};
//!
//! let text: String = Quenya::transcribe("namárië:-");
//!
//! assert_eq!(text, "");
//! ```
//!
//! With the use of the [`ToTengwar`] helper trait, two methods are provided on
//!     the input type directly. This trait is automatically implemented for all
//!     types implementing [`AsRef<str>`]. The first method is [`transcriber`],
//!     which constructs a [`Transcriber`] for the text, allowing iteration over
//!     [`Token`]s.
//!
//! The [`Transcriber`] also holds several public fields, which can be changed
//!     to adjust various aspects of its behavior.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let mut transcriber = "namárië:-".transcriber::<Quenya>();
//! transcriber.alt_a = true; // Use the alternate form of the A-tehta.
//!
//! let text: String = transcriber.collect();
//!
//! assert_eq!(text, "");
//! ```
//!
//! The other method is [`to_tengwar`]. This is mostly a convenience method,
//!     which simply calls [`transcriber`] and immediately [`collect`]s the
//!     iterator into the output type.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let text: String = "namárië:-".to_tengwar::<Quenya, String>();
//!
//! assert_eq!(text, "");
//! ```
//!
//! Also available, and likely the easiest to discover via code completion, is
//!     the top-level [`crate::transcribe`] function, which takes an implementor
//!     of [`TengwarMode`] as a generic parameter. This function accepts any
//!     input type that implements [`ToTengwar`], and is a passthrough to the
//!     [`to_tengwar`] method.
//! ```
//! use tengwar::{Quenya, transcribe};
//!
//! let text: String = transcribe::<Quenya>("namárië:-");
//!
//! assert_eq!(text, "");
//! ```

#[macro_use]
extern crate cfg_if;

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
use mode::ModeIter;


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
    /// Create a [`Transcriber`] to iteratively transcribe this text into the
    ///     Tengwar. The returned iterator will yield [`Token`]s.
    fn transcriber<M: TengwarMode>(&self) -> Transcriber<ModeIter<M>>;

    /// Transcribe this object into the Tengwar.
    fn to_tengwar<M: TengwarMode, T: FromIterator<Token>>(&self) -> T {
        self.transcriber::<M>().collect()
    }
}

impl<S: AsRef<str>> ToTengwar for S {
    fn transcriber<M: TengwarMode>(&self) -> Transcriber<ModeIter<M>> {
        ModeIter::from_str(self).into_transcriber()
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


pub struct Transcriber<I: Iterator<Item=Token>> {
    inner: Peekable<I>,
    pub alt_a: bool,
    // pub alt_rince: bool,
    pub ligate_short: bool,
    pub ligate_zwj: bool,
    pub nuquerna: bool,
    // pub vowels: LongVowels,
}

impl<I: Iterator<Item=Token>> Transcriber<I> {
    pub const fn ligated(mut self) -> Self {
        self.ligate_short = true;
        self.ligate_zwj = true;
        self
    }

    pub const fn with_alt_a(mut self) -> Self {
        self.alt_a = true;
        self
    }

    pub const fn with_nuquerna(mut self) -> Self {
        self.nuquerna = true;
        self
    }
}

impl<T: IntoIterator<Item=Token>> From<T> for Transcriber<T::IntoIter> {
    fn from(iter: T) -> Self {
        Self {
            inner: iter.into_iter().peekable(),
            alt_a: false,
            // alt_rince: false,
            ligate_short: false,
            ligate_zwj: false,
            nuquerna: false,
            // vowels: LongVowels::Doubled,
        }
    }
}

impl<I: Iterator<Item=Token>> Iterator for Transcriber<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: Token = self.inner.next()?;

        if let Token::Tengwa(glyph) = &mut token {
            // glyph.rince_alt = self.alt_rince;
            glyph.ligate_zwj = self.ligate_zwj;
            glyph.nuquerna = self.nuquerna;

            if self.alt_a {
                glyph.set_alt_a();
            }

            match self.inner.peek() {
                Some(Token::Tengwa(next)) => {
                    glyph.rince_alt = false;
                    glyph.ligate_short = self.ligate_short
                        && glyph.is_short_carrier()
                        && next.telco_ligates();
                }
                _ => {
                    glyph.rince_alt = true;
                    glyph.ligate_short = false;
                }
            }
        }

        Some(token)
    }
}
