use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


/// Tengwa for a consonantal initial I.
pub const CONSONANT_I: char = TENGWA_YANTA;

pub const CARRIER_DIPH_E: char = TENGWA_YANTA;
pub const CARRIER_DIPH_I: char = TENGWA_ANNA;
pub const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Tincotéma.
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']           /**/ => TEMA_TINCO.double_dn,
        ['t', 'h']
        | ['θ'] | ['þ'] /**/ => TEMA_TINCO.single_up,
        ['d', 'h']
        | ['ð']         /**/ => TEMA_TINCO.double_up,
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TEMA_TINCO.single_sh,

        //  Parmatéma.
        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['p', 'h']
        | ['φ']         /**/ => TEMA_PARMA.single_up,
        ['v']           /**/ => TEMA_PARMA.double_up,
        ['m']           /**/ => TEMA_PARMA.double_sh,
        // []           /**/ => TEMA_PARMA.single_sh,

        //  Calmatéma.
        // []           /**/ => TEMA_CALMA.single_dn,
        // []           /**/ => TEMA_CALMA.double_dn,
        // []           /**/ => TEMA_CALMA.single_up,
        // []           /**/ => TEMA_CALMA.double_up,
        // []           /**/ => TEMA_CALMA.double_sh,
        // []           /**/ => TEMA_CALMA.single_sh, // Diphthongs of I.

        //  Qessetéma.
        ['c'] | ['k']   /**/ => TEMA_QESSE.single_dn,
        ['g']           /**/ => TEMA_QESSE.double_dn,
        ['c', 'h']
        | ['k', 'h']    /**/ => TEMA_QESSE.single_up,
        ['g', 'h']      /**/ => TEMA_QESSE.double_up,
        ['n', 'g']
        | ['ñ']         /**/ => TEMA_QESSE.double_sh,
        ['w']           /**/ => TEMA_QESSE.single_sh,

        //  Irregulars.
        ['l']           /**/ => TENGWA_LAMBE,
        // ['l', 'h']      /**/ => TENGWA_ALDA, // NOTE: Only for initials.
        // ['r', 'h']      /**/ => TENGWA_ARDA,
        ['m', 'h']      /**/ => TENGWA_MALTA_HOOKED,
        ['s']           /**/ => TENGWA_SILME,
        ['s', 's']
        | ['z']         /**/ => TENGWA_ESSE,

        ['h']           /**/ => TENGWA_HYARMEN,
        ['h', 'w']      /**/ => TENGWA_HWESTA_SINDARINWA,
        ['j']           /**/ => CONSONANT_I,

        _ => { return None; }
    })
}


