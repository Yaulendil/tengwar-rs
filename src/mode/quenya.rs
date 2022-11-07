use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


const CARRIER_DIPH_I: char = TENGWA_YANTA;
const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']
        | ['n', 'd']    /**/ => TEMA_TINCO.double_dn,
        ['þ']
        | ['θ']
        | ['t', 'h']    /**/ => TEMA_TINCO.single_up,
        ['n', 't']      /**/ => TEMA_TINCO.double_up,
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TEMA_TINCO.single_sh,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']
        | ['m', 'b']    /**/ => TEMA_PARMA.double_dn,
        ['φ'] | ['f']   /**/ => TEMA_PARMA.single_up,
        ['m', 'p']      /**/ => TEMA_PARMA.double_up,
        ['m']           /**/ => TEMA_PARMA.double_sh,
        ['v']           /**/ => TEMA_PARMA.single_sh,

        ['c']
        | ['k']         /**/ => TEMA_CALMA.single_dn,
        ['n', 'g']
        | ['g']         /**/ => TEMA_CALMA.double_dn,
        ['c', 'h']      /**/ => TEMA_CALMA.single_up,
        ['n', 'c']      /**/ => TEMA_CALMA.double_up,
        ['ñ']           /**/ => TEMA_CALMA.double_sh,
        ['y']           /**/ => TEMA_CALMA.single_sh,

        ['q']
        | ['q', 'u']
        | ['c', 'w']
        | ['k', 'w']    /**/ => TEMA_QESSE.single_dn,
        ['n', 'g', 'w'] /**/ => TEMA_QESSE.double_dn,
        ['h', 'w']      /**/ => TEMA_QESSE.single_up,
        ['n', 'q', 'u']
        | ['n', 'q']    /**/ => TEMA_QESSE.double_up,
        ['ñ', 'w']      /**/ => TEMA_QESSE.double_sh,
        ['w']           /**/ => TEMA_QESSE.single_sh,

        //  Irregular
        ['r', 'd']      /**/ => TENGWA_ARDA,
        ['l']           /**/ => TENGWA_LAMBE,
        ['l', 'd']      /**/ => TENGWA_ALDA,
        ['s']           /**/ => TENGWA_SILME,
        ['s', 's']
        | ['z']         /**/ => TENGWA_ESSE,

        ['h']           /**/ => TENGWA_HYARMEN,

        _ => { return None; }
    })
}


pub const fn get_consonant(slice: &[char]) -> Option<Glyph> {
    match consonant_char(slice) {
        Some(cons) => Some(Glyph::new_cons(cons, false)),
        None => match slice {
            &[a, b] if a == b => match consonant_char(&[a]) {
                Some(cons) => Some(Glyph::new_cons(cons, true)),
                None => None,
            }
            _ => None,
        }
    }
}


pub const fn get_diphthong(slice: &[char]) -> Option<Glyph> {
    match slice {
        ['a', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_A)),
        ['o', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_O)),
        ['u', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_U)),

        ['a', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_A)),
        ['e', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_E)),
        ['i', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_I)),

        _ => None,
    }
}


pub const fn get_vowel(slice: &[char]) -> Option<(Tehta, bool)> {
    match slice {
        ['a'] | ['ä'] => Some((TEHTA_A, false)),
        ['e'] | ['ë'] => Some((TEHTA_E, false)),
        ['i'] | ['ï'] => Some((TEHTA_I, false)),
        ['o'] | ['ö'] => Some((TEHTA_O, false)),
        ['u'] | ['ü'] => Some((TEHTA_U, false)),

        ['á'] | ['a', 'a'] => Some((TEHTA_A, true)),
        ['é'] | ['e', 'e'] => Some((TEHTA_E, true)),
        ['í'] | ['i', 'i'] => Some((TEHTA_I, true)),
        ['ó'] | ['o', 'o'] => Some((TEHTA_O, true)),
        ['ú'] | ['u', 'u'] => Some((TEHTA_U, true)),

        _ => None,
    }
}


