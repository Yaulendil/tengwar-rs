//! This module defines the basic constants and data structures required to work
//!     effectively with the Tengwar.
//!
//! The core of this module is the [`consts`] submodule, which defines every
//!     codepoint used by the crate. These codepoint definitions are supported
//!     by the [`Tehta`] and [`Tema`] types, and are used to support the
//!     [`Glyph`], [`Numeral`], and [`Tengwa`] types.
//  TODO: Document EVERY ITEM in this module.

// #[cfg_attr(feature = "csur", path = "characters/consts_csur.rs")]
pub mod consts;
pub mod glyph;
pub mod numeral;
pub mod tehta;
pub mod tema;

pub use consts::*;
pub use glyph::Glyph;
pub use numeral::Numeral;
pub use tehta::Tehta;
pub use tema::{Tema, TengwaRegular, Tyelle};


/// A Sa-Rincë is a curl or hook that is attached to a tengwa, and represents a
///     following sibilant sound. There are two forms, depending on the tengwa
///     and its position.
#[derive(Clone, Copy, Debug)]
pub enum Rince {
    /// The basic rincë is a small curl that can be attached to a tengwa in any
    ///     position.
    Basic,
    /// This variant rincë is a larger hook, and can only be applied to the last
    ///     tengwa in a word.
    Final,
}


/// The type of behavior to be followed in the rendering of tehtar representing
///     "long" vowels.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum VowelStyle {
    /// Always use the separate extended carrier mark.
    //  0: No tehta `char`s will ever follow a base tengwa.
    #[value(alias = "s", alias = "0")]
    Separate = 0,
    /// Where possible, write the normal codepoint twice.
    //  2: Up to two tehta `char`s may follow a base tengwa.
    #[value(alias = "d", alias = "2")]
    Doubled = 2,
    /// Where possible, write the dedicated "long" codepoint once.
    //  1: Up to one tehta `char` may follow a base tengwa.
    #[value(alias = "u", alias = "1")]
    Unique = 1,
}

impl VowelStyle {
    /// The default behavior. Everything should use this, if not otherwise
    ///     specified.
    pub const DEFAULT: Self = Self::Doubled;
}

impl Default for VowelStyle {
    fn default() -> Self { Self::DEFAULT }
}


/// Convert non-tengwar punctuation marker into one from the tengwar block.
///     Where unambiguous replacements are not known, this is chosen, admittedly
///     arbitrarily, based on superficial similarity.
///
/// Only single characters are returned; compound punctuation thus may be
///     constructed from similar basic marks, such as `:-` and `::`.
pub const fn punctuation(chr: char) -> Option<char> {
    Some(match chr {
        '\'' | '.' | ',' | '·' => PUNCT_DOT_1,
        ':' | ';' => PUNCT_DOT_2,
        '⁝' | '︙' => PUNCT_DOT_3,
        '⁘' | '⁛' | '…' => PUNCT_DOT_4,
        '⸭' => PUNCT_DOT_5,

        //  NOTE: The Tilde `~` is not converted here because it is used to
        //      denote a specific type of whitespace in LaTeX. Because usage
        //      within LaTeX macros is the primary motivator for the creation of
        //      this program, Tildes are purposefully passed through unaffected.
        '-' => PUNCT_LINE_1,
        '=' => PUNCT_LINE_2,

        '?' => PUNCT_INTERR,
        '!' => PUNCT_EXCLAM,
        '|' | '‖' => PUNCT_PAREN,
        '(' | '[' | '“' => PUNCT_PAREN_L,
        ')' | ']' | '”' | '„' => PUNCT_PAREN_R,

        _ => { return None; }
    })
}

/*
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TengwaIrregular(char);

impl TengwaIrregular {
    // pub const MIN: Self = Self(TENGWA_ROMEN);
    pub const MIN: Self = Self(TENGWA_TINCO);
    pub const MAX: Self = Self(TENGWA_WAIA);

    pub const fn new(char: char) -> Option<Self> {
        if Self::MIN.0 <= char && char <= Self::MAX.0
            // && char != '\u{E02F}' && char != '\u{E033}'
        {
            Some(Self(char))
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(char: char) -> Self { Self(char) }
}

impl std::ops::Deref for TengwaIrregular {
    type Target = char;
    fn deref(&self) -> &Self::Target { &self.0 }
}*/


/// A type representing a single base tengwa, either irregular or regular.
///
/// The `Regular` variant contains a [`TengwaRegular`], which has additional
///     information regarding the actual shape of the character.
#[derive(Clone, Copy, Debug)]
pub enum Tengwa<'t> {
    /// An irregular tengwa that does not follow the rules of the Témar.
    Irregular(char),
    // Irregular(TengwaIrregular),
    /// A [regular tengwa](TengwaRegular) that follows specific formation rules.
    Regular(TengwaRegular<'t>),
}

impl<'t> Tengwa<'t> {
    /// Given an input [`char`], if it maps to a [`TengwaRegular`], define it as
    ///     such; Otherwise, define it as irregular.
    pub const fn either_from(char: char) -> Self {
        match TengwaRegular::find(char) {
            Some(tengwa) => Self::Regular(tengwa),
            None => Self::Irregular(char),
        }
    }

