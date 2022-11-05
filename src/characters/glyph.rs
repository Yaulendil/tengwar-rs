use std::fmt::{Display, Formatter, Write};
use super::*;


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

    pub const fn with_cons(mut self, cons: char) -> Self {
        self.cons = Some(cons);
        self
    }

    pub const fn with_vowel(mut self, tehta: Tehta) -> Self {
        self.vowel = Some(tehta);
        self
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

    /// Update this glyph with the consonant attributes of another glyph.
    pub fn integrate_consonant(&mut self, other: Self) {
        self.cons = other.cons;
        self.nasal = other.nasal;
        self.labial = other.labial;
        self.palatal = other.palatal;
        self.long_cons = other.long_cons;
    }

    /// Update this glyph with the vowel attributes of another glyph.
    pub fn integrate_vowel(&mut self, other: Self) {
        self.vowel = other.vowel;
        self.long_vowel = other.long_vowel;
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl From<char> for Glyph {
    fn from(cons: char) -> Self {
        Self::new_cons(cons, false)
    }
}
