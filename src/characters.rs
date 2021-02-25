use std::fmt::{Formatter, self, Write};


/// Ára
pub const CARRIER_LONG: char = '';
/// Telco
pub const CARRIER_SHORT: char = '';

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


pub const TEHTA_A: Tehta = Tehta::basic('');
/// Tecco
pub const TEHTA_E: Tehta = Tehta::basic('');
/// Tixë
pub const TEHTA_I: Tehta = Tehta::basic('');
// pub const TEHTA_I: Tehta = Tehta::with_variant('', ''); // Tixë
pub const TEHTA_O: Tehta = Tehta::basic('');
pub const TEHTA_U: Tehta = Tehta::basic('');


pub const TEMA_TINCO: Tema = Tema {
    base: '', // Tinco
    voiced: '', // Ando
    fric: '', // Thúlë
    fric_voiced: '', // Anto
    nasal: '', // Númen
    special: '', // Órë
    // e1: '', e2: '',
};
pub const TEMA_PARMA: Tema = Tema {
    base: '', // Parma
    voiced: '', // Umbar
    fric: '', // Formen
    fric_voiced: '', // Ampa
    nasal: '', // Malta
    special: '', // Vala
    // e1: '', e2: '',
};
pub const TEMA_CALMA: Tema = Tema {
    base: '', // Calma
    voiced: '', // Anga
    fric: '', // Harma
    fric_voiced: '', // Anca
    nasal: '', // Ñoldo
    special: '', // Anna
    // e1: '', e2: '',
};
pub const TEMA_QESSE: Tema = Tema {
    base: '', // Qessë
    voiced: '', // Ungwë
    fric: '', // Hwesta
    fric_voiced: '', // Unqë
    nasal: '', // Ñwalmë
    special: '', // Wilya
    // e1: '', e2: '',
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

    pub fn write(
        &self, f: &mut Formatter<'_>, long: bool, palatal: bool,
    ) -> fmt::Result {
        if palatal {
            f.write_char(MOD_PALATAL)?;
        }

        if long {
            // match self.long {
            //     Some(variant) => f.write_char(variant),
            //     None => {
            f.write_char(MOD_LONG_VOWEL)?;
            f.write_char(self.base)
            //     }
            // }
        } else {
            f.write_char(self.base)
        }
    }
}


#[derive(Clone)]
pub struct Tema {
    pub base: char,
    pub voiced: char,
    pub fric: char,
    pub fric_voiced: char,
    pub nasal: char,
    pub special: char,
}


pub type Tengwa = char;
// #[derive(Clone)]
// pub enum Tengwa {
//     Regular(char),
//     Irregular(char),
// }


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
    pub cons: Option<Tengwa>,
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
    const fn get_base(&self, base: char) -> (char, bool) {
        if self.silme {
            if base == TEMA_PARMA.nasal {
                return ('', false);
            } else if base == TEMA_PARMA.special {
                return ('', false);
            }
        }

        //  If Órë takes a tehta, it turns to Rómen.
        if base == TEMA_TINCO.special {
            if self.vowel.is_some() {
                return (TENGWA_ROMEN, self.silme);
            }
        }

        //  If Silmë takes a tehta, it is inverted.
        else if base == TENGWA_SILME {
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

        if let Some(vowel) = vowel {
            vowel.write(f, *long_vowel, *palatal)?;
        }

        if rince {
            f.write_char(mod_rince(base))?;
        }

        Ok(())
    }
}