/// The Classical Mode, developed by Fëanáro Finwion in Valinor, during the
///     Years of the Trees, for writing Quenya.
#[derive(Clone, Copy, Debug, Default)]
pub struct Quenya {
    current: Option<Glyph>,
    previous: Option<Glyph>,
}

impl TengwarMode for Quenya {
    fn finish_current(&mut self) -> Option<Token> {
        self.previous = self.current.take();
        self.previous.map(Token::Tengwa)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        macro_rules! finish {
            ($glyph:expr) => {finish!($glyph, 0)};
            ($glyph:expr, $len:expr) => {{
                let glyph = $glyph;

                self.current = None;
                self.previous = Some(glyph);

                ParseAction::MatchedToken {
                    token: Token::Tengwa(glyph),
                    len: $len,
                }
            }};
        }

        let initial: bool = self.previous.is_none();

        if let ['\\', _, ..] = chunk {
            ParseAction::Escape
        } else if let Some(current) = &mut self.current {
            //  A tengwa is currently being constructed. Try to continue it.

            match &current.vowel {
                Some(_) => ParseAction::MatchedNone,
                None => match chunk {
                    ['y'] if !current.palatal => {
                        current.palatal = true;
                        ParseAction::MatchedPart(1)
                    }
                    ['s'] if !current.silme => {
                        current.silme = true;
                        ParseAction::MatchedPart(1)
                    }
                    ['s', 's'] => {
                        //  This cannot modify a consonant, but if the window is
                        //      allowed to narrow, it will become ['s'], which
                        //      will modify it incorrectly. Need to output the
                        //      current glyph immediately.
                        finish!(*current, 0)
                    }
                    ['l' | 'r'] => {
                        current.replace_consonant(TENGWA_HYARMEN, TENGWA_HALLA);
                        finish!(*current, 0)
                    }
                    _ => {
                        if let Some(_) = get_diphthong(chunk) {
                            current.replace_consonant(TENGWA_ORE, TENGWA_ROMEN);
                            finish!(*current, 0)
                        } else if let Some((vowel, long)) = get_vowel(chunk) {
                            current.replace_consonant(TENGWA_ORE, TENGWA_ROMEN);
                            current.vowel = Some(vowel);
                            current.long_vowel = long;
                            finish!(*current, chunk.len())
                        } else {
                            ParseAction::MatchedNone
                        }
                    }
                }
            }
        } else {
            //  Try to find a new tengwa.

            //  Check for special cases.
            if let ['x'] = chunk {
                self.current = Some(Glyph::from(TENGWA_CALMA).with_silme());
                ParseAction::MatchedPart(1)
            } else if let ['y', ..] = chunk {
                self.current = Some(Glyph::from(TENGWA_ANNA).with_palatal());
                ParseAction::MatchedPart(1)
            }

            //  Check for a consonant.
            else if let Some(mut new) = get_consonant(chunk) {
                if initial {
                    //  TODO: These special cases allow for using basic ASCII
                    //      `ng`, instead of needing to use `ñ`, to specify the
                    //      archaic initial spellings. However, this approach
                    //      will also be applied to detached suffixes, such as
                    //      "-ngwë" (which is technically initial, but does not
                    //      truly represent the initial variant of the tengwa).
                    //      Checking one token previous would not fix it either,
                    //      because that would then misspell hyphenated compound
                    //      phrases like "etya-ngoldorin".

                    //  Initial NG is represented by Ñoldo, not Anga.
                    new.replace_consonant(TENGWA_ANGA, TENGWA_NOLDO);

                    //  Initial NGW is represented by Ñwalmë, not Ungwë.
                    new.replace_consonant(TENGWA_UNGWE, TENGWA_NWALME);
                } else {
                    //  Medial H is represented by Aha, not Hyarmen.
                    new.replace_consonant(TENGWA_HYARMEN, TENGWA_AHA);
                }

                self.current = Some(new);
                ParseAction::MatchedPart(chunk.len())
            }

            //  Check for a diphthong.
            else if let Some(new) = get_diphthong(chunk) {
                self.current = Some(new);
                ParseAction::MatchedPart(chunk.len())
            }

            //  Check for a single vowel.
            else if let Some((vowel, long)) = get_vowel(chunk) {
                self.current = Some(Glyph::new_vowel(vowel, long));
                ParseAction::MatchedPart(chunk.len())
            } else {
                ParseAction::MatchedNone
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quenya() {
        let eleni_silar = test_tengwar!(Quenya, "eleni sílar" => [
            CARRIER_SHORT, TEHTA_E.base(), // e
            TENGWA_LAMBE, TEHTA_E.base(), // le
            TENGWA_NUMEN, TEHTA_I.base(), // ni
            ' ',
            TENGWA_SILME, // s
            CARRIER_LONG, TEHTA_I.long(), // í
            TENGWA_LAMBE, TEHTA_A.base(), // la
            TENGWA_ORE, // r
        ]);
        test_tengwar!(Quenya, "Eleni Sílar" == eleni_silar);
        test_tengwar!(Quenya, "Elënï Sílär" == eleni_silar);
        test_tengwar!(Quenya, "ELËNÏ SÍLÄR" == eleni_silar);
        test_tengwar!(Quenya, "ELeNi SiiLaR" == eleni_silar);
        test_tengwar!(Quenya, "ELENI SIILAR" == eleni_silar);

        test_tengwar!(Quenya, "Elen síla lúmenn' omentielvo :" => [
            CARRIER_SHORT, TEHTA_E.base(), // e
            TENGWA_LAMBE, TEHTA_E.base(), // le
            TENGWA_NUMEN, // n
            ' ',
            TENGWA_SILME, // s
            CARRIER_LONG, TEHTA_I.long(), // í
            TENGWA_LAMBE, TEHTA_A.base(), // la
            ' ',
            TENGWA_LAMBE, pre_long!(TEHTA_U), TEHTA_U.long(), // lú
            TENGWA_MALTA, TEHTA_E.base(), // me
            TENGWA_NUMEN, DC_UNDER_LINE_H, // nn
            PUNCT_DOT_1, // '
            ' ',
            CARRIER_SHORT, TEHTA_O.base(), // o
            TENGWA_MALTA, TEHTA_E.base(), // me
            TENGWA_ANTO, TEHTA_I.base(), // nti
            CARRIER_SHORT, TEHTA_E.base(), // e
            TENGWA_LAMBE, // l
            TENGWA_VALA, TEHTA_O.base(), // vo
            ' ', PUNCT_DOT_2,
        ]);

        let helcaraxe = test_tengwar!(Quenya, "helcaraxë" => [
            TENGWA_HYARMEN, TEHTA_E.base(), // he
            TENGWA_LAMBE, // l
            TENGWA_CALMA, TEHTA_A.base(), // ca
            TENGWA_ROMEN, TEHTA_A.base(), // ra
            TENGWA_CALMA, TEHTA_E.base(), rince!(), // xë
        ]);
        test_tengwar!(Quenya, "helkarakse" == helcaraxe);

        let quenya = test_tengwar!(Quenya, "quenya" => [
            TENGWA_QESSE, TEHTA_E.base(), // que
            TENGWA_NUMEN, MOD_PALATAL, TEHTA_A.base(), // nya
        ]);
        test_tengwar!(Quenya, "qenya" == quenya);
        test_tengwar!(Quenya, "kwenya" == quenya);
        test_tengwar!(Quenya, "cwenya" == quenya);
        test_tengwar!(Quenya, "kuenya" != quenya);
        test_tengwar!(Quenya, "cuenya" != quenya);

        let _aha = test_tengwar!(Quenya, "aha" => [
            CARRIER_SHORT, TEHTA_A.base(), // a
            TENGWA_AHA, TEHTA_A.base(), // ha
        ]);

        let _hyarmen = test_tengwar!(Quenya, "hyarmen" => [
            TENGWA_HYARMEN, MOD_PALATAL, TEHTA_A.base(), // hya
            TENGWA_ORE, // r
            TENGWA_MALTA, TEHTA_E.base(), // me
            TENGWA_NUMEN, // n
        ]);

        let _hwesta = test_tengwar!(Quenya, "hwesta" => [
            TENGWA_HWESTA, TEHTA_E.base(), // hwe
            TENGWA_SILME, // s
            TENGWA_TINCO, TEHTA_A.base(), // ta
        ]);

        let ara = test_tengwar!(Quenya, "ára" => [
            CARRIER_LONG, TEHTA_A.long(), // á
            TENGWA_ROMEN, TEHTA_A.base(), // ra
        ]);
        test_tengwar!(Quenya, "aara" == ara); // ASCII spelling.

        //  Archaic TH (> S).
        let thuule = test_tengwar!(Quenya, "þúlë" => [
            TENGWA_THULE, pre_long!(TEHTA_U), TEHTA_U.long(), // þú
            TENGWA_LAMBE, TEHTA_E.base(), // lë
        ]);
        test_tengwar!(Quenya, "thuule" == thuule); // ASCII spelling.
        test_tengwar!(Quenya, "θúlë" == thuule);
        test_tengwar!(Quenya, "ΘÚLË" == thuule);
        test_tengwar!(Quenya, "ÞÚLË" == thuule);
        test_tengwar!(Quenya, "súlë" != thuule);

        let calma = test_tengwar!(Quenya, "calma" => [
            TENGWA_CALMA, TEHTA_A.base(), // ca
            TENGWA_LAMBE, // l
            TENGWA_MALTA, TEHTA_A.base(), // ma
        ]);
        test_tengwar!(Quenya, "kalma" == calma);

        //  Initial and final N.
        let nuumen = test_tengwar!(Quenya, "númen" => [
            TENGWA_NUMEN, pre_long!(TEHTA_U), TEHTA_U.long(), // nú
            TENGWA_MALTA, TEHTA_E.base(), // me
            TENGWA_NUMEN, // n
        ]);
        test_tengwar!(Quenya, "nuumen" == nuumen); // ASCII spelling.
        test_tengwar!(Quenya, "ngúmen" != nuumen);

        //  Initial NG (> N).
        let ngoldo = test_tengwar!(Quenya, "ñoldo" => [
            TENGWA_NOLDO, TEHTA_O.base(), // ño
            TENGWA_ALDA, TEHTA_O.base(), // ldo
        ]);
        test_tengwar!(Quenya, "ngoldo" == ngoldo); // ASCII spelling.
        test_tengwar!(Quenya, "ÑOLDO" == ngoldo);
        test_tengwar!(Quenya, "noldo" != ngoldo);

        //  Initial NGW (> NW).
        let ngwalme = test_tengwar!(Quenya, "ñwalmë" => [
            TENGWA_NWALME, TEHTA_A.base(), // ñwa
            TENGWA_LAMBE, // l
            TENGWA_MALTA, TEHTA_E.base(), // më
        ]);
        test_tengwar!(Quenya, "ngwalme" == ngwalme); // ASCII spelling.
        test_tengwar!(Quenya, "nwalmë" != ngwalme);

        //  Medial NG.
        let anga = test_tengwar!(Quenya, "anga" => [
            CARRIER_SHORT, TEHTA_A.base(), // a
            TENGWA_ANGA, TEHTA_A.base(), // nga
        ]);
        test_tengwar!(Quenya, "aña" != anga);
        test_tengwar!(Quenya, "ana" != anga);

        //  Medial NGW.
        let ungwe = test_tengwar!(Quenya, "ungwë" => [
            CARRIER_SHORT, TEHTA_U.base(), // u
            TENGWA_UNGWE, TEHTA_E.base(), // ngwë
        ]);
        test_tengwar!(Quenya, "ungwe" == ungwe); // ASCII spelling.
        test_tengwar!(Quenya, "uñwë" != ungwe);
        test_tengwar!(Quenya, "unwë" != ungwe);

        let _silme = test_tengwar!(Quenya, "silmë" => [
            nuq!(TENGWA_SILME), TEHTA_I.base(), // si
            TENGWA_LAMBE, // l
            TENGWA_MALTA, TEHTA_E.base(), // më
        ]);

        let esse = test_tengwar!(Quenya, "essë" => [
            CARRIER_SHORT, TEHTA_E.base(), // e,
            nuq!(TENGWA_ESSE), TEHTA_E.base(), // ssë
        ]);
        test_tengwar!(Quenya, "eze" == esse);

        //  Test all diphthongs.
        test_tengwar!(Quenya, "ai" => [CARRIER_DIPH_I, TEHTA_A.base()]);
        test_tengwar!(Quenya, "oi" => [CARRIER_DIPH_I, TEHTA_O.base()]);
        test_tengwar!(Quenya, "ui" => [CARRIER_DIPH_I, TEHTA_U.base()]);
        test_tengwar!(Quenya, "au" => [CARRIER_DIPH_U, TEHTA_A.base()]);
        test_tengwar!(Quenya, "eu" => [CARRIER_DIPH_U, TEHTA_E.base()]);
        test_tengwar!(Quenya, "iu" => [CARRIER_DIPH_U, TEHTA_I.base()]);

        //  Test all vowels, alone.
        test_tengwar!(Quenya, "a" => [CARRIER_SHORT, TEHTA_A.base()]);
        test_tengwar!(Quenya, "e" => [CARRIER_SHORT, TEHTA_E.base()]);
        test_tengwar!(Quenya, "i" => [CARRIER_SHORT, TEHTA_I.base()]);
        test_tengwar!(Quenya, "o" => [CARRIER_SHORT, TEHTA_O.base()]);
        test_tengwar!(Quenya, "u" => [CARRIER_SHORT, TEHTA_U.base()]);
        test_tengwar!(Quenya, "á" => [CARRIER_LONG, TEHTA_A.base()] as aa);
        test_tengwar!(Quenya, "é" => [CARRIER_LONG, TEHTA_E.base()] as ee);
        test_tengwar!(Quenya, "í" => [CARRIER_LONG, TEHTA_I.base()] as ii);
        test_tengwar!(Quenya, "ó" => [CARRIER_LONG, TEHTA_O.base()] as oo);
        test_tengwar!(Quenya, "ú" => [CARRIER_LONG, TEHTA_U.base()] as uu);
        test_tengwar!(Quenya, "aa" == aa);
        test_tengwar!(Quenya, "ee" == ee);
        test_tengwar!(Quenya, "ii" == ii);
        test_tengwar!(Quenya, "oo" == oo);
        test_tengwar!(Quenya, "uu" == uu);

        //  Test all vowels, after consonants.
        test_tengwar!(Quenya, "la" => [TENGWA_LAMBE, TEHTA_A.base()]);
        test_tengwar!(Quenya, "le" => [TENGWA_LAMBE, TEHTA_E.base()]);
        test_tengwar!(Quenya, "li" => [TENGWA_LAMBE, TEHTA_I.base()]);
        test_tengwar!(Quenya, "lo" => [TENGWA_LAMBE, TEHTA_O.base()]);
        test_tengwar!(Quenya, "lu" => [TENGWA_LAMBE, TEHTA_U.base()]);
        test_tengwar!(Quenya, "lá" => [TENGWA_LAMBE, pre_long!(TEHTA_A), TEHTA_A.long()] as laa);
        test_tengwar!(Quenya, "lé" => [TENGWA_LAMBE, pre_long!(TEHTA_E), TEHTA_E.long()] as lee);
        test_tengwar!(Quenya, "lí" => [TENGWA_LAMBE, pre_long!(TEHTA_I), TEHTA_I.long()] as lii);
        test_tengwar!(Quenya, "ló" => [TENGWA_LAMBE, pre_long!(TEHTA_O), TEHTA_O.long()] as loo);
        test_tengwar!(Quenya, "lú" => [TENGWA_LAMBE, pre_long!(TEHTA_U), TEHTA_U.long()] as luu);
        test_tengwar!(Quenya, "laa" == laa);
        test_tengwar!(Quenya, "lee" == lee);
        test_tengwar!(Quenya, "lii" == lii);
        test_tengwar!(Quenya, "loo" == loo);
        test_tengwar!(Quenya, "luu" == luu);
    }
}
