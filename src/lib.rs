pub mod beleriand;
pub mod characters;
pub mod etc;
pub mod quenya;
pub mod sindarin;

pub use beleriand::Beleriand;
pub use characters::{Glyph, int_10, int_12, ligature_valid, punctuation};
pub use quenya::Quenya;
pub use sindarin::Sindarin;
use std::{
    borrow::Cow,
    fmt::{self, Write},
    iter::{FromIterator, Peekable},
};


pub trait Rules {
    fn tokens(input: impl AsRef<str>) -> Vec<Token>;

    fn transcribe(input: impl AsRef<str>) -> String {
        TokenIter::from(Self::tokens(input)).collect::<String>()
    }

    fn transcribe_with_ligatures(input: impl AsRef<str>) -> String {
        TokenIter::from(Self::tokens(input)).ligated().collect::<String>()
    }
}


pub trait ToTengwar {
    fn to_tengwar<R: Rules>(&self) -> String;
    fn to_tengwar_ligated<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwar for T {
    /// Transliterate this text into the Tengwar.
    fn to_tengwar<R: Rules>(&self) -> String {
        R::transcribe(self)
    }

    /// Transliterate this text into the Tengwar. A post-processor will run over
    ///     it to insert zero-width joiners and create ligatures where possible.
    ///     This affects the text data itself, but should not have any visible
    ///     effect with a font that does not support the ligatures.
    fn to_tengwar_ligated<R: Rules>(&self) -> String {
        R::transcribe_with_ligatures(self)
    }
}


pub enum Token {
    Char(char),
    String(Cow<'static, str>),
    Tengwa(Glyph),
    //  TODO: Find a way to do this that sucks less.
    TengwaLigated(Glyph),
}


impl Token {
    fn ligated(self) -> Self {
        match self {
            Self::Tengwa(t) => Self::TengwaLigated(t),
            other => other,
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(chr) => f.write_char(*chr),
            Self::String(s) => f.write_str(&s),
            Self::Tengwa(t) => t.write(f, false),
            Self::TengwaLigated(t) => t.write(f, true),
        }
    }
}


impl FromIterator<Token> for String {
    fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item=Token>,
            T::IntoIter: Iterator<Item=Token>,
    {
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
    pub inner: Peekable<I>,
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
