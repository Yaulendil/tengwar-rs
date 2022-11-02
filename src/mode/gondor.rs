use crate::{characters::*, Rules, Token};
use crate::mode::{ParseAction, TengwarMode};


const MAX_CHUNK: usize = 3;

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
        ['f']           /**/ => TEMA_PARMA.single_up,
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
pub struct Gondor;


impl Rules for Gondor {
    fn tokens(input: impl AsRef<str>) -> Vec<Token> {
        let cvec: Vec<char> = input.as_ref().to_lowercase().chars().collect();
        let mut line: &[char] = cvec.as_slice();
        let mut out: Vec<Token> = Vec::new();
        let mut tengwa: Option<Glyph> = None;
        let mut sub: &[char];
        let mut len: usize;

        /// Move the working slice forward.
        macro_rules! advance {
            () => { line = &line[1..]; };
            ($n:expr) => { line = &line[$n..]; };
        }

        /// Finalize and push the current tengwa, if there is one.
        macro_rules! commit {
            () => { if let Some(mut g) = tengwa.take() {
                g.long_first = true;
                out.push(Token::Tengwa(g));
            } };
        }

        /// Pass the first `char` in the slice through to the output unchanged.
        macro_rules! pass {
            () => { out.push(Token::Char(line[0])); };
        }

        // /// Check whether the most recently committed `Token` is a tengwa that
        // ///     matches a given pattern.
        // macro_rules! prev {
        //     ($pat:pat) => { matches!( out.last(), Some(Token::Tengwa($pat)) ) };
        // }

        'next_slice:
        while !line.is_empty() {
            if let &[escape @ '\\', ignore, ..] = line {
                commit!();
                out.push(Token::Char(escape));
                out.push(Token::Char(ignore));

                advance!(2);
                continue 'next_slice;
            }

            //  Check first whether a number can be found at the beginning of
            //      the current line.
            if let Some((number, size)) = Numeral::parse(line) {
                commit!();
                out.push(Token::Number(number));

                advance!(size);
                continue 'next_slice;
            }

            len = MAX_CHUNK;

            'same_slice:
            while len > 0 {
                len = line.len().min(len);
                sub = &line[..len];

                //  There is a tengwa currently being constructed. Look for the
                //      next modifier.
                if let Some(current) = &mut tengwa {
                    /*------------------*/

                    //  Check for a special case.
                    match sub {
                        &['w'] => {
                            if let Some(cons) = current.cons {
                                if (cons == TEMA_TINCO.double_dn
                                    || cons == TEMA_QESSE.double_dn
                                ) && !current.labial {
                                    current.labial = true;
                                    advance!();
                                    continue 'next_slice;
                                }
                            }
                        }
                        &['s'] => {
                            //  If the current tengwa has a consonant, we can
                            //      apply a Silmë Rincë to it.
                            if current.cons.map(rince_valid) == Some(true) {
                                current.silme = true;
                                advance!();
                                continue 'next_slice;
                            }
                        }
                        &['x'] => {
                            //  This needs to be treated as if it were "cs".
                            if current.cons.is_some() {
                                commit!();
                            }

                            tengwa = Some(Glyph::new_cons(
                                TEMA_QESSE.single_dn, false,
                            ).with_silme());

                            advance!();
                            continue 'next_slice;
                        }
                        _ => {}
                    }

                    //  Look for a consonant, if we need one.
                    if current.cons.is_none() {
                        //  Look for a nasalized consonant first.
                        if let ['m', rest @ ..] | ['n', rest @ ..] = sub {
                            if let Some(new) = get_consonant(rest) {
                                current.cons = new.cons;
                                current.nasal = true;
                                current.labial = new.labial;
                                current.palatal = new.palatal;
                                current.long_cons = new.long_cons;

                                advance!(sub.len());
                                continue 'next_slice;
                            }
                        }

                        if let Some(new) = get_consonant(sub) {
                            current.cons = new.cons;
                            current.nasal = new.nasal;
                            current.labial = new.labial;
                            current.palatal = new.palatal;
                            current.long_cons = new.long_cons;

                            advance!(sub.len());
                            continue 'next_slice;
                        }
                    }

                    /*------------------*/

                    //  If nothing has been found, allow `len` to decrement.
                    len -= 1;

                    if len > 0 {
                        //  If it is still positive, repeat the same check over
                        //      a new subslice.
                        continue 'same_slice;
                    } else {
                        //  Otherwise, commit the current tengwa, reset it, and
                        //      start looking for a new one. No more changes can
                        //      be made to the current one.
                        commit!();
                        continue 'next_slice;
                    }
                }

                //  No currently active tengwa. Find a new one.
                else {
                    /*------------------*/

                    match &sub {
                        &['x'] => {
                            tengwa = Some(Glyph::new_cons(
                                TEMA_QESSE.single_dn, false,
                            ).with_silme());

                            advance!();
                            continue 'next_slice;
                        }
                        &[only] => if let Some(punct) = punctuation(*only) {
                            //  Look for punctuation marks.
                            out.push(Token::Char(punct));

                            advance!(sub.len());
                            continue 'next_slice;
                        }
                        &['m', rest @ ..] | &['n', rest @ ..] => {
                            if let Some(new) = get_consonant(rest) {
                                tengwa = Some(new.with_nasal());

                                advance!(sub.len());
                                continue 'next_slice;
                            }
                        }
                        _ => {}
                    }

                    //  Look for a consonant.
                    if let Some(new) = get_consonant(sub) {
                        tengwa = Some(new);

                        advance!(sub.len());
                        continue 'next_slice;
                    }

                    //  Look for a diphthong.
                    else if let Some(new) = get_diphthong(sub) {
                        tengwa = Some(new);

                        advance!(sub.len());
                        continue 'next_slice;
                    }

                    //  Look for a vowel.
                    else if let Some((vowel, long)) = get_vowel(sub) {
                        tengwa = Some(Glyph::new_vowel(vowel, long));

                        advance!(sub.len());
                        continue 'next_slice;
                    }

                    /*------------------*/

                    //  If nothing has been found, allow `len` to decrement.
                    len -= 1;

                    if len > 0 {
                        //  If it is still positive, repeat the same check over
                        //      a narrower part of the same slice.
                        continue 'same_slice;
                    } else {
                        //  Otherwise, pass the first character through to the
                        //      output, unaffected, and move on.

                        if let Some(Token::Tengwa(
                            Glyph { cons: Some(cons), .. }
                        )) = out.last_mut() {
                            //  ...But first, make sure the last Tengwa was not
                            //      a Rómen.
                            if *cons == TENGWA_ROMEN {
                                *cons = TEMA_TINCO.single_sh;
                            }
                        }

                        pass!();
                        advance!();
                        continue 'next_slice;
                    }
                }

                // unreachable!();
            }

            unreachable!();
        }

        commit!();

        //  Make sure the last Tengwa was not a Rómen.
        if let Some(Token::Tengwa(
            Glyph { cons: Some(cons), .. }
        )) = out.last_mut() {
            if *cons == TENGWA_ROMEN {
                *cons = TEMA_TINCO.single_sh;
            }
        }

        out
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Gondor2 {
    current: Option<Glyph>,
    previous: Option<Glyph>,
}

impl TengwarMode for Gondor2 {
    fn finalize(&self, token: &mut Token, next: Option<&Token>) {
        if let Token::Tengwa(glyph) = token {
            glyph.long_first = true;

            if glyph.cons == Some(TENGWA_ROMEN) {
                match next {
                    Some(Token::Tengwa(_)) => {}
                    _ => glyph.cons = Some(temar::ORE),
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
                        && (cons == TEMA_TINCO.double_dn
                        || cons == TEMA_QESSE.double_dn)
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

                    glyph.cons = Some(temar::CALMA);
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

                //  Check for a nasalized consonant.
                else {
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
                    token: Token::Tengwa(Glyph::new_cons(temar::CALMA, false)),
                    len: 1,
                };
            }

            //  Check for voiceless initials.
            else if ['l', 'h'] == chunk && !self.previous.is_some() {
                self.current = Some(Glyph::new_cons(TENGWA_ALDA, false));
                return ParseAction::MatchedPart(2);
                // return finish!(Glyph::new_cons(TENGWA_ALDA, false), 2);
            } else if ['r', 'h'] == chunk && !self.previous.is_some() {
                self.current = Some(Glyph::new_cons(TENGWA_ARDA, false));
                return ParseAction::MatchedPart(2);
                // return finish!(Glyph::new_cons(TENGWA_ARDA, false), 2);
            }

            //  Check for a nasalized consonant.
            else if let ['m' | 'n', rest @ ..] = chunk {
                if let Some(new) = get_consonant(rest) {
                    self.current = Some(new.with_nasal());
                    return ParseAction::MatchedPart(chunk.len());
                    // return finish!(new.with_nasal(), chunk.len());
                }
            }

            //  Check for any consonant.
            if let Some(glyph) = get_consonant(chunk) {
                self.current = Some(glyph);
                ParseAction::MatchedPart(chunk.len())
                // finish!(new, chunk.len())
            }

            //  Check for a diphthong.
            else if let Some(new) = get_diphthong(chunk) {
                self.current = Some(new);
                ParseAction::MatchedPart(chunk.len())
                // finish!(new, chunk.len())
            }

            //  Check for a single vowel.
            else if let Some((vowel, long)) = get_vowel(chunk) {
                self.current = Some(Glyph::new_vowel(vowel, long));
                ParseAction::MatchedPart(chunk.len())
                // finish!(Glyph::new_vowel(vowel, long), chunk.len())
            }

            //  Give up.
            else {
                ParseAction::MatchedNone
            }
        }
    }
}
