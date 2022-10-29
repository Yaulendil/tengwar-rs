//! This file contains the basic constants and data structures required to work
//!     effectively with the Tengwar.
//  TODO: Document EVERY ITEM in this file.

use std::fmt::{self, Display, Formatter, Write};


/// Ára, a carrier marking for a "long" vowel.
pub const CARRIER_LONG: char = '';
/// Telco, a carrier marking for a "short" vowel.
pub const CARRIER_SHORT: char = '';
/// A ligating variant of [Telco](CARRIER_SHORT).
pub const CARRIER_SHORT_LIG: char = '';


/// One acute accent above a tengwa.
pub const DC_OVER_ACUTE_1: char = '';
/// Two acute accents above a tengwa.
pub const DC_OVER_ACUTE_2: char = '';

/// One dot above a tengwa.
pub const DC_OVER_DOT_1: char = '';
/// Two dots above a tengwa, arranged horizontally.
pub const DC_OVER_DOT_2: char = '';
/// Three dots above a tengwa, with one above the others.
pub const DC_OVER_DOT_3: char = '';
/// Three dots above a tengwa, with one below the others.
pub const DC_OVER_DOT_3_INV: char = '';

/// A line above a tengwa, which curls to the left.
pub const DC_OVER_HOOK_L_1: char = '';
/// Two left-curling lines above a tengwa.
pub const DC_OVER_HOOK_L_2: char = '';

/// A line above a tengwa, which curls to the right.
pub const DC_OVER_HOOK_R_1: char = '';
/// Two right-curling lines above a tengwa.
pub const DC_OVER_HOOK_R_2: char = '';

/// A straight horizontal overline.
pub const DC_OVER_LINE: char = '';
/// A wavy horizontal overline, similar to a tilde.
pub const DC_OVER_WAVE: char = '';

/// A diacritic similar to a breve.
pub const DC_OVER_BREVE: char = '';
/// One grave accent above a tengwa.
pub const DC_OVER_GRAVE: char = '';
/// A diacritic similar to a circumflex.
pub const DC_OVER_CIRCUMFLEX: char = '';


/// One inverted acute accent below a tengwa.
pub const DC_UNDER_ACUTE_1: char = '';
/// Two inverted acute accents below a tengwa.
pub const DC_UNDER_ACUTE_2: char = '';

/// One dot below a tengwa.
pub const DC_UNDER_DOT_1: char = '';
/// Two dots below a tengwa, arranged horizontally.
pub const DC_UNDER_DOT_2: char = '';
/// Three dots below a tengwa, with one below the others.
pub const DC_UNDER_DOT_3: char = '';

/// A line below a tengwa, which curls to the left.
pub const DC_UNDER_HOOK_L_1: char = '';
/// A line below a tengwa, which curls to the right.
pub const DC_UNDER_HOOK_R_1: char = '';
/// A horizontal underline.
pub const DC_UNDER_LINE_H: char = '';
/// A vertical line below a tengwa. Has various meanings but usually pertains to
///     the vowel marking above it.
pub const DC_UNDER_LINE_V: char = '';

/// An unfilled circle below a tengwa. Most often used to denote the least
///     significant digit in a duodecimal figure.
pub const DC_UNDER_RING: char = '';


/// Marking to denote a sound that leads into a "w" sound.
pub const MOD_LABIAL: char = DC_OVER_WAVE;
/// Long/double consonant.
pub const MOD_LONG_CONS: char = DC_UNDER_LINE_H;
/// Long vowel after consonant.
pub const MOD_LONG_VOWEL: char = DC_UNDER_LINE_V;

/// Marking to denote a sound preceded by M or N.
pub const MOD_NASAL: char = DC_OVER_LINE;
/// Marking to denote a sound that leads into a "y" sound.
pub const MOD_PALATAL: char = DC_UNDER_DOT_2;
/// A small hook added to a tengwa to indicate a following sibilant.
pub const MOD_SA_RINCE: char = '';
/// An ornate flourish added to a tengwa to indicate a following sibilant.
pub const MOD_SA_RINCE_FINAL: char = '';

pub mod numeral {
    pub const NUM_0: char = '';
    pub const NUM_1: char = '';
    pub const NUM_2: char = '';
    pub const NUM_3: char = '';
    pub const NUM_4: char = '';
    pub const NUM_5: char = '';
    pub const NUM_6: char = '';
    pub const NUM_7: char = '';
    pub const NUM_8: char = '';
    pub const NUM_9: char = '';
    pub const NUM_A: char = '';
    pub const NUM_B: char = '';
    pub const NUM_C: char = '';
}

