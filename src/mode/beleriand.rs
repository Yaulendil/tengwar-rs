use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


/// Tengwa for a consonantal initial I.
pub const CONSONANT_I: char = TENGWA_ARA;

/// Tehta indicating a long vowel.
pub const ANDAITH: Tehta = Tehta::single(DC_OVER_ACUTE_1);

pub const VOWEL_A: char = TENGWA_OSSE;
pub const VOWEL_E: char = TENGWA_YANTA;
pub const VOWEL_I: char = TENGWA_TELCO;
pub const VOWEL_O: char = TENGWA_ANNA;
pub const VOWEL_U: char = TENGWA_URE;
pub const VOWEL_Y: char = TENGWA_SILME_NUQ;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Tincotéma.
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']           /**/ => TEMA_TINCO.double_dn,
        ['t', 'h']
        | ['θ'] | ['þ'] /**/ => TEMA_TINCO.single_up,
        ['d', 'h']
        | ['ð']         /**/ => TEMA_TINCO.double_up,
        ['n', 'n']      /**/ => TEMA_TINCO.double_sh,
        ['n']           /**/ => TEMA_TINCO.single_sh,

        //  Parmatéma.
        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['p', 'h']
        | ['φ']         /**/ => TEMA_PARMA.single_up,
        ['v']           /**/ => TEMA_PARMA.double_up,
        ['m', 'm']      /**/ => TEMA_PARMA.double_sh,
        ['m']           /**/ => TEMA_PARMA.single_sh,

        //  Calmatéma.
        ['c'] | ['k']   /**/ => TEMA_CALMA.single_dn,
        ['g']           /**/ => TEMA_CALMA.double_dn,
        ['c', 'h']
        | ['k', 'h']    /**/ => TEMA_CALMA.single_up,
        ['g', 'h']      /**/ => TEMA_CALMA.double_up,
        ['n', 'g']
        | ['ñ']         /**/ => TEMA_CALMA.double_sh,
        // []           /**/ => TEMA_CALMA.single_sh, // Vowel O.

        //  Qessetéma.
        // []           /**/ => TEMA_QESSE.single_dn,
        // []           /**/ => TEMA_QESSE.double_dn,
        // []           /**/ => TEMA_QESSE.single_up,
        // []           /**/ => TEMA_QESSE.double_up,
        // []           /**/ => TEMA_QESSE.double_sh,
        ['w']           /**/ => TEMA_QESSE.single_sh,

        //  Irregulars.
        ['l']           /**/ => TENGWA_LAMBE,
        ['r']           /**/ => TENGWA_ROMEN,
        // ['l', 'h']      /**/ => TENGWA_ALDA, // NOTE: Only for initials.
        // ['r', 'h']      /**/ => TENGWA_ARDA,
        ['m', 'h']      /**/ => TENGWA_VALA_HOOKED,
        ['s']           /**/ => TENGWA_SILME,
        ['s', 's']
        | ['z']         /**/ => TENGWA_ESSE,

        ['h']           /**/ => TENGWA_HYARMEN,
        ['h', 'w']      /**/ => TENGWA_HWESTA_SINDARINWA,
        ['j']           /**/ => CONSONANT_I,

        _ => { return None; }
    })
}


const fn get_consonant(slice: &[char]) -> Option<Glyph> {
    match consonant_char(slice) {
        Some(cons) => Some(Glyph::new_base(cons)),
        None => None,
    }
}


pub const fn get_diphthong(slice: &[char]) -> Option<Glyph> {
    match slice {
        // ['a', 'e'] => Some(Glyph::new_both(VOWEL_A, TEHTA_YANTA)),
        // ['o', 'e'] => Some(Glyph::new_both(VOWEL_O, TEHTA_YANTA)),

        ['a', 'i'] => Some(Glyph::new_both(VOWEL_A, TEHTA_Y)),
        ['e', 'i'] => Some(Glyph::new_both(VOWEL_E, TEHTA_Y)),
        ['u', 'i'] => Some(Glyph::new_both(VOWEL_U, TEHTA_Y)),

        ['a', 'u']
        | ['a', 'w'] => Some(Glyph::new_base(VOWEL_A).with_labial()),

        _ => None,
    }
}