pub const fn get_consonant(slice: &[char]) -> Option<Glyph> {
    match consonant_char(slice) {
        Some(cons) => Some(Glyph::new_base(cons)),
        None => match slice {
            &[a, b] if a == b => match consonant_char(&[a]) {
                Some(cons) => Some(Glyph::new_base(cons).with_underline(true)),
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


pub const fn get_vowel(slice: &[char]) -> Option<Glyph> {
    match slice {
        ['a'] | ['ä'] => Some(Glyph::new_tehta(TEHTA_A)),
        ['e'] | ['ë'] => Some(Glyph::new_tehta(TEHTA_E)),
        ['i'] | ['ï'] => Some(Glyph::new_tehta(TEHTA_I)),
        ['o'] | ['ö'] => Some(Glyph::new_tehta(TEHTA_O)),
        ['u'] | ['ü'] => Some(Glyph::new_tehta(TEHTA_U)),
        ['y'] | ['ÿ'] => Some(Glyph::new_tehta(TEHTA_Y)),

        ['á'] | ['â'] | ['a', 'a'] => Some(Glyph::new_tehta_alt(TEHTA_A)),
        ['é'] | ['ê'] | ['e', 'e'] => Some(Glyph::new_tehta_alt(TEHTA_E)),
        ['í'] | ['î'] | ['i', 'i'] => Some(Glyph::new_tehta_alt(TEHTA_I)),
        ['ó'] | ['ô'] | ['o', 'o'] => Some(Glyph::new_tehta_alt(TEHTA_O)),
        ['ú'] | ['û'] | ['u', 'u'] => Some(Glyph::new_tehta_alt(TEHTA_U)),
        ['ý'] | ['ŷ'] | ['y', 'y'] => Some(Glyph::new_tehta_alt(TEHTA_Y)),

        _ => None,
    }
}


pub const fn get_vowel_either(slice: &[char]) -> Option<Glyph> {
    if let Some(glyph) = get_diphthong(slice) {
        Some(glyph)
    } else if let Some(glyph) = get_vowel(slice) {
        Some(glyph)
    } else {
        None
    }
}


/// The Mode of Gondor, used in the Third Age for writing Sindarin.
#[derive(Clone, Copy, Debug, Default)]
pub struct Gondor {
    current: Option<Glyph>,
    previous: Option<Glyph>,
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
                    //  Next token is a glyph.
                    is_final = false;
                    break;
                }
                ParseAction::MatchedToken { token: Token::Glyph(_), .. } => {
                    //  Next token is a glyph.
                    is_final = false;
                    break;
                }
                _ => {
                    //  Next token is NOT a glyph.
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
                return Some((Glyph::new_base(TENGWA_ALDA), 2));
            } else if let ['r', 'h'] = chunk {
                return Some((Glyph::new_base(TENGWA_ARDA), 2));
            }
        }

        //  Check for a nasalized consonant.
        if let ['m' | 'n', rest @ ..] = chunk {
            //  ...but NOT for a nasalized H.
            if chunk != ['m', 'h'] {
                if let Some(new) = get_consonant(rest) {
                    return Some((new.with_nasal(true), chunk.len()));
                }
            }
        }

        if let ['x'] = chunk {
            Some((Glyph::new_base(TENGWA_CALMA).with_rince(true), 1))
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
        if let Token::Glyph(glyph) = token {
            glyph.tehta_first = true;

            if let Some(Token::Glyph(_)) = next {
                glyph.replace_base(TENGWA_ORE, TENGWA_ROMEN);
            }
        }
    }

    fn finish_current(&mut self) -> Option<Token> {
        self.previous = self.current.take();
        self.previous.map(Token::Glyph)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        macro_rules! finish {
            ($glyph:expr) => {finish!($glyph, 0)};
            ($glyph:expr, $len:expr) => {{
                let glyph = $glyph;

                self.current = None;
                self.previous = Some(glyph);

                ParseAction::MatchedToken {
                    token: Token::Glyph(glyph),
                    len: $len,
                }
            }};
        }

        let initial: bool = self.previous.is_none();

        if let [ESC, ESC_NOP, ..] = chunk {
            self.previous = None;
            ParseAction::matched_opt(self.current.take().map(Token::Glyph), 2)
        } else if let [ESC, _, ..] = chunk {
            ParseAction::ESC_BACKSLASH
        } else if let Some(current) = &mut self.current {
            //  A glyph is currently being constructed. Try to continue it.

            if let Some(base) = current.base {
                //  Current glyph already has a base tengwa. Look for something
                //      that would modify it.
                match chunk {
                    ['w'] if !current.labial
                        && (base == TENGWA_ANDO || base == TENGWA_UNGWE)
                    => {
                        //  TODO: Rediscover why this is done. Tecendil displays
                        //      the same behavior, but this is not listed in the
                        //      handbook or any offline notes. Why does it only
                        //      apply to these two tengwar?
                        current.labial = true;
                        ParseAction::MatchedPart(1)
                    }
                    ['s' | 'z'] if current.can_take_rince() => {
                        current.rince = true;
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
                //  Current glyph does NOT have a base tengwa. Try to find one.
                if let Some((new, len)) = Self::find_consonant(chunk, false) {
                    current.integrate_consonant(new);
                    ParseAction::MatchedPart(len)
                } else {
                    ParseAction::MatchedNone
                }
            }
        } else {
            //  Try to find a new glyph.

            //  Check for any consonant.
            if let Some((new, len)) = Self::find_consonant(chunk, true) {
                self.current = Some(new);
                ParseAction::MatchedPart(len)
            }

            //  Check for a vowel or diphthong.
            else if let Some(new) = get_vowel_either(chunk) {
                self.current = Some(new);
                ParseAction::MatchedPart(chunk.len())
            } else {
                //  An initial I, followed by a vowel, acts as a consonant.
                if initial {
                    if let ['i', rest @ ..] = chunk {
                        if let Some(new) = get_vowel_either(rest) {
                            self.current = Some(new);

                            return ParseAction::MatchedToken {
                                token: Token::Glyph(CONSONANT_I.into()),
                                len: chunk.len(),
                            };
                        }
                    }
                }

                ParseAction::MatchedNone
            }
        }
    }
}