/// Index in this array corresponds to the numerical value of the digit.
pub const NUMERAL: [char; 13] = [
    numeral::NUM_0,
    numeral::NUM_1, numeral::NUM_2, numeral::NUM_3, numeral::NUM_4,
    numeral::NUM_5, numeral::NUM_6, numeral::NUM_7, numeral::NUM_8,
    numeral::NUM_9, numeral::NUM_A, numeral::NUM_B, numeral::NUM_C,
];


/// A single dot positioned inside the preceding character.
pub const PUNCT_DOT_0: char = '';
/// One dot, at middle height.
pub const PUNCT_DOT_1: char = '';
/// Two dots, resembling an ASCII colon.
pub const PUNCT_DOT_2: char = '';
/// Three dots, arranged vertically.
pub const PUNCT_DOT_3: char = '';
/// Four dots in a diamond configuration.
pub const PUNCT_DOT_4: char = '';
/// Five dots in a plus-shape.
pub const PUNCT_DOT_5: char = '';

/// A wavy vertical line, used to express strong feeling.
pub const PUNCT_EXCLAM: char = '';
/// A variant exclamatory marking, used in the contract given to Bilbo Baggins
///     by Thorin Oakenshield.
pub const PUNCT_THORIN: char = '';
/// A symbol resembling a capital `B`, serving as an interrogative marking.
pub const PUNCT_INTERR: char = '';

/// A single horizontal line with a wave, resembling a tilde.
pub const PUNCT_LINE_1: char = '';
/// Two horizontal waved lines, stacked vertically.
pub const PUNCT_LINE_2: char = '';

/// An ornate vertical bar, used as a parenthesis (both opening and closing).
pub const PUNCT_PAREN: char = '';
/// The opening of a pair of parenthetical markings, used to indicate that the
///     text enclosed uses a different mode of the Tengwar.
pub const PUNCT_PAREN_L: char = '';
/// The closing of a pair of parenthetical markings, used to indicate that the
///     text enclosed uses a different mode of the Tengwar.
pub const PUNCT_PAREN_R: char = '';

/// A compound marking most often denoting the end of a passage.
pub const PUNCT_EOF: &str = "";


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


/// A diacritical marking resembling a circumflex.
pub const TEHTA_CIRCUMFLEX: Tehta = Tehta::Single(DC_OVER_CIRCUMFLEX);


/// The diacritic for an `A` vowel, in its standard three-dot form.
#[cfg(not(feature = "circumflex"))]
const _A: char = DC_OVER_DOT_3;


/// The diacritic for an `A` vowel, in its alternate circumflex-like form.
#[cfg(feature = "circumflex")]
const _A: char = DC_OVER_CIRCUMFLEX;


#[cfg(not(any(feature = "long-vowel-double", feature = "long-vowel-unique")))]
mod _vowels {
    use super::*;

    /// A diacritical Tehta used in most systems to represent the `A` vowel.
    pub const TEHTA_A: Tehta = Tehta::Single(_A);
    /// A diacritical Tehta used in most systems to represent the `E` vowel.
    pub const TEHTA_E: Tehta = Tehta::Single(DC_OVER_ACUTE_1);
    /// A diacritical Tehta used in most systems to represent the `I` vowel.
    pub const TEHTA_I: Tehta = Tehta::Single(DC_OVER_DOT_1);
    /// A diacritical Tehta used in most systems to represent the `O` vowel.
    pub const TEHTA_O: Tehta = Tehta::Single(DC_OVER_HOOK_R_1);
    /// A diacritical Tehta used in most systems to represent the `U` vowel.
    pub const TEHTA_U: Tehta = Tehta::Single(DC_OVER_HOOK_L_1);
    /// A diacritical Tehta used in the Sindarin modes to represent the `Y` vowel.
    pub const TEHTA_Y: Tehta = Tehta::Single(DC_OVER_DOT_2);
}


#[cfg(all(feature = "long-vowel-double", not(feature = "long-vowel-unique")))]
mod _vowels {
    use super::*;

