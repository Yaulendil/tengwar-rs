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
//!     [`Rules`] trait.
//!
//! # Examples
//!
//! [`AsRef<str>`]: AsRef
//! [`transcribe`]: Rules::transcribe
//! [`to_tengwar`]: ToTengwar::to_tengwar
//!
//! The most basic way to convert text is the [`transcribe`] associated function
//!     on the [`Rules`] trait. This function accepts any input type that
//!     implements [`AsRef<str>`].
//! ```
//! use tengwar::{Quenya, Rules};
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
//!     [`Rules::transcribe`] function.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let text: String = "namárië:-".to_tengwar::<Quenya>();
//!
//! if cfg!(feature = "circumflex") {
//!     assert_eq!(text, "");
//! } else {
//!     assert_eq!(text, "");
//! }
//! ```
//!
//! Also available, and likely the easiest to discover via code completion, is
//!     the [`transliterate`] function, which takes an implementor of [`Rules`]
//!     as a generic parameter. This function accepts any input type that
//!     implements [`ToTengwar`], and is a passthrough to the [`to_tengwar`]
//!     method.
//! ```
//! use tengwar::{Quenya, transliterate};
//!
//! let text: String = transliterate::<Quenya>("namárië:-");
//!
//! if cfg!(feature = "circumflex") {
//!     assert_eq!(text, "");
//! } else {
//!     assert_eq!(text, "");
//! }
//! ```

pub mod characters;
mod etc;
pub mod mode;

pub use characters::{Glyph, int_10, int_12, Numeral};
pub use mode::{Beleriand, Gondor, Quenya};

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write},
    iter::{FromIterator, Peekable},
    vec::IntoIter,
};


/// Convert a compatible object (typically text) into the Tengwar.
///
/// This function merely calls a Trait method, but is likely the most readily
///     discoverable part of the library when using code completion tools.
pub fn transliterate<R: Rules>(text: impl ToTengwar) -> String {
    text.to_tengwar::<R>()
}


/// Convert a compatible object into the Tengwar, using Zero-Width Joiners to
///     form ligatures.
///
/// The ligated counterpart of [`transliterate`].
pub fn transliterate_ligated<R: Rules>(text: impl ToTengwarLigated) -> String {
    text.to_tengwar_ligated::<R>()
}


/// A trait implementing the rules for converting text into the Tengwar.
///
/// The only required method is the one to produce a sequence of [`Token`]s;
///     This can be collected into a [`String`] easily enough.
pub trait Rules {
    /// Produce a sequence of [`Token`]s representing the Tengwar form of some
    ///     text.
    fn tokens(input: impl AsRef<str>) -> Vec<Token>;

    /// Produce an iterator of [`Token`]s representing the Tengwar form of some
    ///     text.
    fn token_iter(input: impl AsRef<str>) -> TokenIter<IntoIter<Token>> {
        TokenIter::from(Self::tokens(input))
    }

    /// Produce a sequence of [`Token`]s, and then immediately post-process and
    ///     collect them into a `String`.
    fn transcribe(input: impl AsRef<str>) -> String {
        Self::token_iter(input).collect::<String>()
    }

    /// Produce a sequence of [`Token`]s, and then immediately post-process and
    ///     collect them into a [`String`]. Zero-Width Joiners will be included
    ///     in the output data to form ligatures where appropriate.
    fn transcribe_with_ligatures(input: impl AsRef<str>) -> String {
        Self::token_iter(input).ligated().collect::<String>()
    }
}


/// A very small trait serving to implement ergonomic transliteration methods
///     directly onto text objects.
pub trait ToTengwar {
    /// Transliterate this object into the Tengwar.
    fn to_tengwar<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwar for T {
    /// Transliterate this text into the Tengwar.
    fn to_tengwar<R: Rules>(&self) -> String {
        R::transcribe(self)
    }
}


/// A very small trait serving to implement ergonomic transliteration methods
///     with ligation directly onto text objects.
pub trait ToTengwarLigated {
    /// Transliterate this object into the Tengwar, with ligature processing.
    fn to_tengwar_ligated<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwarLigated for T {
    /// Transliterate this text into the Tengwar. A post-processor will run over
    ///     it to insert zero-width joiners and create ligatures where possible.
    ///     This affects the text data itself, but should not have any visible
    ///     effect with a font that does not support the ligatures.
    fn to_tengwar_ligated<R: Rules>(&self) -> String {
        R::transcribe_with_ligatures(self)
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
    fn ligated(mut self) -> Self {
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
                Some(Token::Tengwa(_)) => {
                    glyph.is_final = false;
                    glyph.ligate_short = self.ligate_short
                        && glyph.is_short_carrier();
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
