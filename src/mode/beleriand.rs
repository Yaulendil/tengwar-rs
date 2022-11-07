use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


/// Tehta indicating a long vowel.
const TEHTA_LONG: Tehta = Tehta::Double(DC_OVER_ACUTE_1);

const VOWEL_A: char = TENGWA_OSSE;
const VOWEL_E: char = TENGWA_YANTA;
const VOWEL_I: char = TENGWA_TELCO;
const VOWEL_O: char = TENGWA_ANNA;
const VOWEL_U: char = TENGWA_URE;
const VOWEL_Y: char = TENGWA_SILME_NUQ;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn, // Base
        ['d']           /**/ => TEMA_TINCO.double_dn, // Voiced
        ['þ']
        | ['θ']
        | ['t', 'h']    /**/ => TEMA_TINCO.single_up, // Fricative
        ['ð']
        | ['d', 'h']    /**/ => TEMA_TINCO.double_up, // Voiced Fricative
        ['n', 'n']      /**/ => TEMA_TINCO.double_sh,
        ['n']           /**/ => TEMA_TINCO.single_sh,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['p', 'h']
        | ['f']
        | ['φ']         /**/ => TEMA_PARMA.single_up,
        ['v']           /**/ => TEMA_PARMA.double_up,
        ['m', 'm']      /**/ => TEMA_PARMA.double_sh,
        ['m']           /**/ => TEMA_PARMA.single_sh,

        ['c']
        | ['k']         /**/ => TEMA_CALMA.single_dn,
        ['g']           /**/ => TEMA_CALMA.double_dn,
        ['c', 'h']
        | ['k', 'h']    /**/ => TEMA_CALMA.single_up,
        ['g', 'h']      /**/ => TEMA_CALMA.double_up,

        ['w']           /**/ => TEMA_QESSE.single_sh,

        //  Irregular
        ['r']           /**/ => TENGWA_ROMEN,
        // ['r', 'h']      /**/ => TENGWA_ARDA,
        ['l']           /**/ => TENGWA_LAMBE,
        // ['l', 'h']      /**/ => TENGWA_ALDA,
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
    // consonant_char(slice).map(|cons| Glyph::new_cons(cons, false))

    match consonant_char(slice) {
        Some(cons) => Some(Glyph::new_cons(cons, false)),
        None => None,
    }
}


pub const fn get_diphthong(slice: &[char]) -> Option<Glyph> {
    match slice {
        // ['a', 'e'] => Some(Glyph::new_both(VOWEL_A, TEHTA_CIRCUMFLEX)),
        // ['o', 'e'] => Some(Glyph::new_both(VOWEL_O, TEHTA_CIRCUMFLEX)),

        ['a', 'i'] => Some(Glyph::new_both(VOWEL_A, TEHTA_Y)),
        ['e', 'i'] => Some(Glyph::new_both(VOWEL_E, TEHTA_Y)),
        ['u', 'i'] => Some(Glyph::new_both(VOWEL_U, TEHTA_Y)),

        ['a', 'u']
        | ['a', 'w'] => Some(Glyph::new_cons(VOWEL_A, false).with_labial()),

        _ => None,
    }
}


pub const fn get_vowel(slice: &[char]) -> Option<Glyph> {
    Some(match slice {
        ['a'] | ['ä'] => Glyph::new_cons(VOWEL_A, false),
        ['e'] | ['ë'] => Glyph::new_cons(VOWEL_E, false),
        ['i'] | ['ï'] => Glyph::new_cons(VOWEL_I, false),
        ['o'] | ['ö'] => Glyph::new_cons(VOWEL_O, false),
        ['u'] | ['ü'] => Glyph::new_cons(VOWEL_U, false),
        ['y'] | ['ÿ'] => Glyph::new_cons(VOWEL_Y, false),

        ['á'] | ['â'] | ['a', 'a'] => Glyph::new_both(VOWEL_A, TEHTA_LONG),
        ['é'] | ['ê'] | ['e', 'e'] => Glyph::new_both(VOWEL_E, TEHTA_LONG),
        ['í'] | ['î'] | ['i', 'i'] => Glyph::new_both(VOWEL_I, TEHTA_LONG),
        ['ó'] | ['ô'] | ['o', 'o'] => Glyph::new_both(VOWEL_O, TEHTA_LONG),
        ['ú'] | ['û'] | ['u', 'u'] => Glyph::new_both(VOWEL_U, TEHTA_LONG),
        ['ý'] | ['ŷ'] | ['y', 'y'] => Glyph::new_both(VOWEL_Y, TEHTA_LONG),

        _ => { return None; }
    })
}


/// The Mode of Beleriand, developed in the First Age for writing Sindarin.
#[derive(Clone, Copy, Debug, Default)]
pub struct Beleriand {
    current: Option<Glyph>,
    previous: Option<Glyph>,
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

        if let ['\\', _, ..] = chunk {
            ParseAction::Escape
        } else if let Some(current) = &mut self.current {
            //  A tengwa is currently being constructed. Finish it.

            finish!(*current)
        } else {
            //  Try to find a new tengwa.

            //  Check for special cases.
            if let ['x'] = chunk {
                self.current = Some(Glyph::new_cons(TENGWA_SILME, false));
                self.previous = None;

                ParseAction::MatchedToken {
                    token: Token::Tengwa(Glyph::new_cons(TENGWA_CALMA, false)),
                    len: 1,
                }
            }

            //  Check for voiceless initials.
            else if ['l', 'h'] == chunk && !self.previous.is_some() {
                finish!(Glyph::new_cons(TENGWA_ALDA, false), 2)
            } else if ['r', 'h'] == chunk && !self.previous.is_some() {
                finish!(Glyph::new_cons(TENGWA_ARDA, false), 2)
            }

            //  Check for a consonant.
            else if let Some(new) = get_consonant(chunk) {
                finish!(new, chunk.len())
            } else {
                //  Check for a nazalized consonant.
                if let ['m', rest @ ..] | ['n', rest @ ..] = chunk {
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
                }

                //  Give up.
                else {
                    ParseAction::MatchedNone
                }
            }
        }
    }
}
