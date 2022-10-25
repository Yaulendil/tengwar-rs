pub mod characters;
mod etc;
pub mod mode;

pub use characters::{Glyph, int_10, int_12, ligature_valid, punctuation};
pub use mode::{Beleriand, Gondor, Quenya};
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write},
    iter::{FromIterator, Peekable},
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
/// The ligated counterpart of `transliterate()`.
pub fn transliterate_ligated<R: Rules>(text: impl ToTengwarLigated) -> String {
    text.to_tengwar_ligated::<R>()
}


/// A trait implementing the rules for converting text into the Tengwar.
///
/// The only required method is the one to produce a sequence of Tokens; This
///     can be collected into a `String` easily enough.
pub trait Rules {
    /// Produce a sequence of Tokens representing the Tengwar form of some text.
    fn tokens(input: impl AsRef<str>) -> Vec<Token>;

    /// Produce a sequence of Tokens, and then immediately post-process and
    ///     collect them into a `String`.
    fn transcribe(input: impl AsRef<str>) -> String {
        TokenIter::from(Self::tokens(input)).collect::<String>()
    }

    /// Produce a sequence of Tokens, and then immediately post-process and
    ///     collect them into a `String`. Zero-Width Joiners will be included in
    ///     the output data to form ligatures where appropriate.
    fn transcribe_with_ligatures(input: impl AsRef<str>) -> String {
        TokenIter::from(Self::tokens(input)).ligated().collect::<String>()
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


/// A small container for either plain text or a glyph specification. Serves as
///     the top level of throughput for the transliteration process.
#[derive(Clone, Debug)]
pub enum Token {
    /// A single Unicode codepoint.
    Char(char),
    /// UTF-8 text data.
    String(Cow<'static, str>),
    /// A specified base character and any extra tags it requires.
    Tengwa(Glyph),
    /// A glyph specification, but specifically one that should be ligated, if
    ///     it is appropriate.
    //  TODO: Find a way to do this that sucks less.
    TengwaLigated(Glyph),
}

impl Token {
    /// Mark this Token as one that should, if possible, be ligated.
    pub fn ligated(self) -> Self {
        match self {
            Self::Tengwa(t) => Self::TengwaLigated(t),
            other => other,
        }
    }

    pub fn write(&self, f: &mut Formatter<'_>, ligate: bool) -> fmt::Result {
        match self {
            &Self::Char(ch) => f.write_char(ch),
            Self::String(s) => f.write_str(s),
            Self::Tengwa(t) => t.write(f, ligate),
            Self::TengwaLigated(t) => t.write(f, true),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Char(ch) => f.write_char(ch),
            Self::String(s) => f.write_str(s),
            Self::Tengwa(t) => t.write(f, false),
            Self::TengwaLigated(t) => t.write(f, true),
        }
    }
}

impl FromIterator<Token> for String {
    fn from_iter<T: IntoIterator<Item=Token>>(iter: T) -> Self {
        let mut iter = iter.into_iter().peekable();
        let mut buf = String::new();

        while let Some(token) = iter.next() {
            write!(buf, "{}", token).expect("Error writing String");

            if let Token::TengwaLigated(prev) = token {
                if let Some(Token::TengwaLigated(next)) = iter.peek() {
                    if ligature_valid(&prev, &next) {
                        buf.push(characters::ZWJ);
                    }
                }
            }
        }

        buf
    }
}


struct TokenIter<I: Iterator<Item=Token>> {
    inner: Peekable<I>,
}

impl<I: Iterator<Item=Token>> TokenIter<I> {
    fn ligated(self) -> impl Iterator<Item=Token> {
        self.map(Token::ligated)
    }
}

impl<T: IntoIterator<Item=Token>> From<T> for TokenIter<T::IntoIter> {
    fn from(iter: T) -> Self {
        Self { inner: iter.into_iter().peekable() }
    }
}

impl<I: Iterator<Item=Token>> Iterator for TokenIter<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Option<Token> = self.inner.next();

        if let Some(Token::Tengwa(Glyph { is_final, .. })) = &mut next {
            *is_final = !matches!(self.inner.peek(), Some(Token::Tengwa(..)));
        }

        next
    }
}
