use std::fmt::{Formatter, self, Write};


/// Ára
pub const CARRIER_LONG: char = '';
/// Telco
pub const CARRIER_SHORT: char = '';


/// One acute accent above a tengwa.
pub const DC_OVER_ACUTE_1: char = '';
/// Two acute accents above a tengwa.
pub const DC_OVER_ACUTE_2: char = '';
/// One grave accent above a tengwa.
pub const DC_OVER_GRAVE: char = '';

/// A diacritic similar to a circumflex.
pub const DC_OVER_CIRCUMFLEX: char = '';

/// One dot above a tengwa.
pub const DC_OVER_DOT_1: char = '';
/// Two dots above a tengwa, arranged horizontally.
pub const DC_OVER_DOT_2: char = '';
/// Three dots above a tengwa, with one above the others.
pub const DC_OVER_DOT_3: char = '';
/// Three dots above a tengwa, with one below the others.
pub const DC_OVER_DOT_3_INV: char = '';

pub const DC_OVER_HOOK_L_1: char = '';
pub const DC_OVER_HOOK_L_2: char = '';

pub const DC_OVER_HOOK_R_1: char = '';
pub const DC_OVER_HOOK_R_2: char = '';

pub const DC_OVER_LINE: char = '';
pub const DC_OVER_WAVE: char = '';


/// One acute accent below a tengwa.
pub const DC_UNDER_ACUTE_1: char = '';
/// One inverted acute accent below a tengwa.
pub const DC_UNDER_ACUTE_2: char = '';

/// One dot below a tengwa.
pub const DC_UNDER_DOT_1: char = '';
/// Two dots below a tengwa, arranged horizontally.
pub const DC_UNDER_DOT_2: char = '';
/// Three dots below a tengwa, with one below the others.
pub const DC_UNDER_DOT_3: char = '';

pub const DC_UNDER_HOOK_L_1: char = '';
pub const DC_UNDER_HOOK_R_1: char = '';
pub const DC_UNDER_LINE_H: char = '';
pub const DC_UNDER_LINE_V: char = '';

pub const DC_UNDER_RING: char = '';


pub const MOD_LABIAL: char = DC_OVER_WAVE;
/// Long/double consonant.
pub const MOD_LONG_CONS: char = DC_UNDER_LINE_H;
/// Long vowel after consonant.
pub const MOD_LONG_VOWEL: char = DC_UNDER_LINE_V;

pub const MOD_NASAL: char = DC_OVER_LINE;
pub const MOD_PALATAL: char = DC_UNDER_DOT_2;
pub const MOD_SARINCE_L: char = '';
pub const MOD_SARINCE_R: char = '';

/// Index in this array corresponds to the numerical value of the character.
pub const NUMERAL: [char; 12] = [
    '', '', '', '', '',
    '', '', '', '', '',
    '', '',
];


pub const PUNCT_DOT_1: &str = "";
pub const PUNCT_DOT_2: &str = "";
pub const PUNCT_DOT_3: &str = "";
pub const PUNCT_DOT_4: &str = "";
pub const PUNCT_DOT_5: &str = "";

pub const PUNCT_DOT_S1: &str = "  ";
pub const PUNCT_DOT_TRI: &str = "";
pub const PUNCT_DOT_DIAM: &str = "";

pub const PUNCT_EXCLAM: &str = "";
pub const PUNCT_INTERR: &str = "";

pub const PUNCT_LINE_1: &str = "";
pub const PUNCT_LINE_2: &str = "";
pub const PUNCT_LINE_S1: &str = "  ";
pub const PUNCT_LINE_S2: &str = "  ";

pub const PUNCT_PAREN: &str = "";
pub const PUNCT_PAREN_L: &str = "";
pub const PUNCT_PAREN_R: &str = "";

pub const PUNCT_EOF: &str = "";


pub const TEHTA_CIRCUMFLEX: Tehta = Tehta::basic(DC_OVER_CIRCUMFLEX);


/// The diacritic for an `A` vowel, in its standard three-dot form.
#[cfg(not(feature = "circumflex"))]
const _A: char = DC_OVER_DOT_3;


/// The diacritic for an `A` vowel, in its alternate circumflex-like form.
#[cfg(feature = "circumflex")]
const _A: char = DC_OVER_CIRCUMFLEX;


