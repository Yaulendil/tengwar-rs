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
//! [`collect`]: Iterator::collect
//!
//! ## TengwarMode trait
//!
//! The most basic way to convert text is the [`TengwarMode::transcribe`]
//!     associated function. This function accepts any input type that
//!     implements `AsRef<str>`, and can return any type that implements
//!     `FromIterator<Token>`; This includes `Vec<Token>` and [`String`].
//! ```
//! use tengwar::{Quenya, TengwarMode};
//!
//! let text: String = Quenya::transcribe("namárië !");
//! assert_eq!(text, " ");
//! ```
//!
//! ## ToTengwar trait
//!
//! With the use of the [`ToTengwar`] helper trait, some methods are provided on
//!     the input type directly. This trait is automatically implemented for any
//!     type implementing `AsRef<str>`. The first is [`ToTengwar::transcriber`],
//!     which constructs a [`Transcriber`] for the text, allowing iteration over
//!     [`Token`]s.
//!
//! The `Transcriber` also has [`TranscriberSettings`], holding several public
//!     fields, which can be changed to adjust various aspects of its behavior.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let mut transcriber = "namárië !".transcriber::<Quenya>();
//! transcriber.settings.alt_a = true; // Use the alternate form of the A-tehta.
//!
//! let text: String = transcriber.collect();
//! assert_eq!(text, " ");
//! ```
//!
//! The second method is [`ToTengwar::to_tengwar`]. This is mostly a convenience
//!     method, which simply calls [`ToTengwar::transcriber`] and immediately
//!     [`collect`]s the Iterator into a [`String`].
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let text: String = "namárië !".to_tengwar::<Quenya>();
//! assert_eq!(text, " ");
//! ```
//!
//! The third method is [`ToTengwar::to_tengwar_with`], which does the same, but
//!     takes [`TranscriberSettings`] to modify the [`Transcriber`] before it is
//!     collected. This allows settings to be specified once and reused.
//! ```
//! use tengwar::{Quenya, ToTengwar, TranscriberSettings};
//!
//! let mut settings = TranscriberSettings::new();
//! settings.alt_a = true;
//! settings.nuquerna = true;
//!
//! let text: String = "namárië !".to_tengwar_with::<Quenya>(settings);
//! assert_eq!(text, " ");
//!
//! let text: String = "lotsë súva".to_tengwar_with::<Quenya>(settings);
//! assert_eq!(text, " ");
//! ```
//!
//! ## Crate-level function
//!
//! Also available, and likely the easiest to discover via code completion, is
//!     the top-level [`crate::transcribe`] function, which takes an implementor
//!     of [`TengwarMode`] as a generic parameter. This function accepts any
//!     input type that implements [`ToTengwar`], and is a passthrough to the
//!     [`ToTengwar::to_tengwar`] method.
//! ```
//! use tengwar::{Quenya, transcribe};
//!
//! let text: String = transcribe::<Quenya>("namárië !");
//! assert_eq!(text, " ");
//! ```

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate clap;

#[macro_use]
#[cfg(feature = "serde")]
extern crate serde;

pub mod characters;
pub mod mode;
mod policy;

pub use characters::{Glyph, Numeral, VowelStyle};
pub use mode::{Beleriand, Gondor, Quenya, TengwarMode};

use std::{
    fmt::{Display, Formatter, Write},
    iter::{FromIterator, Peekable},
};
use mode::Tokenizer;


/// Convert a compatible object (typically text) into the Tengwar.
///
/// This function merely calls a Trait method, but is likely the most readily
///     discoverable part of the library when using code completion tools.
pub fn transcribe<M: TengwarMode>(text: impl ToTengwar) -> String {
    text.to_tengwar::<M>()
}


/// A very small trait serving to implement ergonomic transcription methods
///     directly onto text objects.
pub trait ToTengwar {
    /// Create a [`Transcriber`] to iteratively transcribe this text into the
    ///     Tengwar. The returned iterator will yield [`Token`]s.
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar, VowelStyle};
    ///
    /// const INPUT: &str = "lotsë súva"; // "a flower is sinking"
    ///
    ///
    /// //  Collect directly with default settings.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Unique Tehtar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.vowels = VowelStyle::Unique;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Nuquernë Tengwar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.nuquerna = true;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Unique Tehtar and Nuquernë Tengwar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.nuquerna = true;
    /// ts.settings.vowels = VowelStyle::Unique;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use several options.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.alt_a = true;
    /// ts.settings.alt_rince = true;
    /// ts.settings.nuquerna = true;
    /// ts.settings.vowels = VowelStyle::Separate;
    /// assert_eq!(ts.into_string(), " ");
    /// ```
    fn transcriber<M: TengwarMode>(&self) -> Transcriber<M>;

    /// Transcribe this object into the Tengwar directly.
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar};
    ///
    /// let text: String = "namárië !".to_tengwar::<Quenya>();
    /// assert_eq!(text, " ");
    /// ```
    fn to_tengwar<M: TengwarMode>(&self) -> String {
        self.transcriber::<M>().into_string()
    }

    /// Transcribe this object into the Tengwar, using [`TranscriberSettings`]
    ///     provided as an argument. This allows the settings to be reused much
    ///     more easily.
    ///
    /// For examples of the available settings, see the documentation of
    ///     [`Self::transcriber`].
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar, TranscriberSettings};
    ///
    /// let mut settings = TranscriberSettings::new();
    /// settings.alt_a = true;
    /// settings.nuquerna = true;
    ///
    /// let text: String = "namárië !".to_tengwar_with::<Quenya>(settings);
    /// assert_eq!(text, " ");
    ///
    /// let text: String = "lotsë súva".to_tengwar_with::<Quenya>(settings);
    /// assert_eq!(text, " ");
    /// ```
    fn to_tengwar_with<M: TengwarMode>(&self, settings: TranscriberSettings) -> String {
        self.transcriber::<M>().with_settings(settings).into_string()
    }
}