    /// Define a [`char`] as an irregular tengwa.
    pub const fn irregular_from(char: char) -> Self {
        Self::Irregular(char)
    }

    /*pub const unsafe fn either_from_unchecked(char: char) -> Self {
        match TengwaRegular::find(char) {
            Some(tengwa) => Self::Regular(tengwa),
            None => Self::Irregular(TengwaIrregular::new_unchecked(char)),
        }
    }

    pub const fn try_either_from(char: char) -> Option<Self> {
        match TengwaRegular::find(char) {
            Some(tengwa) => Some(Self::Regular(tengwa)),
            None => Self::try_irregular_from(char),
        }
    }

    pub const fn try_irregular_from(char: char) -> Option<Self> {
        match TengwaIrregular::new(char) {
            Some(tengwa) => Some(Self::Irregular(tengwa)),
            None => None,
        }
    }*/

    /// Return the regular tengwa matching a given [`char`], if there is one.
    pub const fn try_regular_from(char: char) -> Option<Self> {
        match TengwaRegular::find(char) {
            Some(tengwa) => Some(Self::Regular(tengwa)),
            None => None,
        }
    }

    /// Return a reference to the [`char`] representing this tengwa.
    pub const fn as_char(&self) -> &char {
        match self {
            Self::Irregular(char) => char,
            // Self::Irregular(char) => &char.0,
            Self::Regular(tengwa) => tengwa.as_char(),
        }
    }

    /// Return a reference to the [`char`] representing this tengwa, if it is
    ///     irregular.
    pub const fn as_irregular(&self) -> Option<&char> {
        match self {
            Self::Irregular(char) => Some(char),
            // Self::Irregular(char) => Some(&char.0),
            Self::Regular(_) => None,
        }
    }

    /// Return a reference to the [`TengwaRegular`] in this tengwa, if it is
    ///     regular.
    pub const fn as_regular(&self) -> Option<&TengwaRegular<'t>> {
        match self {
            Self::Irregular(_) => None,
            Self::Regular(tengwa) => Some(tengwa),
        }
    }

    /// Return `true` if this tengwa is irregular.
    pub const fn is_irregular(&self) -> bool {
        match self {
            Self::Irregular(_) => true,
            Self::Regular(_) => false,
        }
    }

    /// Return `true` if this tengwa is regular.
    pub const fn is_regular(&self) -> bool {
        match self {
            Self::Irregular(_) => false,
            Self::Regular(_) => true,
        }
    }
}

impl<'t> std::ops::Deref for Tengwa<'t> {
    type Target = char;
    fn deref(&self) -> &Self::Target { self.as_char() }
}

impl<'t> From<TengwaRegular<'t>> for Tengwa<'t> {
    fn from(tengwa: TengwaRegular<'t>) -> Self { Self::Regular(tengwa) }
}

impl<'t> From<char> for Tengwa<'t> {
    fn from(char: char) -> Self {
        Self::either_from(char)
        // Self::irregular_from(char)
    }
}

/*impl<'t> TryFrom<char> for Tengwa<'t> {
    type Error = ();

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Self::try_either_from(char).ok_or(())
    }
}*/


#[derive(Clone, Copy, Debug)]
pub enum BaseChar<'t> {
    Carrier(bool),
    Tengwa(Tengwa<'t>),
}

impl<'t> BaseChar<'t> {
    pub const fn to_char(&self) -> char {
        match self {
            Self::Carrier(long) => carrier(*long),
            Self::Tengwa(tengwa) => *tengwa.as_char(),
        }
    }

    pub const fn tengwa(&self) -> Option<&Tengwa<'t>> {
        match self {
            Self::Carrier(_) => None,
            Self::Tengwa(tengwa) => Some(tengwa),
        }
    }
}

impl<'t> From<char> for BaseChar<'t> {
    fn from(char: char) -> Self {
        match char {
            CARRIER_LONG => Self::Carrier(true),
            CARRIER_SHORT => Self::Carrier(false),
            CARRIER_SHORT_LIG => Self::Carrier(false),
            c => Self::Tengwa(c.into()),
        }
    }
}

impl<'t> From<Tengwa<'t>> for BaseChar<'t> {
    fn from(tengwa: Tengwa<'t>) -> Self { Self::Tengwa(tengwa) }
}

impl<'t> From<TengwaRegular<'t>> for BaseChar<'t> {
    fn from(tengwa: TengwaRegular<'t>) -> Self { Self::Tengwa(tengwa.into()) }
}


#[test]
#[cfg(test)]
fn report_sizes() {
    use std::mem::size_of;

    eprintln!("Glyph helpers:");
    dbg!(
        size_of::<glyph::Parts>(),
        size_of::<glyph::TehtaChar>(),
        size_of::<glyph::TengwaTehta>(),
    );

    eprintln!("Tengwa helpers:");
    dbg!(
        size_of::<Tema>(),
        size_of::<&Tema>(),
        size_of::<Tyelle>(),
        size_of::<TengwaRegular>(),
        size_of::<Tengwa>(),
        size_of::<BaseChar>(),
    );

    eprintln!("Main:");
    dbg!(
        size_of::<VowelStyle>(),
        size_of::<Glyph>(),
        size_of::<Numeral>(),
        size_of::<crate::Token>(),
    );

    eprintln!();
}
