//! This module defines all mappings to Unicode codepoints used for output. Any
//!     use of a [`char`] from the tengwar block, that hath not its uttermost
//!     source in this module, is an error.

use super::{tehta::Tehta, tema::Tema};


/// A carrier marking for a long vowel.
pub const CARRIER_LONG: char = TENGWA_ARA;
/// A carrier marking for a short vowel.
pub const CARRIER_SHORT: char = TENGWA_TELCO;
/// A ligating variant of the [short carrier](CARRIER_SHORT).
pub const CARRIER_SHORT_LIG: char = TENGWA_TELCO_LIG;


/// Return a carrier mark appropriate for whether a vowel is long.
pub const fn carrier(long: bool) -> char {
    if long { CARRIER_LONG } else { CARRIER_SHORT }
}


/// One dot inside a tengwa.
pub const DC_INNER_DOT_1: char = '';

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

/// Index in this array corresponds to the numerical value of the digit.
pub const NUMERAL: [char; 13] = [
    NUM_0,
    NUM_1, NUM_2, NUM_3, NUM_4,
    NUM_5, NUM_6, NUM_7, NUM_8,
    NUM_9, NUM_A, NUM_B, NUM_C,
];


/// A single dot positioned inside the preceding character.
pub const PUNCT_DOT_0: char = DC_INNER_DOT_1;

cfg_if! (if #[cfg(feature = "dots-standard")] {
    /// One dot, at middle height.
    pub const PUNCT_DOT_1: char = '⸱';
    /// Two dots, arranged vertically; The ASCII colon.
    pub const PUNCT_DOT_2: char = ':';
    /// Three dots, arranged vertically; The tricolon.
    pub const PUNCT_DOT_3: char = '⁝';
    /// Four dots in a diamond configuration.
    pub const PUNCT_DOT_4: char = '⁘';
    /// Five dots in a plus-shape.
    pub const PUNCT_DOT_5: char = '⸭';
} else {
    /// One dot, at middle height.
    pub const PUNCT_DOT_1: char = '';
    /// Two dots, arranged vertically, resembling an ASCII colon.
    pub const PUNCT_DOT_2: char = '';
    /// Three dots, arranged vertically, resembling a tricolon.
    pub const PUNCT_DOT_3: char = '';
    /// Four dots in a diamond configuration.
    pub const PUNCT_DOT_4: char = '';
    /// Five dots in a plus-shape.
    pub const PUNCT_DOT_5: char = '';
});

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


/// A small hook added to a tengwa to indicate a following sibilant.
pub const SA_RINCE: char = '';
/// An ornate flourish added to a tengwa to indicate a following sibilant.
pub const SA_RINCE_FINAL: char = '';


/// A diacritical marking resembling a circumflex.
pub const TEHTA_YANTA: Tehta = Tehta::single(DC_OVER_CIRCUMFLEX);

/// A diacritical Tehta used in most systems to represent the `A` vowel.
pub const TEHTA_A: Tehta = Tehta::single(DC_OVER_DOT_3);
/// A diacritical Tehta used in most systems to represent the `E` vowel.
pub const TEHTA_E: Tehta = Tehta::altern(DC_OVER_ACUTE_1, DC_OVER_ACUTE_2);
/// A diacritical Tehta used in most systems to represent the `I` vowel.
pub const TEHTA_I: Tehta = Tehta::single(DC_OVER_DOT_1);
/// A diacritical Tehta used in most systems to represent the `O` vowel.
pub const TEHTA_O: Tehta = Tehta::altern(DC_OVER_HOOK_R_1, DC_OVER_HOOK_R_2);
/// A diacritical Tehta used in most systems to represent the `U` vowel.
pub const TEHTA_U: Tehta = Tehta::altern(DC_OVER_HOOK_L_1, DC_OVER_HOOK_L_2);
/// A diacritical Tehta used in the Sindarin modes to represent the `Y` vowel.
pub const TEHTA_Y: Tehta = Tehta::single(DC_OVER_DOT_2);


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

impl Tema {
    pub const TINCO: Self = TEMA_TINCO;
    pub const PARMA: Self = TEMA_PARMA;
    pub const CALMA: Self = TEMA_CALMA;
    pub const QESSE: Self = TEMA_QESSE;
}