    /// A diacritical Tehta used in most systems to represent the `A` vowel.
    pub const TEHTA_A: Tehta = Tehta::Single(_A);
    /// A diacritical Tehta used in most systems to represent the `E` vowel.
    pub const TEHTA_E: Tehta = Tehta::Double(DC_OVER_ACUTE_1);
    /// A diacritical Tehta used in most systems to represent the `I` vowel.
    pub const TEHTA_I: Tehta = Tehta::Single(DC_OVER_DOT_1);
    /// A diacritical Tehta used in most systems to represent the `O` vowel.
    pub const TEHTA_O: Tehta = Tehta::Double(DC_OVER_HOOK_R_1);
    /// A diacritical Tehta used in most systems to represent the `U` vowel.
    pub const TEHTA_U: Tehta = Tehta::Double(DC_OVER_HOOK_L_1);
    /// A diacritical Tehta used in the Sindarin modes to represent the `Y` vowel.
    pub const TEHTA_Y: Tehta = Tehta::Single(DC_OVER_DOT_2);
}


#[cfg(feature = "long-vowel-unique")]
mod _vowels {
    use super::*;

    /// A diacritical Tehta used in most systems to represent the `A` vowel.
    pub const TEHTA_A: Tehta = Tehta::Single(_A);
    /// A diacritical Tehta used in most systems to represent the `E` vowel.
    pub const TEHTA_E: Tehta = Tehta::Altern(DC_OVER_ACUTE_1, DC_OVER_ACUTE_2);
    /// A diacritical Tehta used in most systems to represent the `I` vowel.
    pub const TEHTA_I: Tehta = Tehta::Single(DC_OVER_DOT_1);
    /// A diacritical Tehta used in most systems to represent the `O` vowel.
    pub const TEHTA_O: Tehta = Tehta::Altern(DC_OVER_HOOK_R_1, DC_OVER_HOOK_R_2);
    /// A diacritical Tehta used in most systems to represent the `U` vowel.
    pub const TEHTA_U: Tehta = Tehta::Altern(DC_OVER_HOOK_L_1, DC_OVER_HOOK_L_2);
    /// A diacritical Tehta used in the Sindarin modes to represent the `Y` vowel.
    pub const TEHTA_Y: Tehta = Tehta::Single(DC_OVER_DOT_2);
}


pub use _vowels::*;


/// The T-series, with an open bow to the right.
pub const TEMA_TINCO: Tema = Tema {
    single_dn: '', // Tinco
    double_dn: '', // Ando
    single_up: '', // Thúlë
    double_up: '', // Anto
    double_sh: '', // Númen
    single_sh: '', // Órë
    single_ex: '',
    double_ex: '',
};
/// The P-series, with a closed bow to the right.
pub const TEMA_PARMA: Tema = Tema {
    single_dn: '', // Parma
    double_dn: '', // Umbar
    single_up: '', // Formen
    double_up: '', // Ampa
    double_sh: '', // Malta
    single_sh: '', // Vala
    single_ex: '',
    double_ex: '',
};
/// The C-series, with an open bow to the left.
pub const TEMA_CALMA: Tema = Tema {
    single_dn: '', // Calma
    double_dn: '', // Anga
    single_up: '', // Aha
    double_up: '', // Anca
    double_sh: '', // Ñoldo
    single_sh: '', // Anna
    single_ex: '',
    double_ex: '',
};
/// The Q-series, with a closed bow to the left.
pub const TEMA_QESSE: Tema = Tema {
    single_dn: '', // Qessë
    double_dn: '', // Ungwë
    single_up: '', // Hwesta
    double_up: '', // Unquë
    double_sh: '', // Ñwalmë
    single_sh: '', // Wilya
    single_ex: '',
    double_ex: '',
};


//  TODO: Is there any actual benefit to explicitly naming ALL of these?
pub mod temar {
    use super::{TEMA_TINCO, TEMA_PARMA, TEMA_CALMA, TEMA_QESSE};

    pub const TINCO: char = TEMA_TINCO.single_dn;
    pub const ANDO: char = TEMA_TINCO.double_dn;
    pub const THULE: char = TEMA_TINCO.single_up;
    pub const ANTO: char = TEMA_TINCO.double_up;
    pub const NUMEN: char = TEMA_TINCO.double_sh;
    pub const ORE: char = TEMA_TINCO.single_sh;

    pub const PARMA: char = TEMA_PARMA.single_dn;
    pub const UMBAR: char = TEMA_PARMA.double_dn;
    pub const FORMEN: char = TEMA_PARMA.single_up;
    pub const AMPA: char = TEMA_PARMA.double_up;
    pub const MALTA: char = TEMA_PARMA.double_sh;
    pub const VALA: char = TEMA_PARMA.single_sh;