#[cfg(not(any(feature = "long-vowel-double", feature = "long-vowel-unique")))]
mod _vowels {
    use super::*;

    pub const TEHTA_A: Tehta = Tehta::basic(_A);
    pub const TEHTA_E: Tehta = Tehta::basic(DC_OVER_ACUTE_1);
    pub const TEHTA_I: Tehta = Tehta::basic(DC_OVER_DOT_1);
    pub const TEHTA_O: Tehta = Tehta::basic(DC_OVER_HOOK_R_1);
    pub const TEHTA_U: Tehta = Tehta::basic(DC_OVER_HOOK_L_1);
    pub const TEHTA_Y: Tehta = Tehta::basic(DC_OVER_DOT_2);
}


#[cfg(feature = "long-vowel-double")]
mod _vowels {
    use super::*;

    pub const TEHTA_A: Tehta = Tehta::basic(_A);
    pub const TEHTA_E: Tehta = Tehta::with_double(DC_OVER_ACUTE_1);
    // pub const TEHTA_I: Tehta = Tehta::with_double(DC_OVER_DOT_1);
    pub const TEHTA_I: Tehta = Tehta::basic(DC_OVER_DOT_1);
    pub const TEHTA_O: Tehta = Tehta::with_double(DC_OVER_HOOK_R_1);
    pub const TEHTA_U: Tehta = Tehta::with_double(DC_OVER_HOOK_L_1);
    pub const TEHTA_Y: Tehta = Tehta::basic(DC_OVER_DOT_2);
}


#[cfg(all(feature = "long-vowel-unique", not(feature = "long-vowel-double")))]
mod _vowels {
    use super::*;

    pub const TEHTA_A: Tehta = Tehta::basic(_A);
    pub const TEHTA_E: Tehta = Tehta::with_variant(DC_OVER_ACUTE_1, DC_OVER_ACUTE_2);
    // pub const TEHTA_I: Tehta = Tehta::with_variant(DC_OVER_DOT_1, DC_OVER_DOT_2);
    pub const TEHTA_I: Tehta = Tehta::basic(DC_OVER_DOT_1);
    pub const TEHTA_O: Tehta = Tehta::with_variant(DC_OVER_HOOK_R_1, DC_OVER_HOOK_R_2);
    pub const TEHTA_U: Tehta = Tehta::with_variant(DC_OVER_HOOK_L_1, DC_OVER_HOOK_L_2);
    pub const TEHTA_Y: Tehta = Tehta::basic(DC_OVER_DOT_2);
}


pub use _vowels::*;


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

pub const TENGWA_CURL_SINGLE: char = '';
pub const TENGWA_CURL_DOUBLE: char = '';

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
        digits.push((n % base).abs() as usize);
        n /= base;
    }

    (neg, digits)
}


pub fn int_10(n: isize) -> String {
    let (neg, digits): (bool, Vec<usize>) = int(n, 10);
    let mut out = String::with_capacity(neg as usize + digits.len() * 6);
    let iter = digits.iter();

    if neg { out.push('-'); }

    for &digit in iter {
        out.push(NUMERAL[digit]);
        out.push(DC_OVER_DOT_1);
    }

    out
}


pub fn int_12(n: isize) -> String {
    let (neg, digits): (bool, Vec<usize>) = int(n, 12);
    let mut out = String::with_capacity(neg as usize + digits.len() * 6);
    let mut iter = digits.iter();

    if neg { out.push('-'); }

    //  Mark the least significant digit uniquely.
    if let Some(&first) = iter.next() {
        out.push(NUMERAL[first]);
        out.push(DC_UNDER_RING);
    }

    for &digit in iter {
        out.push(NUMERAL[digit]);
        out.push(DC_UNDER_DOT_1);
    }

    out
}


#[derive(Clone)]
pub struct Tehta {
    pub base: char,
    pub long: Option<char>,
    pub double: bool,
}


impl Tehta {
    pub const fn basic(base: char) -> Self {
        Self { base, long: None, double: false }
    }

    pub const fn with_double(base: char) -> Self {
        Self { base, long: None, double: true }
    }

    pub const fn with_variant(base: char, long: char) -> Self {
        Self { base, long: Some(long), double: false }
    }

