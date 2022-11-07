use crate::characters::*;


/// This trait defines higher-level behavior for rendering Tengwar.
#[allow(unused_variables)]
pub trait Policy {
    /// Returns a boolean indicating whether a given character may form a
    ///     ligature with a [long carrier](CARRIER_LONG) that follows it.
    ///
    /// The ligature will be formed by emitting a Zero-Width Joiner between the
    ///     two characters.
    fn can_ligate_with_ara(&self, c: char) -> bool { false }

    /// Returns a boolean indicating whether a [short carrier](CARRIER_SHORT)
    ///     may form a ligature with a given character that follows it.
    ///
    /// The ligature will be formed by replacing the short carrier character
    ///     with [a variant](CARRIER_SHORT_LIG).
    fn can_telco_ligate_with(&self, c: char) -> bool { false }

    /// Returns the "Nuquerna", or inverted, variant of a given character, if it
    ///     has one.
    ///
    /// The Nuquerna variant is used when a significant portion of a tengwa
    ///     extends above the center of the character, but a diacritical tehta
    ///     also needs to occupy that same space. The Nuquerna variant instead
    ///     extends downwards, leaving the space above free for the tehta.
    fn nuquerna(&self, c: char) -> Option<char> { None }

    /// Returns the appropriate "Sa-Rincë", or "S-hook", for a given character,
    ///     if it can host one, taking into account whether it is the final
    ///     character in a word.
    ///
    /// The Sa-Rincë is attached to indicate that a sibilant sound follows the
    ///     character. For a character at the end of a word, a more ornate
    ///     variant may be used.
    fn sa_rince(&self, c: char, is_final: bool) -> Option<char> { None }
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Standard;

impl Policy for Standard {
    fn can_ligate_with_ara(&self, c: char) -> bool {
        match c {
            TENGWA_SILME_NUQ => false,
            TENGWA_ESSE_NUQ => false,
            TENGWA_ESSE => false,

            TENGWA_TINCO..=TENGWA_HWESTA_SINDARINWA => true,
            _ => false,
        }
    }

    fn can_telco_ligate_with(&self, _c: char) -> bool {
        //  TODO
        true
    }

    fn nuquerna(&self, c: char) -> Option<char> {
        match c {
            TENGWA_SILME => Some(TENGWA_SILME_NUQ),
            TENGWA_ESSE => Some(TENGWA_ESSE_NUQ),
            _ => None,
        }
    }

    #[cfg_attr(not(feature = "alt-rince"), allow(unused_variables))]
    fn sa_rince(&self, c: char, is_final: bool) -> Option<char> {
        match c {
            TENGWA_ROMEN | TENGWA_ARDA
            | TENGWA_SILME | TENGWA_SILME_NUQ
            | TENGWA_ESSE | TENGWA_ESSE_NUQ
            => None,

            #[cfg(feature = "alt-rince")]
            TENGWA_TINCO..=TENGWA_WILYA
            | TENGWA_LAMBE | TENGWA_ALDA | TENGWA_HYARMEN
            if is_final => Some(SA_RINCE_FINAL),

            _ => Some(SA_RINCE),
        }
    }
}