    pub const CALMA: char = TEMA_CALMA.single_dn;
    pub const ANGA: char = TEMA_CALMA.double_dn;
    pub const AHA: char = TEMA_CALMA.single_up;
    pub const ANCA: char = TEMA_CALMA.double_up;
    pub const NOLDO: char = TEMA_CALMA.double_sh;
    pub const ANNA: char = TEMA_CALMA.single_sh;

    pub const QESSE: char = TEMA_QESSE.single_dn;
    pub const UNGWE: char = TEMA_QESSE.double_dn;
    pub const HWESTA: char = TEMA_QESSE.single_up;
    pub const UNQUE: char = TEMA_QESSE.double_up;
    pub const NWALME: char = TEMA_QESSE.double_sh;
    pub const WILYA: char = TEMA_QESSE.single_sh;
}

/// A variant of Vala with an extra hook.
pub const TENGWA_CURL_SINGLE: char = '';
/// A variant of Malta with an extra hook.
pub const TENGWA_CURL_DOUBLE: char = '';

pub const TENGWA_ROMEN: char = '';
pub const TENGWA_ARDA: char = '';
pub const TENGWA_LAMBE: char = '';
pub const TENGWA_ALDA: char = '';

pub const TENGWA_SILME: char = '';
pub const TENGWA_SILME_NUQ: char = '';
/// Originally "ázë", for `z`, later for `ss`.
pub const TENGWA_ESSE: char = '';
pub const TENGWA_ESSE_NUQ: char = '';

pub const TENGWA_HYARMEN: char = '';
pub const TENGWA_HWESTA_SINDARINWA: char = '';
/// Carrier Tengwa for '_i' Diphthongs.
pub const TENGWA_YANTA: char = '';
/// Carrier Tengwa for '_u' Diphthongs.
pub const TENGWA_URE: char = '';

pub const TENGWA_HALLA: char = '';
pub const TENGWA_WAIA: char = '';
pub const TENGWA_OSSE: char = '';
pub const TENGWA_OSSE_REV: char = '';

pub const TENGWA_ANNA_OPEN: char = '';
pub const TENGWA_BOMBADIL_HW: char = '';
pub const TENGWA_BOMBADIL_W: char = '';
pub const TENGWA_LOWDHAM_HW: char = '';

pub const TENGWA_CHRISTOPHER_QU: char = '';

/// "Zero-Width Joiner", used for forming ligatures.
pub const ZWJ: char = '‍';


/// Return a carrier mark appropriate for whether a vowel is long.
pub const fn carrier(long: bool) -> char {
    if long { CARRIER_LONG } else { CARRIER_SHORT }
}


fn int(mut n: isize, base: isize) -> (bool, Vec<usize>) {
    if n == 0 {
        return (false, vec![0]);
    }

    let mut digits = Vec::new();
    let neg = n.is_negative();

    while n != 0 {
        digits.push((n % base).unsigned_abs());
        n /= base;
    }

    (neg, digits)
}


//  TODO: Either figure out what to do about floats or drop the generic.
#[derive(Clone, Copy, Debug)]
pub struct Numeral<N> {
    /// Numeric value.
    pub value: N,

    /// Whether the number will be displayed in Decimal, base 10, rather than in
    ///     Duodecimal, base 12.
    pub decimal: bool,
    /// Whether the number is ordinal ("first"), rather than cardinal ("one").
    pub ordinal: bool,

    /// Whether the base of the number will be denoted with lines, rather than
    ///     with dots.
    pub lines: bool,
}

impl<N> Numeral<N> {
    pub const PREFIX_NEGATIVE: char = '-';

    pub const fn new(value: N, decimal: bool) -> Self {
        Self {
            value,
            decimal,
            ordinal: false,
            lines: false,
        }
    }

    pub const fn decimal(value: N) -> Self {
        Self::new(value, true)
    }

    pub const fn duodecimal(value: N) -> Self {
        Self::new(value, false)
    }

    pub const fn with_decimal(mut self, decimal: bool) -> Self {
        self.decimal = decimal;
        self
    }

    pub const fn with_lines(mut self, lines: bool) -> Self {
        self.lines = lines;
        self
    }

    pub const fn with_ordinal(mut self, ordinal: bool) -> Self {
        self.ordinal = ordinal;
        self
    }
}