pub const fn get_vowel(slice: &[char]) -> Option<Glyph> {
    Some(match slice {
        ['a'] | ['ä'] => Glyph::new_base(VOWEL_A),
        ['e'] | ['ë'] => Glyph::new_base(VOWEL_E),
        ['i'] | ['ï'] => Glyph::new_base(VOWEL_I),
        ['o'] | ['ö'] => Glyph::new_base(VOWEL_O),
        ['u'] | ['ü'] => Glyph::new_base(VOWEL_U),
        ['y'] | ['ÿ'] => Glyph::new_base(VOWEL_Y),

        ['á'] | ['â'] | ['a', 'a'] => Glyph::new_both(VOWEL_A, ANDAITH),
        ['é'] | ['ê'] | ['e', 'e'] => Glyph::new_both(VOWEL_E, ANDAITH),
        ['í'] | ['î'] | ['i', 'i'] => Glyph::new_both(VOWEL_I, ANDAITH),
        ['ó'] | ['ô'] | ['o', 'o'] => Glyph::new_both(VOWEL_O, ANDAITH),
        ['ú'] | ['û'] | ['u', 'u'] => Glyph::new_both(VOWEL_U, ANDAITH),
        ['ý'] | ['ŷ'] | ['y', 'y'] => Glyph::new_both(VOWEL_Y, ANDAITH),

        _ => { return None; }
    })
}


/// The Mode of Beleriand, developed in the First Age for writing Sindarin.
#[derive(Clone, Copy, Debug, Default)]
pub struct Beleriand {
    current: Option<Glyph>,
    previous: Option<Glyph>,
}

impl Beleriand {
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
}

impl TengwarMode for Beleriand {
    fn finish_current(&mut self) -> Option<Token> {
        self.previous = self.current.take();
        self.previous.map(Token::Tengwa)
    }

    //  TODO: Completely review this code; A significant mistake was found. It
    //      should probably be rebuilt from the ground up, directly from specs.
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
            //  A glyph is currently being constructed, but this mode does not
            //      have any modifications. Finish it.
            finish!(*current)
        } else {
            //  Try to find a new glyph.

            //  Check for special cases.
            if let ['x'] = chunk {
                self.current = Some(Glyph::new_base(TENGWA_SILME));
                self.previous = None;

                ParseAction::MatchedToken {
                    token: Token::Tengwa(Glyph::new_base(TENGWA_CALMA)),
                    len: 1,
                }
            }

            //  Check for voiceless initials.
            else if ['l', 'h'] == chunk && !self.previous.is_some() {
                finish!(Glyph::new_base(TENGWA_ALDA), 2)
            } else if ['r', 'h'] == chunk && !self.previous.is_some() {
                finish!(Glyph::new_base(TENGWA_ARDA), 2)
            }

            //  Check for F, and decide whether it is final.
            else if let ['f', ahead @ ..]  = chunk {
                finish!(Self::decide_f(ahead), 1)
            }

            //  Check for a consonant.
            else if let Some(new) = get_consonant(chunk) {
                finish!(new, chunk.len())
            } else {
                //  Check for a nazalized consonant.
                if let ['m' | 'n', rest @ ..] = chunk {
                    if let Some(new) = get_consonant(rest) {
                        return finish!(new.with_nasal(), chunk.len());
                    }
                }

                //  Check for a diphthong.
                if let Some(new) = get_diphthong(chunk) {
                    finish!(new, chunk.len())
                }

                //  Check for a single vowel.
                else if let Some(glyph) = get_vowel(chunk) {
                    finish!(glyph, chunk.len())
                } else {
                    if initial {
                        if let ['i', rest @ ..] = chunk {
                            let first = ParseAction::MatchedToken {
                                token: Token::Tengwa(CONSONANT_I.into()),
                                len: chunk.len(),
                            };

                            if let Some(new) = get_diphthong(rest) {
                                self.current = Some(new);
                                return first;
                            } else if let Some(new) = get_vowel(rest) {
                                self.current = Some(new);
                                return first;
                            }
                        }
                    }

                    ParseAction::MatchedNone
                }
            }
        }
    }
}
