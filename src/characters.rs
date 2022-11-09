//! This file contains the basic constants and data structures required to work
//!     effectively with the Tengwar.
//  TODO: Document EVERY ITEM in this module.

pub mod consts;
pub mod glyph;
pub mod numeral;
pub mod tehta;
pub mod tema;

pub use consts::*;
pub use glyph::*;
pub use numeral::*;
pub use tehta::*;
pub use tema::*;


#[derive(Clone, Copy, Debug)]
pub enum VowelStyle {
    /// Always use the extended carrier mark.
    Separate,
    /// Where possible, use doubled diacritics.
    Doubled,
    /// Where possible, use unique diacritics.
    Unique,
}

impl VowelStyle {
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


/// Convert a tengwa to its inverted variant.
pub const fn nuquerna(c: char) -> char {
    match c {
        TENGWA_SILME => TENGWA_SILME_NUQ,
        TENGWA_ESSE => TENGWA_ESSE_NUQ,
        other => other,
    }
}

/// Check whether a tengwa has an inverted variant.
pub const fn nuquerna_valid(c: char) -> bool {
    c == TENGWA_SILME || c == TENGWA_ESSE
}


/// Check whether a base tengwa is suitable for ligation with the extended
///     carrier mark. This is to some degree based on opinion.
pub const fn ligates_with_ara(base: char) -> bool {
    TEMA_TINCO.single_dn <= base && base <= TENGWA_HWESTA_SINDARINWA
        && base != TENGWA_SILME_NUQ
        && base != TENGWA_ESSE_NUQ
        && base != TENGWA_ESSE
}


/// Check whether a base tengwa is suitable for ligation with the short carrier
///     mark. This is to some degree based on opinion.
pub const fn telco_ligates_with(_base: char) -> bool {
    //  TODO
    true
}


/// Determine whether two `Glyph`s can be joined by a zero-width joiner. These
///     rules are based on the "Tengwar Telcontar" font, and are to some degree
///     based on opinion.
pub const fn ligature_valid(prev: &Glyph, next: &Glyph) -> bool {
    if matches!(prev.base, Some(TENGWA_SILME) | Some(TENGWA_ESSE)) {
        !(prev.tehta.is_some() && next.tehta.is_some()) && match next.base {
            Some(con) => nuquerna_valid(con)
                || (TEMA_TINCO.single_dn <= con && con <= TENGWA_ARDA),
            None => false,
        }
    } else if let (Some(left), Some(right)) = (prev.base, next.base) {
        TEMA_TINCO.single_dn <= left && left <= TENGWA_ARDA
            && TEMA_TINCO.single_dn <= right && right <= TENGWA_ARDA
    } else {
        !(prev.tehta.is_some() && next.tehta.is_some())
    }
}


/// Choose the appropriate form of sa-rincë for a base tengwa.
pub const fn rince(base: char, is_final: bool) -> char {
    if is_final && rince_valid_final(base) {
        SA_RINCE_FINAL
    } else {
        SA_RINCE
    }
}

/// Check whether a base tengwa is suitable to receive a sa-rincë. This is to
///     some degree based on opinion.
pub const fn rince_valid(base: char) -> bool {
    match base {
        TENGWA_ROMEN | TENGWA_ARDA
        | TENGWA_SILME | TENGWA_SILME_NUQ
        | TENGWA_ESSE | TENGWA_ESSE_NUQ => false,
        _ => true,
    }
}

/// Check whether a base tengwa is suitable to receive the alternate sa-rincë.
///     This is to some degree based on opinion.
pub const fn rince_valid_final(base: char) -> bool {
    match base {
        TENGWA_LAMBE | TENGWA_ALDA | TENGWA_HYARMEN => true,
        tengwa if TEMA_TINCO.contains(tengwa) => true,
        tengwa if TEMA_PARMA.contains(tengwa) => true,
        //  NOTE: The left-bow Témar CAN support the alternate, but are written
        //      with the basic form in canonical sources.
        // tengwa if TEMA_CALMA.contains(tengwa) => true,
        // tengwa if TEMA_QESSE.contains(tengwa) => true,
        _ => false,
    }
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
    Irregular(char),
    // Irregular(TengwaIrregular),
    Regular(TengwaRegular<'t>),
}

impl<'t> Tengwa<'t> {
    pub const fn either_from(char: char) -> Self {
        match TengwaRegular::find(char) {
            Some(tengwa) => Self::Regular(tengwa),
            None => Self::Irregular(char),
        }
    }

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

    pub const fn try_regular_from(char: char) -> Option<Self> {
        match TengwaRegular::find(char) {
            Some(tengwa) => Some(Self::Regular(tengwa)),
            None => None,
        }
    }

    pub const fn as_char(&self) -> &char {
        match self {
            Tengwa::Irregular(char) => char,
            // Tengwa::Irregular(char) => &char.0,
            Tengwa::Regular(tengwa) => tengwa.as_char(),
        }
    }

    pub const fn as_irregular(&self) -> Option<&char> {
        match self {
            Tengwa::Irregular(char) => Some(char),
            // Tengwa::Irregular(char) => Some(&char.0),
            Tengwa::Regular(_) => None,
        }
    }

    pub const fn as_regular(&self) -> Option<&TengwaRegular<'t>> {
        match self {
            Tengwa::Irregular(_) => None,
            Tengwa::Regular(tengwa) => Some(tengwa),
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
        // Self::either_from(char)
        Self::irregular_from(char)
    }
}

/*impl<'t> TryFrom<char> for Tengwa<'t> {
    type Error = ();

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Self::try_either_from(char).ok_or(())
    }
}*/