impl Numeral<isize> {
    //noinspection RsBorrowChecker
    pub fn render(&self) -> String {
        let negative: bool;
        let digits: Vec<usize>;
        let size: usize;

        let base_marker: char;
        let mark_ones: bool;

        if self.decimal {
            //  Base-10 number.
            (negative, digits) = int(self.value, 10);
            size = negative as usize + digits.len() * 6 + 3;

            if self.lines {
                base_marker = DC_OVER_LINE;
                mark_ones = true;
            } else {
                base_marker = DC_OVER_DOT_1;
                mark_ones = true;
            }
        } else {
            //  Base-12 number.
            (negative, digits) = int(self.value, 12);
            size = negative as usize + digits.len() * 6;

            if self.lines {
                base_marker = DC_UNDER_LINE_H;
                mark_ones = true;
            } else {
                base_marker = DC_UNDER_DOT_1;
                mark_ones = false;
            }
        }

        let mut text = String::with_capacity(size);

        if negative {
            text.push(Self::PREFIX_NEGATIVE);
        }

        match digits.as_slice() {
            [] => {}
            /*[0, 1] if !self.decimal => {
                //  TODO
                text.push(NUMERAL[12]);
                text.push(base_marker);
            }*/
            [digit] => {
                text.push(NUMERAL[*digit]);
                text.push(base_marker);
            }
            [first, digits @ ..] => {
                text.push(NUMERAL[*first]);
                text.push(DC_UNDER_RING);

                if mark_ones {
                    text.push(base_marker);
                }

                for digit in digits {
                    text.push(NUMERAL[*digit]);
                    text.push(base_marker);
                }
            }
        }

        if self.ordinal {
            text.push(TEMA_TINCO.single_ex);
        }

        text
    }
}

impl<N> From<N> for Numeral<N> {
    fn from(value: N) -> Self {
        Self::new(value, false)
    }
}


pub const fn width(c: char) -> Option<usize> {
    match c {
        DC_OVER_DOT_3..=DC_UNDER_LINE_V
        | MOD_SA_RINCE..=PUNCT_DOT_0
        | DC_UNDER_RING
        | ZWJ
        => Some(0),

        temar::TINCO..=CARRIER_SHORT
        | TENGWA_OSSE_REV..=TENGWA_OSSE
        | CARRIER_SHORT_LIG..=TENGWA_WAIA
        | MOD_SA_RINCE_FINAL
        | PUNCT_DOT_1..=PUNCT_THORIN
        | numeral::NUM_0..=numeral::NUM_C
        => Some(1),

        _ => None,
    }
}


/// A diacritical vowel marker that may be rendered in an alternate "long" form.
#[derive(Clone, Copy, Debug)]
pub enum Tehta {
    /// There is only one form that this diacritic can take. The long form must
    ///     be rendered with an extended carrier mark.
    Single(char),
    /// There is one form that this diacritic can take, but it may be repeated
    ///     for the long form.
    Double(char),
    /// This diacritic has an alternate character to represent its long form.
    Altern(char, char),
}

impl Tehta {
    /// Write this Tehta into a Formatter. The provided boolean argument will
    ///     determine whether the basic short form or the variable long form is
    ///     written.
    pub fn write(&self, f: &mut Formatter<'_>, long: bool) -> fmt::Result {
        match self {
            &Self::Single(mark) => {
                if long {
                    f.write_char(carrier(true))?;
                }

                f.write_char(mark)
            }
            &Self::Double(mark) => {
                if long {
                    f.write_char(mark)?;
                }

                f.write_char(mark)
            }
            &Self::Altern(mark, alt) => {
                if long {
                    f.write_char(alt)
                } else {
                    f.write_char(mark)
                }
            }
        }
    }

    /// Returns true if the long variant of this Tehta would be written with the
    ///     extended "Ára" Telco.
    pub const fn uses_ara(&self) -> bool {
        matches!(self, Self::Single(..))
    }
}