impl<S: AsRef<str>> ToTengwar for S {
    fn transcriber<M: TengwarMode>(&self) -> Transcriber<M> {
        Tokenizer::from_str(self).into_transcriber()
    }
}


/// A small container for either plain text or a [`Glyph`] specification. Serves
///     as the top level of throughput for the transliteration process.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Token {
    /// A single Unicode codepoint.
    Char(char),
    /// A specified base character and any extra tags it requires.
    Glyph(Glyph),
    /// A numeric value.
    Number(Numeral),
    // /// UTF-8 text data.
    // String(Cow<'static, str>),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Char(ch) => f.write_char(ch),
            Self::Glyph(t) => t.fmt(f),
            Self::Number(n) => n.fmt(f),
            // Self::String(s) => f.write_str(s),
        }
    }
}

impl FromIterator<Token> for String {
    fn from_iter<T: IntoIterator<Item=Token>>(iter: T) -> Self {
        let mut iter = iter.into_iter().peekable();
        let mut buf = String::new();

        while let Some(token) = iter.next() {
            write!(buf, "{token}").expect("Error writing String");

            if let Token::Glyph(current) = token {
                if let Some(Token::Glyph(next)) = iter.peek() {
                    if 0 < current.ligate_zwj && current.ligates_with(next) {
                        buf.push(characters::ZWJ);
                    }
                }
            }
        }

        buf
    }
}


/// An iterator over a sequence of [`Token`]s which applies various rules. This
///     is the top level construct of the transcription process.
///
/// This type is a special case of a [`TokenIter`], where the internal iterator
///     is a [`Tokenizer`].
pub type Transcriber<M> = TokenIter<Tokenizer<M>>;


/// An iterator over a sequence of [`Token`]s which applies various rules. This
///     is the top level construct of the transcription process.
///
/// This iterator is intended to work with a [`Tokenizer`], but is able to wrap
///     any type that iterates over `Token`s. Whether this would be useful is
///     not yet clear, but it is likely a good capability to have, just in case.
pub struct TokenIter<I: Iterator<Item=Token>> {
    inner: Peekable<I>,
    last: Option<Token>,
    pub settings: TranscriberSettings,
}

impl<I: Iterator<Item=Token>> TokenIter<I> {
    /// Construct a TokenIter around an Iterator of [`Token`]s.
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter.peekable(),
            last: None,
            settings: Default::default(),
        }
    }

    /// Collect all [`Token`]s into a [`String`].
    pub fn into_string(self) -> String { self.collect() }

    /// Return a reference to the previous Token.
    pub fn last(&self) -> Option<&Token> { self.last.as_ref() }

    /// Return a reference to the next Token, without advancing the Iterator.
    pub fn peek(&mut self) -> Option<&Token> { self.inner.peek() }

    /// Change the transcription behavior settings.
    pub const fn with_settings(mut self, new: TranscriberSettings) -> Self {
        self.settings = new;
        self
    }
}

impl<T: IntoIterator<Item=Token>> From<T> for TokenIter<T::IntoIter> {
    fn from(iter: T) -> Self { Self::new(iter.into_iter()) }
}

impl<I: Iterator<Item=Token>> Iterator for TokenIter<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: Token = self.inner.next()?;

        if let Token::Glyph(glyph) = &mut token {
            glyph.ligate_zwj = self.settings.ligate_zwj;
            glyph.nuquerna = self.settings.nuquerna;
            glyph.vowels = self.settings.vowels;

            if self.settings.alt_a {
                glyph.set_alt_a();
            }

            match self.inner.peek() {
                Some(Token::Glyph(next)) => {
                    glyph.rince_final = false;
                    glyph.ligate_short = self.settings.ligate_short
                        // && glyph.is_short_carrier()
                        && next.telco_ligates();
                }
                _ => {
                    glyph.rince_final = self.settings.alt_rince;
                    glyph.ligate_short = false;
                }
            }
        }

        self.last = Some(token);
        self.last
    }
}


/// Behavior settings to be used by a [`TokenIter`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TranscriberSettings {
    /// If this is `true`, the [A-tehta](characters::TEHTA_A) will be replaced
    ///     with its [alternate form](characters::TEHTA_YANTA).
    pub alt_a: bool,

    /// If this is `true`, [Sa-Rinci](characters::SA_RINCE) at the ends of words
    ///     will use the [alternate form](characters::SA_RINCE_FINAL) where
    ///     appropriate.
    pub alt_rince: bool,

    /// If this is `true`, the [short carrier](characters::CARRIER_SHORT) will
    ///     be replaced by its [ligating variant](characters::CARRIER_SHORT_LIG)
    ///     where appropriate.
    pub ligate_short: bool,

    /// If this is nonzero, [zero-width joiners](characters::ZWJ) will be placed
    ///     between glyphs to form font ligatures where appropriate.
    pub ligate_zwj: u8,

    /// If this is `true`, the characters [Silmë](characters::TENGWA_SILME) and
    ///     [Essë](characters::TENGWA_ESSE) will use their inverted Nuquernë
    ///     variants when holding a tehta.
    pub nuquerna: bool,

    /// This defines the treatment of "long" vowels.
    pub vowels: VowelStyle,
}

impl TranscriberSettings {
    pub const fn new() -> Self {
        Self {
            alt_a: false,
            alt_rince: false,
            ligate_short: false,
            ligate_zwj: 0,
            nuquerna: false,
            vowels: VowelStyle::DEFAULT,
        }
    }
}

impl Default for TranscriberSettings {
    fn default() -> Self { Self::new() }
}