    pub fn write(&self, f: &mut Formatter<'_>, long: bool) -> fmt::Result {
        if long {
            if self.double {
                f.write_char(self.base)?;
                f.write_char(self.base)
            } else {
                match self.long {
                    Some(variant) => f.write_char(variant),
                    None => {
                        // f.write_char(MOD_LONG_VOWEL)?;
                        f.write_char(carrier(true))?;
                        f.write_char(self.base)
                    }
                }
            }
        } else {
            f.write_char(self.base)
        }
    }

    /// Returns true if the long variant of this Tehta would be written with the
    ///     extended "Ára" Telco.
    pub const fn uses_ara(&self) -> bool {
        !self.double && self.long.is_none()
    }
}


/// The Témar are the four series of the primary tengwar. Each Téma is composed
///     of eight Tyeller, each modifying the tengwa in a different way, and is
///     named after its base tengwa.
///
/// Only the first six Tyeller are used in Quenya.
#[derive(Clone)]
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


/// Determine whether two `Glyph`s can be joined by a zero-width joiner. These
///     rules are based on the "Tengwar Telcontar" font.
pub const fn ligature_valid(prev: &Glyph, next: &Glyph) -> bool {
    //  TODO
    true
}


/// Choose the appropriate form of sa-rincë for a base tengwa.
#[cfg(not(feature = "alt-rince"))]
pub const fn mod_rince(_base: char) -> char {
    MOD_SARINCE_L
}


/// Choose the appropriate form of sa-rincë for a base tengwa.
#[cfg(feature = "alt-rince")]
pub const fn mod_rince(base: char) -> char {
    match base {
        '' | '' | '' | '' | '' | '' | '' | ''
        => MOD_SARINCE_R,
        _ => MOD_SARINCE_L,
    }
}


#[derive(Clone, Default)]
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
}


impl Glyph {
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
        }
    }

    pub const fn new_both(cons: char, vowel: Tehta) -> Self {
        Self {
            cons: Some(cons),
            vowel: Some(vowel),
            ..Self::new()
        }
    }

    pub const fn new_cons(cons: char, long: bool) -> Self {
        Self {
            cons: Some(cons),
            long_cons: long,
            ..Self::new()
        }
    }

    pub const fn new_vowel(vowel: Tehta, long: bool) -> Self {
        Self {
            vowel: Some(vowel),
            long_vowel: long,
            ..Self::new()
        }
    }

    pub const fn with_labial(mut self) -> Self {
        self.labial = true;
        self
    }

    pub const fn with_nasal(mut self) -> Self {
        self.nasal = true;
        self
    }

    pub const fn with_palatal(mut self) -> Self {
        self.palatal = true;
        self
    }

    pub const fn with_silme(mut self) -> Self {
        self.silme = true;
        self
    }

    const fn get_base(&self, base: char) -> (char, bool) {
        #[cfg(feature = "nuquernar")]
        //  If Silmë takes a tehta, it is inverted.
        if base == TENGWA_SILME {
            if self.vowel.is_some() {
                return (TENGWA_SILME_NUQ, self.silme);
            }
        }

        #[cfg(feature = "nuquernar")]
        //  If Essë takes a tehta, it is inverted.
        if base == TENGWA_ESSE {
            if self.vowel.is_some() {
                return (TENGWA_ESSE_NUQ, self.silme);
            }
        }

        (base, self.silme)
    }
}


impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Glyph {
            cons, vowel, silme,
            nasal, labial, palatal,
            long_cons, long_vowel, long_first,
        } = self;

        let vowel_first: bool = *long_first
            && (*long_vowel || *labial)
            && cons.is_some();

        if vowel_first {
            if let Some(vowel) = vowel {
                if vowel.uses_ara() && *long_vowel {
                    vowel.write(f, true)?;
                } else {
                    f.write_char(carrier(*long_vowel))?;
                    vowel.write(f, false)?;
                }
            }

            let (base, rince) = self.get_base(cons.unwrap());

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

            if rince {
                f.write_char(mod_rince(base))?;
            }

            Ok(())
        } else {
            let (base, rince): (char, bool) = match cons {
                Some(ch) => self.get_base(*ch),
                None => (carrier(*long_vowel), *silme),
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

            if let Some(vowel) = vowel {
                vowel.write(f, *long_vowel && cons.is_some())?;
            }

            if rince {
                f.write_char(mod_rince(base))?;
            }

            Ok(())
        }
    }
}