/// The Témar are the four series of the primary tengwar. Each Téma is composed
///     of eight Tyeller, each modifying the tengwa in a different way, and is
///     named after its base tengwa.
///
/// Only the first six Tyeller are used in Quenya.
#[derive(Clone, Copy, Debug)]
pub struct Tema {
    /// A hanging Telco, and one Lúva.
    ///     Typically represents a voiceless plosive.
    pub single_dn: char,
    /// A hanging Telco, and two Lúvar.
    ///     Typically represents a voiced plosive.
    pub double_dn: char,
    /// A raised Telco, and one Lúva.
    ///     Typically represents a voiceless fricative.
    pub single_up: char,
    /// A raised Telco, and two Lúvar.
    ///     Typically represents either a voiced fricative or a nasalized
    ///     voiceless plosive.
    pub double_up: char,
    /// A short Telco, and two Lúvar.
    ///     Typically represents a nasal long.
    pub double_sh: char,
    /// A short Telco, and one Lúva.
    ///     Typically represents a nasal short.
    pub single_sh: char,
    /// An extended Telco, and one Lúva.
    ///     Not used in canonical sources.
    pub single_ex: char,
    /// An extended Telco, and two Lúvar.
    ///     Not used in canonical sources.
    pub double_ex: char,
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
    (TEMA_TINCO.single_dn <= base && base <= TENGWA_HWESTA_SINDARINWA)
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
        ''..='' | '' | '' | '' if is_final => MOD_SA_RINCE_FINAL,
        _ => MOD_SA_RINCE,
    }
}


/// A single base tengwa, and all of its modifications. This includes the vowel
///     marking, flags for additional diacritics, flags for consonant and vowel
///     length, and an indicator of finality.
#[derive(Clone, Copy, Debug, Default)]
pub struct Glyph {
    /// A consonant character.
    pub cons: Option<char>,
    /// A diacritical vowel to modify the consonant.
    pub vowel: Option<Tehta>,
    /// If Silmë follows another tengwa, the base character may be modified by
    ///     a sa-rincë instead.
    pub silme: bool,
    /// A nasalized consonant is typically represented by an overbar.
    pub nasal: bool,
    /// A labialized consonant is represented by an additional diacritic.
    pub labial: bool,
    /// A palatalized vowel is represented by an additional diacritic.
    pub palatal: bool,
    /// A lengthened consonant is typically represented by an underbar.
    pub long_cons: bool,
    /// A lengthened vowel can be represented in various ways.
    pub long_vowel: bool,
    /// Indicates whether a long vowel using the extended "Ára" Telco should be
    ///     placed before this glyph.
    pub long_first: bool,

    /// Indicates that this glyph should use the [ligating short carrier], if it
    ///     is applicable.
    ///
    /// [ligating short carrier]: CARRIER_SHORT_LIG
    pub ligate_short: bool,

    /// Indicates whether this glyph should try to use [`ZWJ`] ligation if it
    ///     needs to output a separate carrier. This does NOT cause the glyph to
    ///     try to ligate with the NEXT glyph.
    pub ligate_zwj: bool,

    /// This glyph is the final one in a word, and may use a more ornate rincë.
    pub is_final: bool,
}

impl Glyph {
    /// Define a new empty glyph.
    pub const fn new() -> Self {
        Self {
            cons: None,
            vowel: None,
            silme: false,
            nasal: false,
            labial: false,
            palatal: false,
            long_cons: false,
            long_vowel: false,
            long_first: false,
            ligate_short: false,
            ligate_zwj: false,
            is_final: false,
        }
    }

    /// Define a glyph with both a base character and a diacritical `Tehta`.
    pub const fn new_both(cons: char, vowel: Tehta) -> Self {
        Self {
            cons: Some(cons),
            vowel: Some(vowel),
            ..Self::new()
        }
    }

    /// Define a glyph with only a base character. It may be marked as Long.
    pub const fn new_cons(cons: char, long: bool) -> Self {
        Self {
            cons: Some(cons),
            long_cons: long,
            ..Self::new()
        }
    }

    /// Define a glyph with only a diacritical Tehta. It may be marked as Long.
    pub const fn new_vowel(vowel: Tehta, long: bool) -> Self {
        Self {
            vowel: Some(vowel),
            long_vowel: long,
            ..Self::new()
        }
    }

    pub const fn is_short_carrier(&self) -> bool {
        match self {
            Self { cons: None, vowel: None, .. } => true,
            Self { cons: None, long_vowel: false, .. } => true,
            Self { .. } => false,
        }
    }

    /// Mark this glyph as being labialized. It will be rendered with a wavy
    ///     overbar.
    pub const fn with_labial(mut self) -> Self {
        self.labial = true;
        self
    }

    /// Mark this glyph as being nasalized. It will be rendered overlined.
    pub const fn with_nasal(mut self) -> Self {
        self.nasal = true;
        self
    }

