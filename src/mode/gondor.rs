use crate::{characters::*, Token};
use crate::mode::{ParseAction, TengwarMode};


const CARRIER_DIPH_E: char = TENGWA_YANTA;
const CARRIER_DIPH_I: char = TEMA_CALMA.single_sh;
const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn, // Base
        ['d']           /**/ => TEMA_TINCO.double_dn, // Voiced
        ['þ']
        | ['t', 'h']    /**/ => TEMA_TINCO.single_up, // Fricative
        ['ð']
        | ['d', 'h']    /**/ => TEMA_TINCO.double_up, // Voiced Fricative
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TENGWA_ROMEN,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['p', 'h']
        | ['f']         /**/ => TEMA_PARMA.single_up,
        ['v']           /**/ => TEMA_PARMA.double_up,
        ['m']           /**/ => TEMA_PARMA.double_sh,
        // ['v']           /**/ => TEMA_PARMA.single_sh,

        // ['y']           /**/ => TEMA_CALMA.single_sh,

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
        ['r', 'h']      /**/ => TENGWA_ARDA,
        ['l']           /**/ => TENGWA_LAMBE,
        ['l', 'h']      /**/ => TENGWA_ALDA,
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
    previous: Option<Glyph>,
}

impl Gondor {
    fn decide_f(next: &[char]) -> Glyph {
        // let unvoiced = Glyph::new_cons(TENGWA_FORMEN, false);
        let unvoiced = get_consonant(&['f']).unwrap();

        let mut mode = Self::default();
        mode.previous = Some(unvoiced);

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

        if is_final {
            // Glyph::new_cons(TENGWA_AMPA, false)
            get_consonant(&['v']).unwrap()
        } else {
            unvoiced
        }
    }
}

impl TengwarMode for Gondor {
    fn finalize(&self, token: &mut Token, next: Option<&Token>) {
        if let Token::Tengwa(glyph) = token {
            glyph.long_first = true;

            if glyph.cons == Some(TENGWA_ROMEN) {
                match next {
                    Some(Token::Tengwa(_)) => {}
                    _ => glyph.cons = Some(TENGWA_ORE),
                }
            }
        }
    }

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

        if let ['\\', _, ..] = chunk {
            ParseAction::Escape
        } else if let Some(current) = &mut self.current {
            //  A tengwa is currently being constructed. Try to continue it.

            if let Some(cons) = current.cons {
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
                    ['s', 's'] => finish!(*current),
                    _ => ParseAction::MatchedNone,
                }
            } else {
                //  Check for special case.
                if let ['x'] = chunk {
                    let mut glyph = *current;

                    glyph.cons = Some(TENGWA_CALMA);
                    self.current = Some(Glyph::new_cons(TENGWA_SILME, false));
                    self.previous = Some(glyph);

                    ParseAction::MatchedToken {
                        token: Token::Tengwa(glyph),
                        len: 1,
                    }
                }

                //  Check for voiceless initials.
                else if ['l', 'h'] == chunk && !self.previous.is_some() {
                    current.cons = Some(TENGWA_ALDA);
                    ParseAction::MatchedPart(2)
                } else if ['r', 'h'] == chunk && !self.previous.is_some() {
                    current.cons = Some(TENGWA_ARDA);
                    ParseAction::MatchedPart(2)
                }

                else {
                    //  Check for a nasalized consonant.
                    if let ['m' | 'n', rest @ ..] = chunk {
                        if let Some(new) = get_consonant(rest) {
                            current.cons = new.cons;
                            current.nasal = true;
                            current.labial = new.labial;
                            current.palatal = new.palatal;
                            current.long_cons = new.long_cons;

                            return ParseAction::MatchedPart(chunk.len());
                        }
                    }

                    //  Check for a final F, which should be spelled with Ampa
                    //      instead of Formen.
                    else if let ['f', ahead @ ..] = chunk {
                        let new = Self::decide_f(ahead);

                        current.cons = new.cons;
                        current.nasal = new.nasal;
                        current.labial = new.labial;
                        current.palatal = new.palatal;
                        current.long_cons = new.long_cons;

                        return ParseAction::MatchedPart(1);
                    }

                    //  Check for a regular consonant.
                    if let Some(new) = get_consonant(chunk) {
                        current.cons = new.cons;
                        current.nasal = new.nasal;
                        current.labial = new.labial;
                        current.palatal = new.palatal;
                        current.long_cons = new.long_cons;

                        ParseAction::MatchedPart(chunk.len())
                    } else {
                        ParseAction::MatchedNone
                    }
                }
            }
        } else {
            //  Try to find a new tengwa.

            //  Check for special case.
            if let ['x'] = chunk {
                self.current = Some(Glyph::new_cons(TENGWA_SILME, false));
                self.previous = None;

                return ParseAction::MatchedToken {
                    token: Token::Tengwa(Glyph::new_cons(TENGWA_CALMA, false)),
                    len: 1,
                };
            }

            //  Check for voiceless initials.
            else if ['l', 'h'] == chunk && !self.previous.is_some() {
                self.current = Some(Glyph::new_cons(TENGWA_ALDA, false));
                return ParseAction::MatchedPart(2);
            } else if ['r', 'h'] == chunk && !self.previous.is_some() {
                self.current = Some(Glyph::new_cons(TENGWA_ARDA, false));
                return ParseAction::MatchedPart(2);
            }

            //  Check for a nasalized consonant.
            else if let ['m' | 'n', rest @ ..] = chunk {
                if let Some(new) = get_consonant(rest) {
                    self.current = Some(new.with_nasal());
                    return ParseAction::MatchedPart(chunk.len());
                }
            }

            //  Check for a final F, which should be spelled with Ampa
            //      instead of Formen.
            else if let ['f', ahead @ ..] = chunk {
                self.current = Some(Self::decide_f(ahead));
                return ParseAction::MatchedPart(1);
            }

            //  Check for any consonant.
            if let Some(glyph) = get_consonant(chunk) {
                self.current = Some(glyph);
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
            }

            //  Give up.
            else {
                ParseAction::MatchedNone
            }
        }
    }
}


