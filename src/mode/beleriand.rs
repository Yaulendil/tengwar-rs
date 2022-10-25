use crate::{characters::*, etc::find_integer, Rules, Token};
use std::borrow::Cow;


const MAX_CHUNK: usize = 3;

const TEHTA_LONG: Tehta = TEHTA_E;

const VOWEL_A: char = TENGWA_OSSE;
const VOWEL_E: char = TENGWA_YANTA;
const VOWEL_I: char = CARRIER_SHORT;
const VOWEL_O: char = TEMA_CALMA.single_sh;
const VOWEL_U: char = TENGWA_URE;
const VOWEL_Y: char = TENGWA_SILME_NUQ;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn, // Base
        ['d']           /**/ => TEMA_TINCO.double_dn, // Voiced
        ['þ']
        | ['t', 'h']    /**/ => TEMA_TINCO.single_up, // Fricative
        ['ð']
        | ['d', 'h']    /**/ => TEMA_TINCO.double_up, // Voiced Fricative
        ['n', 'n']      /**/ => TEMA_TINCO.double_sh,
        ['n']           /**/ => TEMA_TINCO.single_sh,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']           /**/ => TEMA_PARMA.double_dn,
        ['f']           /**/ => TEMA_PARMA.single_up,
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
pub struct Beleriand;


impl Rules for Beleriand {
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
        //     ($pat:pat) => { matches!(out.last(), Some(Token::Tengwa($pat))) };
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
            if let Some((number, size)) = find_integer::<isize>(line) {
                commit!();
                out.push(Token::String(Cow::Owned(int_12(number))));

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
                                    || cons == TEMA_CALMA.double_dn
                                ) && !current.labial {
                                    current.labial = true;
                                    advance!();
                                    continue 'next_slice;
                                }
                            }
                        }
                        _ => {}
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
                        //  Check for an X. This should not happen in Sindarin,
                        //      but just in case, make sure we can handle it,
                        //      because it needs to be treated as two different
                        //      characters. It must be hacked in.
                        &['x'] => {
                            out.push(Token::Tengwa(Glyph::new_cons(
                                TEMA_CALMA.single_dn,
                                false,
                            )));
                            tengwa = Some(Glyph::new_cons(TENGWA_SILME, false));

                            advance!();
                            continue 'next_slice;
                        }
                        &[only] => if let Some(punct) = punctuation(*only) {
                            //  Look for punctuation marks.
                            out.push(Token::Char(punct));

                            advance!(sub.len());
                            continue 'next_slice;
                        }
                        _ => {}
                    }

                    if !matches!(out.last(), Some(Token::Tengwa(Glyph { .. }))) {
                        //  Check for Alda and Arda, but ONLY at the beginning
                        //      of a word.
                        match sub {
                            ['l', 'h'] => {
                                tengwa = Some(Glyph::new_cons(TENGWA_ALDA, false));

                                advance!(sub.len());
                                continue 'next_slice;
                            }
                            ['r', 'h'] => {
                                tengwa = Some(Glyph::new_cons(TENGWA_ARDA, false));

                                advance!(sub.len());
                                continue 'next_slice;
                            }
                            _ => {}
                        }
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
                    else if let Some(vowel) = get_vowel(sub) {
                        tengwa = Some(vowel);

                        advance!(sub.len());
                        continue 'next_slice;
                    }

                    if let ['m', rest @ ..] | ['n', rest @ ..] = sub {
                        if let Some(new) = get_consonant(rest) {
                            tengwa = Some(new.with_nasal());

                            advance!(sub.len());
                            continue 'next_slice;
                        }
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
