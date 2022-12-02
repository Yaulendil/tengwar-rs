use std::fmt::{Display, Formatter, Write};
use crate::{
    characters::{Glyph, Numeral, ZWJ},
    policy::{Policy, Standard},
};


/// A small container for either plain text or a [`Glyph`] specification. Serves
///     as the top level of throughput for the transliteration process.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Token<P: Policy = Standard> {
    /// A single Unicode codepoint.
    Char(char),
    /// A specified base character and any extra tags it requires.
    Glyph(Glyph<P>),
    /// A numeric value.
    Number(Numeral),
    // /// UTF-8 text data.
    // String(Cow<'static, str>),
}

impl<P: Policy> Token<P> {
    /// Switch any [`Glyph`] in the token to use a different [`Policy`].
    pub const fn change_policy<Q: Policy>(self) -> Token<Q> {
        match self {
            Self::Glyph(glyph) => Token::Glyph(glyph.change_policy()),
            Self::Char(char) => Token::Char(char),
            Self::Number(number) => Token::Number(number),
        }
    }

    /// Return a reference to the [`Glyph`], if there is one.
    pub const fn glyph(&self) -> Option<&Glyph<P>> {
        match self {
            Self::Char(_) => None,
            Self::Glyph(g) => Some(g),
            Self::Number(_) => None,
        }
    }

    /// Return a reference to the [`Numeral`], if there is one.
    pub const fn numeral(&self) -> Option<&Numeral> {
        match self {
            Self::Char(_) => None,
            Self::Glyph(_) => None,
            Self::Number(n) => Some(n),
        }
    }
}

impl<P: Policy> Display for Token<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(ch) => f.write_char(*ch),
            Self::Glyph(g) => g.fmt(f),
            Self::Number(n) => n.fmt(f),
            // Self::String(s) => f.write_str(s),
        }
    }
}


impl<P: Policy> FromIterator<Token<P>> for String {
    fn from_iter<I: IntoIterator<Item=Token<P>>>(iter: I) -> Self {
        let mut iter = iter.into_iter().peekable();
        let mut buf = String::new();

        while let Some(token) = iter.next() {
            write!(buf, "{token}").expect("Error writing String");

            if let Token::Glyph(current) = token {
                if let Some(Token::Glyph(next)) = iter.peek() {
                    if current.ligates_with(next) {
                        buf.push(ZWJ);
                    }
                }
            }
        }

        buf
    }
}