#[test]
#[cfg(test)]
fn test_gondor() {
    let edhellen = test_tengwar!(Gondor, "edhellen" => [
        TENGWA_ANTO, TEHTA_E.short(), // edh
        TENGWA_LAMBE, DC_UNDER_LINE_H, TEHTA_E.short(), // ell
        TENGWA_NUMEN, TEHTA_E.short(), // en
    ]);
    test_tengwar!(Gondor, "eðellen" == edhellen);
    test_tengwar!(Gondor, "edellen" != edhellen);
    test_tengwar!(Gondor, "eþellen" != edhellen);
    test_tengwar!(Gondor, "ethellen" != edhellen);

    let andaith = test_tengwar!(Gondor, "andaith" => [
        TENGWA_ANDO, DC_OVER_LINE, TEHTA_A.short(), // and
        CARRIER_DIPH_I, TEHTA_A.short(), // ai
        TENGWA_THULE, // th
    ]);
    test_tengwar!(Gondor, "andaiþ" == andaith);
    test_tengwar!(Gondor, "andait" != andaith);
    test_tengwar!(Gondor, "andaið" != andaith);
    test_tengwar!(Gondor, "andaidh" != andaith);

    //  Final F, after consonant.
    let parf = test_tengwar!(Gondor, "parf" => [
        TENGWA_PARMA, // p
        TENGWA_ROMEN, TEHTA_A.short(), // ar
        TENGWA_AMPA, // f (final)
    ]);
    test_tengwar!(Gondor, "parv" == parf);
    test_tengwar!(Gondor, "parph" != parf);

    //  Final F, after vowel.
    let alaf = test_tengwar!(Gondor, "alaf" => [
        TENGWA_LAMBE, TEHTA_A.short(), // al
        TENGWA_AMPA, TEHTA_A.short(), // af (final)
    ]);
    test_tengwar!(Gondor, "alav" == alaf);
    test_tengwar!(Gondor, "alaph" != alaf);

    //  Medial F, after consonant.
    let alfirin = test_tengwar!(Gondor, "alfirin" => [
        TENGWA_LAMBE, TEHTA_A.short(), // al
        TENGWA_FORMEN, // f (medial)
        TENGWA_ROMEN, TEHTA_I.short(), // ir
        TENGWA_NUMEN, TEHTA_I.short(), // in
    ]);
    test_tengwar!(Gondor, "alphirin" == alfirin);
    test_tengwar!(Gondor, "alvirin" != alfirin);

    //  Medial F, after vowel.
    let aphadon = test_tengwar!(Gondor, "aphadon" => [
        TENGWA_FORMEN, TEHTA_A.short(), // af (medial)
        TENGWA_ANDO, TEHTA_A.short(), // ad
        TENGWA_NUMEN, TEHTA_O.short(), // on
    ]);
    test_tengwar!(Gondor, "afadon" == aphadon);
    test_tengwar!(Gondor, "avadon" != aphadon);
}
