use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


const CARRIER_DIPH_E: char = TENGWA_YANTA;
const CARRIER_DIPH_I: char = TENGWA_ANNA;
const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']           /**/ => TEMA_TINCO.double_dn,
        ['t', 'h']
        | ['θ']
        | ['þ']         /**/ => TEMA_TINCO.single_up,
        ['d', 'h']
        | ['ð']         /**/ => TEMA_TINCO.double_up,
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TEMA_TINCO.single_sh,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['p', 'h']
        | ['φ']         /**/ => TEMA_PARMA.single_up,
        ['v']           /**/ => TEMA_PARMA.double_up,
        ['m']           /**/ => TEMA_PARMA.double_sh,

        ['c']
        | ['k']         /**/ => TEMA_QESSE.single_dn,
        ['g']           /**/ => TEMA_QESSE.double_dn,
        ['c', 'h']
        | ['k', 'h']    /**/ => TEMA_QESSE.single_up,
        ['g', 'h']      /**/ => TEMA_QESSE.double_up,
        ['ñ']
        | ['n', 'g']    /**/ => TEMA_QESSE.double_sh,
        ['w']           /**/ => TEMA_QESSE.single_sh,

        //  Irregular
        ['l']           /**/ => TENGWA_LAMBE,
        ['l', 'h']      /**/ => TENGWA_ALDA,
        ['r', 'h']      /**/ => TENGWA_ARDA,
        ['s']           /**/ => TENGWA_SILME,
        ['s', 's']
        | ['z']         /**/ => TENGWA_ESSE,

        ['h']           /**/ => TENGWA_HYARMEN,
        ['h', 'w']      /**/ => TENGWA_HWESTA_SINDARINWA,
        ['j']           /**/ => TENGWA_YANTA,

        _ => { return None; }
    })
}


const fn get_consonant(slice: &[char]) -> Option<Glyph> {
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
        ['a', 'e'] => Some(Glyph::new_both(CARRIER_DIPH_E, TEHTA_A)),
        ['o', 'e'] => Some(Glyph::new_both(CARRIER_DIPH_E, TEHTA_O)),

        ['a', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_A)),
        ['e', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_E)),
        ['u', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_U)),

        ['a', 'u']
        | ['a', 'w'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_A)),

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
        ['y'] | ['ÿ'] => Some((TEHTA_Y, false)),

        ['á'] | ['â'] | ['a', 'a'] => Some((TEHTA_A, true)),
        ['é'] | ['ê'] | ['e', 'e'] => Some((TEHTA_E, true)),
        ['í'] | ['î'] | ['i', 'i'] => Some((TEHTA_I, true)),
        ['ó'] | ['ô'] | ['o', 'o'] => Some((TEHTA_O, true)),
        ['ú'] | ['û'] | ['u', 'u'] => Some((TEHTA_U, true)),
        ['ý'] | ['ŷ'] | ['y', 'y'] => Some((TEHTA_Y, true)),

        _ => None,
    }
}


/// The Mode of Gondor, used in the Third Age for writing Sindarin.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gondor {
    current: Option<Glyph>,
}

impl Gondor {
    pub fn decide_f(next: &[char]) -> Glyph {
        let mut mode = Self::default();
        let mut is_final: bool = true;
        let mut n: usize = next.len();

        while 0 < n {
            match mode.process(&next[..n]) {
                ParseAction::MatchedNone => {
                    //  Next token is unknown.
                    n -= 1;
                }
                ParseAction::MatchedPart(_) => {
                    //  Next token is a tengwa.
                    is_final = false;
                    break;
                }
                ParseAction::MatchedToken {
                    token: Token::Tengwa(_),
                    ..
                } => {
                    //  Next token is a tengwa.
                    is_final = false;
                    break;
                }
                _ => {
                    //  Next token is NOT a tengwa.
                    is_final = true;
                    break;
                }
            }
        }

        let phonetic: &[char] = if is_final { &['v'] } else { &['p', 'h'] };
        consonant_char(phonetic).unwrap().into()
    }

