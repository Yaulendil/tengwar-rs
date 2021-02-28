use std::fmt::{Formatter, self, Write};


/// Ára
pub const CARRIER_LONG: char = '';
/// Telco
pub const CARRIER_SHORT: char = '';

pub const MOD_LABIAL: char = '';
/// Long/double consonant.
pub const MOD_LONG_CONS: char = '';
/// Long vowel after consonant.
pub const MOD_LONG_VOWEL: char = '';
pub const MOD_NASAL: char = '';
pub const MOD_PALATAL: char = '';
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


#[cfg(feature = "double-vowels")]
mod _vowels {
    use super::Tehta;

    pub const TEHTA_A: Tehta = Tehta::basic('');
    // pub const TEHTA_A: Tehta = Tehta::basic('');
    /// Tecco
    pub const TEHTA_E: Tehta = Tehta::with_variant('', '');
    /// Tixë
    pub const TEHTA_I: Tehta = Tehta::with_variant('', '');
    pub const TEHTA_O: Tehta = Tehta::with_variant('', '');
    pub const TEHTA_U: Tehta = Tehta::with_variant('', '');
}


#[cfg(not(feature = "double-vowels"))]
mod _vowels {
    use super::Tehta;

    pub const TEHTA_A: Tehta = Tehta::basic('');
    // pub const TEHTA_A: Tehta = Tehta::basic('');
    /// Tecco
    pub const TEHTA_E: Tehta = Tehta::basic('');
    /// Tixë
    pub const TEHTA_I: Tehta = Tehta::basic('');
    pub const TEHTA_O: Tehta = Tehta::basic('');
    pub const TEHTA_U: Tehta = Tehta::basic('');
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
    single_up: '', // Harma
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
    double_up: '', // Unqë
    double_sh: '', // Ñwalmë
    single_sh: '', // Wilya
    single_ex: '',
    double_ex: '',
};


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
        out.push('');
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
        out.push('');
    }

    for &digit in iter {
        out.push(NUMERAL[digit]);
        out.push('');
    }

    out
}


#[derive(Clone)]
pub struct Tehta {
    pub base: char,
    pub long: Option<char>,
}


impl Tehta {
    pub const fn basic(base: char) -> Self {
        Self { base, long: None }
    }

    pub const fn with_variant(base: char, long: char) -> Self {
        Self { base, long: Some(long) }
    }

    pub fn write(&self, f: &mut Formatter<'_>, long: bool) -> fmt::Result {
        if long {
            match self.long {
                Some(variant) => f.write_char(variant),
                None => {
                    // f.write_char(MOD_LONG_VOWEL)?;
                    f.write_char(carrier(true))?;
                    f.write_char(self.base)
                }
            }
        } else {
            f.write_char(self.base)
        }
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
    /// A short Telco, and one Lúva.
    ///     Typically represents a nasal long.
    pub double_sh: char,
    /// A short Telco, and two Lúvar.
    ///     Typically represents a nasal short.
    pub single_sh: char,
    /// An extended Telco, and one Lúva.
    ///     Not used in canonical sources.
    pub single_ex: char,
    /// An extended Telco, and two Lúvar.
    ///     Not used in canonical sources.
    pub double_ex: char,
}


/// Choose the appropriate form of sa-rincë for a base tengwa.
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
    /// A palatalized vowel is represented by an additional diacritic.
    pub palatal: bool,
    pub long_cons: bool,
    pub long_vowel: bool,
}


impl Glyph {
    pub const fn with_both(cons: char, vowel: Tehta) -> Self {
        Self {
            cons: Some(cons),
            vowel: Some(vowel),
            silme: false,
            palatal: false,
            long_cons: false,
            long_vowel: false,
        }
    }

    pub const fn with_cons(cons: char) -> Self {
        Self {
            cons: Some(cons),
            vowel: None,
            silme: false,
            palatal: false,
            long_cons: false,
            long_vowel: false,
        }
    }

    pub const fn with_vowel(vowel: Tehta) -> Self {
        Self {
            cons: None,
            vowel: Some(vowel),
            silme: false,
            palatal: false,
            long_cons: false,
            long_vowel: false,
        }
    }

    const fn get_base(&self, base: char) -> (char, bool) {
        if self.silme {
            if base == TEMA_PARMA.double_sh {
                return ('', false);
            } else if base == TEMA_PARMA.single_sh {
                return ('', false);
            }
        }

        //  If Silmë takes a tehta, it is inverted.
        if base == TENGWA_SILME {
            if self.vowel.is_some() {
                return (TENGWA_SILME_NUQ, self.silme);
            }
        }

        //  If Essë takes a tehta, it is inverted.
        else if base == TENGWA_ESSE {
            if self.vowel.is_some() {
                return (TENGWA_ESSE_NUQ, self.silme);
            }
        }

        (base, self.silme)
    }
}


impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Glyph { cons, vowel, silme, palatal, long_cons, long_vowel } = self;
        let (base, rince): (char, bool) = match cons {
            Some(ch) => self.get_base(*ch),
            None => (carrier(*long_vowel), *silme),
        };

        f.write_char(base)?;

        if *long_cons {
            f.write_char(MOD_LONG_CONS)?;
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