    /// Mark this glyph as being palatalized. It will be rendered with a pair of
    ///     dots below it.
    pub const fn with_palatal(mut self) -> Self {
        self.palatal = true;
        self
    }

    /// Mark this glyph as being followed by a sibilant. It may be rendered with
    ///     a flourish.
    pub const fn with_silme(mut self) -> Self {
        self.silme = true;
        self
    }
}

impl Glyph {
    /// Determine the base character to be used for this glyph. If one is not
    ///     set, an appropriate "carrier" mark will be returned instead.
    pub const fn base(&self) -> char {
        match self {
            #[cfg(feature = "nuquernar")]
            &Glyph { cons: Some(con), vowel: Some(ref vowel), long_vowel, .. } if {
                can_be_nuquerna(con) && !(long_vowel && vowel.uses_ara())
            } => nuquerna(con),

            &Glyph { cons: Some(con), .. } => con,
            /*// &Glyph { long_vowel: true, .. } => carrier(true),
            // &Glyph { ligate_short: true, .. } => CARRIER_SHORT_LIG,
            &Glyph { long_vowel, .. } => carrier(long_vowel),*/

            &Glyph { long_vowel, ligate_short, .. } => {
                if long_vowel {
                    CARRIER_LONG
                } else if ligate_short {
                    CARRIER_SHORT_LIG
                } else {
                    CARRIER_SHORT
                }
            }
        }
    }

    pub const fn ligates_with(&self, other: &Self) -> bool {
        ligature_valid(self, other)
    }

    pub const fn telco_ligates(&self) -> bool {
        telco_ligates_with(self.base())
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let base: char = self.base();
        let Glyph {
            cons, vowel, silme,
            nasal, labial, palatal,
            long_cons, long_vowel, long_first,
            ligate_zwj, is_final,
            ..
        } = self;

        #[cfg_attr(feature = "nuquernar", allow(unused_mut))]
        let mut long: bool = *long_vowel && cons.is_some();
        let nuquerna_ignored: bool = !cfg!(feature = "nuquernar")
            && can_be_nuquerna(base);

        let vowel_post: Option<&Tehta> = match vowel {
            Some(tehta) if long && *long_first => {
                //  This tehta is a long vowel, and represents the preceding
                //      vowel.
                if tehta.uses_ara() {
                    //  This tehta will go on a long carrier. It therefore needs
                    //      to be written before the base.
                    tehta.write(f, true)?;
                    None
                } else if nuquerna_ignored {
                    //  This tengwa will not be be Nuquerna, but it could have
                    //      been. This means that it cannot host a lengthened
                    //      tehta, and so the vowel should be pushed onto a
                    //      preceding long carrier.
                    f.write_char(carrier(true))?;
                    tehta.write(f, false)?;
                    None
                } else {
                    //  This tehta will be displayed on the tengwa, and so must
                    //      still be written after the base character.
                    Some(tehta)
                }
            }
            Some(tehta) => Some(tehta),
            None => None,
        };

        f.write_char(base)?;

        if *nasal {
            f.write_char(MOD_NASAL)?;
        }

        if *long_cons {
            f.write_char(MOD_LONG_CONS)?;
        }

        if *labial {
            f.write_char(MOD_LABIAL)?;
        }

        if *palatal {
            f.write_char(MOD_PALATAL)?;
        }

        if let Some(vowel) = vowel_post {
            if long {
                if vowel.uses_ara() {
                    //  The vowel tehta will be placed on a following Ára
                    //      carrier. If the base should ligate with Ára, write
                    //      the joiner now.
                    if *ligate_zwj && ligates_with_ara(base) {
                        f.write_char(ZWJ)?;
                    }
                } else if nuquerna_ignored {
                    //  This tengwa has a Nuquerna variant, but it will not be
                    //      used. However, it also has a long vowel attached,
                    //      which, without intervention, will use a more complex
                    //      diacritic. The long vowel should be put on an Ára
                    //      carrier instead to decrease visual chaos.
                    if *ligate_zwj && ligates_with_ara(base) {
                        f.write_char(ZWJ)?;
                    }

                    f.write_char(carrier(true))?;
                    long = false;
                }
            }

            vowel.write(f, long)?;
        }

        if *silme {
            f.write_char(mod_rince(base, *is_final))?;
        }

        Ok(())
    }
}