    pub fn find_consonant(chunk: &[char], initial: bool) -> Option<(Glyph, usize)> {
        if initial {
            //  Check for voiceless initials.
            if let ['l', 'h'] = chunk {
                return Some((Glyph::from(TENGWA_ALDA), 2));
            } else if let ['r', 'h'] = chunk {
                return Some((Glyph::from(TENGWA_ARDA), 2));
            }
        }

        //  Check for a nasalized consonant.
        if let ['m' | 'n', rest @ ..] = chunk {
            if let Some(new) = get_consonant(rest) {
                return Some((new.with_nasal(), chunk.len()));
            }
        }

        if let ['x'] = chunk {
            Some((Glyph::from(TENGWA_CALMA).with_silme(), 1))
        }

        //  Check for a final F, which should be spelled with Ampa instead of
        //      Formen.
        else if let ['f', ahead @ ..] = chunk {
            Some((Self::decide_f(ahead), 1))
        }

        //  Check for any consonant.
        else if let Some(glyph) = get_consonant(chunk) {
            Some((glyph, chunk.len()))
        } else {
            None
        }
    }
}

impl TengwarMode for Gondor {
    fn finalize(&self, token: &mut Token, next: Option<&Token>) {
        if let Token::Tengwa(glyph) = token {
            glyph.long_first = true;

            if let Some(Token::Tengwa(_)) = next {
                glyph.replace_consonant(TENGWA_ORE, TENGWA_ROMEN);
            }
        }
    }

