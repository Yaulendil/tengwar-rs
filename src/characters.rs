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


/// Check whether a tengwa has an inverted variant.
pub const fn can_be_nuquerna(c: char) -> bool {
    c == TENGWA_SILME || c == TENGWA_ESSE
}


/// Convert a tengwa to its inverted variant.
pub const fn nuquerna(c: char) -> char {
    match c {
        TENGWA_SILME => TENGWA_SILME_NUQ,
        TENGWA_ESSE => TENGWA_ESSE_NUQ,
        other => other,
    }
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
    if matches!(prev.cons, Some(TENGWA_SILME) | Some(TENGWA_ESSE)) {
        !(prev.vowel.is_some() && next.vowel.is_some()) && match next.cons {
            Some(con) => can_be_nuquerna(con)
                || (TEMA_TINCO.single_dn <= con && con <= TENGWA_ARDA),
            None => false,
        }
    } else if let (Some(left), Some(right)) = (prev.cons, next.cons) {
        TEMA_TINCO.single_dn <= left && left <= TENGWA_ARDA
            && TEMA_TINCO.single_dn <= right && right <= TENGWA_ARDA
    } else {
        !(prev.vowel.is_some() && next.vowel.is_some())
    }
}


/// Check whether a base tengwa is suitable to receive a sa-rincë. This is to
///     some degree based on opinion.
pub const fn rince_valid(base: char) -> bool {
    !matches!(base, '' | '' | '' | '' | '' | '')
}


/// Choose the appropriate form of sa-rincë for a base tengwa.
#[cfg_attr(not(feature = "alt-rince"), allow(unused_variables))]
pub const fn mod_rince(base: char, is_final: bool) -> char {
    match base {
        #[cfg(feature = "alt-rince")]
        ''..='' | '' | '' | '' if is_final => SA_RINCE_FINAL,
        _ => SA_RINCE,
    }
}