//  Alternate spellings.
// pub const TENGWA_SULE: char = TENGWA_THULE; // þ -> s
// pub const TENGWA_HARMA: char = TENGWA_AHA;
// pub const TENGWA_QUESSE: char = TENGWA_QESSE;
// pub const TENGWA_VILYA: char = TENGWA_WILYA; // w -> v

//  Explicit names for the Tincotéma.
pub const TENGWA_TINCO: char = TEMA_TINCO.single_dn;
pub const TENGWA_ANDO: char = TEMA_TINCO.double_dn;
pub const TENGWA_THULE: char = TEMA_TINCO.single_up;
pub const TENGWA_ANTO: char = TEMA_TINCO.double_up;
pub const TENGWA_NUMEN: char = TEMA_TINCO.double_sh;
pub const TENGWA_ORE: char = TEMA_TINCO.single_sh;

//  Explicit names for the Parmatéma.
pub const TENGWA_PARMA: char = TEMA_PARMA.single_dn;
pub const TENGWA_UMBAR: char = TEMA_PARMA.double_dn;
pub const TENGWA_FORMEN: char = TEMA_PARMA.single_up;
pub const TENGWA_AMPA: char = TEMA_PARMA.double_up;
pub const TENGWA_MALTA: char = TEMA_PARMA.double_sh;
pub const TENGWA_VALA: char = TEMA_PARMA.single_sh;

//  Explicit names for the Calmatéma.
pub const TENGWA_CALMA: char = TEMA_CALMA.single_dn;
pub const TENGWA_ANGA: char = TEMA_CALMA.double_dn;
pub const TENGWA_AHA: char = TEMA_CALMA.single_up;
pub const TENGWA_ANCA: char = TEMA_CALMA.double_up;
pub const TENGWA_NOLDO: char = TEMA_CALMA.double_sh;
pub const TENGWA_ANNA: char = TEMA_CALMA.single_sh;

//  Explicit names for the Qessetéma.
pub const TENGWA_QESSE: char = TEMA_QESSE.single_dn;
pub const TENGWA_UNGWE: char = TEMA_QESSE.double_dn;
pub const TENGWA_HWESTA: char = TEMA_QESSE.single_up;
pub const TENGWA_UNQUE: char = TEMA_QESSE.double_up;
pub const TENGWA_NWALME: char = TEMA_QESSE.double_sh;
pub const TENGWA_WILYA: char = TEMA_QESSE.single_sh;


//  Irregular tengwar. Arranged in MOSTLY the same order as the codepoints, with
//      some changes for (subjectively) more understandable grouping.
//  U+E02_
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
pub const TENGWA_YANTA: char = '';
pub const TENGWA_URE: char = '';

/// A short vertical mark.
pub const TENGWA_TELCO: char = '';
/// A long vertical mark extending downwards.
pub const TENGWA_ARA: char = '';
/// A long vertical mark extending upwards.
pub const TENGWA_HALLA: char = '';

//  U+E03_
pub const TENGWA_OSSE: char = '';
pub const TENGWA_OSSE_REV: char = '';
pub const TENGWA_BOMBADIL_W: char = '';

/// A ligating variant of [Telco](TENGWA_TELCO).
pub const TENGWA_TELCO_LIG: char = '';
pub const TENGWA_ANNA_OPEN: char = '';
pub const TENGWA_CHRISTOPHER_QU: char = '';
pub const TENGWA_BOMBADIL_HW: char = '';
/// A variant of [Malta](TENGWA_MALTA) with an extra hook.
pub const TENGWA_MALTA_HOOKED: char = '';
/// A variant of [Vala](TENGWA_VALA) with an extra hook.
pub const TENGWA_VALA_HOOKED: char = '';
pub const TENGWA_LOWDHAM_HW: char = '';
pub const TENGWA_WAIA: char = '';

/// "Zero-Width Joiner", used for forming ligatures.
pub const ZWJ: char = '‍';


pub const fn width(c: char) -> Option<usize> {
    match c {
        DC_OVER_DOT_3..=DC_UNDER_LINE_V
        | SA_RINCE..=PUNCT_DOT_0
        | DC_UNDER_RING
        | ZWJ
        => Some(0),

        TENGWA_TINCO..=TENGWA_TELCO
        | TENGWA_OSSE_REV..=TENGWA_OSSE
        | TENGWA_TELCO_LIG..=TENGWA_WAIA
        | SA_RINCE_FINAL
        | PUNCT_DOT_1..=PUNCT_THORIN
        | NUM_0..=NUM_C
        => Some(1),

        _ => None,
    }
}