    fn finish_current(&mut self) -> Option<Token> {
        self.current.take().map(Token::Tengwa)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        macro_rules! finish {
            ($glyph:expr) => {finish!($glyph, 0)};
            ($glyph:expr, $len:expr) => {{
                let glyph = $glyph;
                self.current = None;

                ParseAction::MatchedToken {
                    token: Token::Tengwa(glyph),
                    len: $len,
                }
            }};
        }

        if let ['\\', _, ..] = chunk {
            ParseAction::Escape
        } else if let Some(current) = &mut self.current {
            //  A tengwa is currently being constructed. Try to continue it.

            if let Some(cons) = current.cons {
                //  Current tengwa already has a consonant. Look for something
                //      that would modify it.

                match chunk {
                    ['w'] if !current.labial
                        && (cons == TENGWA_ANDO || cons == TENGWA_UNGWE)
                    => {
                        //  TODO: Rediscover why this is done. Tecendil displays
                        //      the same behavior, but this is not listed in the
                        //      handbook or any offline notes. Why does it only
                        //      apply to these two tengwar?
                        current.labial = true;
                        ParseAction::MatchedPart(1)
                    }
                    ['s'] if !current.silme && rince_valid(cons) => {
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
                    _ => ParseAction::MatchedNone,
                }
            } else {
                //  Current tengwa does NOT have a consonant. Try to find one.

                if let Some((new, len)) = Self::find_consonant(chunk, false) {
                    current.integrate_consonant(new);
                    ParseAction::MatchedPart(len)
                } else {
                    ParseAction::MatchedNone
                }
            }
        } else {
            //  Try to find a new tengwa.

            //  Check for any consonant.
            if let Some((new, len)) = Self::find_consonant(chunk, true) {
                self.current = Some(new);
                ParseAction::MatchedPart(len)
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


#[test]
#[cfg(test)]
fn test_gondor() {
    test_tengwar!(Gondor, "axë" => [
        TENGWA_CALMA, TEHTA_A.base(), SA_RINCE, // ax
        CARRIER_SHORT, TEHTA_E.base(), // ë
    ]);

    test_tengwar!(Gondor, "ae" => [CARRIER_DIPH_E, TEHTA_A.base()]);
    test_tengwar!(Gondor, "oe" => [CARRIER_DIPH_E, TEHTA_O.base()]);
    test_tengwar!(Gondor, "ai" => [CARRIER_DIPH_I, TEHTA_A.base()]);
    test_tengwar!(Gondor, "ei" => [CARRIER_DIPH_I, TEHTA_E.base()]);
    test_tengwar!(Gondor, "ui" => [CARRIER_DIPH_I, TEHTA_U.base()]);
    test_tengwar!(Gondor, "au" => [CARRIER_DIPH_U, TEHTA_A.base()] as au);
    test_tengwar!(Gondor, "aw" == au);

    let edhellen = test_tengwar!(Gondor, "edhellen" => [
        TENGWA_ANTO, TEHTA_E.base(), // edh
        TENGWA_LAMBE, DC_UNDER_LINE_H, TEHTA_E.base(), // ell
        TENGWA_NUMEN, TEHTA_E.base(), // en
    ]);
    test_tengwar!(Gondor, "eðellen" == edhellen);
    test_tengwar!(Gondor, "EÐELLEN" == edhellen);
    test_tengwar!(Gondor, "edellen" != edhellen);
    test_tengwar!(Gondor, "eθellen" != edhellen);
    test_tengwar!(Gondor, "eþellen" != edhellen);
    test_tengwar!(Gondor, "ethellen" != edhellen);

    let andaith = test_tengwar!(Gondor, "andaith" => [
        TENGWA_ANDO, DC_OVER_LINE, TEHTA_A.base(), // and
        CARRIER_DIPH_I, TEHTA_A.base(), // ai
        TENGWA_THULE, // th
    ]);
    test_tengwar!(Gondor, "andaiθ" == andaith);
    test_tengwar!(Gondor, "ANDAIΘ" == andaith);
    test_tengwar!(Gondor, "andaiþ" == andaith);
    test_tengwar!(Gondor, "ANDAIÞ" == andaith);
    test_tengwar!(Gondor, "andait" != andaith);
    test_tengwar!(Gondor, "andaið" != andaith);
    test_tengwar!(Gondor, "andaidh" != andaith);

    //  Final F, after consonant.
    let parf = test_tengwar!(Gondor, "parf" => [
        TENGWA_PARMA, // p
        TENGWA_ROMEN, TEHTA_A.base(), // ar
        TENGWA_AMPA, // v
    ]);
    test_tengwar!(Gondor, "parv" == parf);
    test_tengwar!(Gondor, "parφ" != parf);
    test_tengwar!(Gondor, "parph" != parf);

    //  Final F, after vowel.
    let alaf = test_tengwar!(Gondor, "alaf" => [
        TENGWA_LAMBE, TEHTA_A.base(), // al
        TENGWA_AMPA, TEHTA_A.base(), // av
    ]);
    test_tengwar!(Gondor, "alav" == alaf);
    test_tengwar!(Gondor, "alaφ" != alaf);
    test_tengwar!(Gondor, "alaph" != alaf);

    //  Medial F, after consonant.
    let alfirin = test_tengwar!(Gondor, "alfirin" => [
        TENGWA_LAMBE, TEHTA_A.base(), // al
        TENGWA_FORMEN, // ph
        TENGWA_ROMEN, TEHTA_I.base(), // ir
        TENGWA_NUMEN, TEHTA_I.base(), // in
    ]);
    test_tengwar!(Gondor, "alphirin" == alfirin);
    test_tengwar!(Gondor, "alφirin" == alfirin);
    test_tengwar!(Gondor, "ALΦIRIN" == alfirin);
    test_tengwar!(Gondor, "alvirin" != alfirin);

    //  Medial F, after vowel.
    let aphadon = test_tengwar!(Gondor, "aphadon" => [
        TENGWA_FORMEN, TEHTA_A.base(), // aph
        TENGWA_ANDO, TEHTA_A.base(), // ad
        TENGWA_NUMEN, TEHTA_O.base(), // on
    ]);
    test_tengwar!(Gondor, "afadon" == aphadon);
    test_tengwar!(Gondor, "aφadon" == aphadon);
    test_tengwar!(Gondor, "AΦADON" == aphadon);
    test_tengwar!(Gondor, "avadon" != aphadon);
}
