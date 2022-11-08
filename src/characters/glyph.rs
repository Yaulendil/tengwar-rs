use std::fmt::{Display, Formatter, Write};
use super::*;


/// A single base tengwa, and all of its modifications. This includes the tehta
///     marking, flags for additional diacritics, flags for consonant and vowel
///     length, and an indicator of finality.
#[derive(Clone, Copy, Debug, Default)]
pub struct Glyph {
    /// A base character.
    pub base: Option<char>,
    /// A diacritical marking above the base character.
    pub tehta: Option<Tehta>,

    /// If Silmë follows another tengwa, the base character may be modified by
    ///     a sa-rincë instead.
    pub rince: bool,
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
            base: None,
            tehta: None,
            rince: false,
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

    /// Define a glyph with both a base [`char`] and a diacritical [`Tehta`].
    pub const fn new_both(tengwa: char, tehta: Tehta) -> Self {
        Self { base: Some(tengwa), tehta: Some(tehta), ..Self::new() }
    }

    /// Define a glyph with only a base [`char`]. It may be marked as Long.
    pub const fn new_cons(tengwa: char, long_cons: bool) -> Self {
        Self { base: Some(tengwa), long_cons, ..Self::new() }
    }

    /// Define a glyph with only a diacritical [`Tehta`]. It may be marked as
    ///     Long.
    pub const fn new_vowel(tehta: Tehta, long_vowel: bool) -> Self {
        Self { tehta: Some(tehta), long_vowel, ..Self::new() }
    }

    pub const fn is_short_carrier(&self) -> bool {
        match self {
            Self { base: None, tehta: None, .. } => true,
            Self { base: None, long_vowel: false, .. } => true,
            Self { .. } => false,
        }
    }

    pub const fn with_tengwa(mut self, tengwa: char) -> Self {
        self.base = Some(tengwa);
        self
    }

    pub const fn with_tehta(mut self, tehta: Tehta) -> Self {
        self.tehta = Some(tehta);
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
        self.rince = true;
        self
    }

    /// Update this glyph with the consonant attributes of another glyph.
    pub fn integrate_consonant(&mut self, other: Self) {
        self.base = other.base;
        self.rince = other.rince;
        self.nasal = other.nasal;
        self.labial = other.labial;
        self.palatal = other.palatal;
        self.long_cons = other.long_cons;
    }

    /// Update this glyph with the vowel attributes of another glyph.
    pub fn integrate_vowel(&mut self, other: Self) {
        self.tehta = other.tehta;
        self.long_vowel = other.long_vowel;
    }

    pub fn replace_consonant(&mut self, old: char, new: char) -> bool {
        if self.base == Some(old) {
            self.base = Some(new);
            true
        } else {
            false
        }
    }
}

impl Glyph {
    /// Determine the base character to be used for this glyph. If one is not
    ///     set, an appropriate "carrier" mark will be returned instead.
    pub const fn base(&self) -> char {
        match self {
            #[cfg(feature = "nuquernar")]
            &Glyph { base: Some(base), tehta: Some(ref tehta), long_vowel, .. } if {
                can_be_nuquerna(base) && !(long_vowel && tehta.uses_ara())
            } => nuquerna(base),

            &Glyph { base: Some(base), .. } => base,
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
            base: tengwa, tehta, rince,
            nasal, labial, palatal,
            long_cons, long_vowel, long_first,
            ligate_zwj, is_final,
            ..
        } = self;

        #[cfg_attr(feature = "nuquernar", allow(unused_mut))]
        let mut long: bool = *long_vowel && tengwa.is_some();
        let nuquerna_ignored: bool = !cfg!(feature = "nuquernar")
            && can_be_nuquerna(base);

        let tehta_post: Option<&Tehta> = match tehta {
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

        if let Some(tehta) = tehta_post {
            if long {
                if tehta.uses_ara() {
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

            tehta.write(f, long)?;
        }

        if *rince {
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
