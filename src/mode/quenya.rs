use crate::{characters::*, Rules, Token};
use super::{ParseAction, TengwarMode};


const MAX_CHUNK: usize = 3;

const CARRIER_DIPH_I: char = TENGWA_YANTA;
const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Regular
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']
        | ['n', 'd']    /**/ => TEMA_TINCO.double_dn,
        ['þ']
        | ['t', 'h']    /**/ => TEMA_TINCO.single_up,
        ['n', 't']      /**/ => TEMA_TINCO.double_up,
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TEMA_TINCO.single_sh,

        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']
        | ['m', 'b']    /**/ => TEMA_PARMA.double_dn,
        ['f']           /**/ => TEMA_PARMA.single_up,
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
pub struct Quenya;


impl Rules for Quenya {
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
            () => { if let Some(g) = tengwa.take() {
                out.push(Token::Tengwa(g));
            } };
        }

        /// Pass the first `char` in the slice through to the output unchanged.
        macro_rules! pass {() => {out.push(Token::Char(line[0]))}}

        /// Check whether the most recently committed `Token` is a tengwa that
        ///     matches a given pattern.
        macro_rules! prev {
            ($pat:pat) => { matches!( out.last(), Some(Token::Tengwa($pat)) ) };
        }

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
                        &['s'] => {
                            //  If the current tengwa has a consonant with no
                            //      vowel, we can apply a Silmë Rincë to it.
                            if let Some(base) = &current.cons {
                                if rince_valid(*base) && current.vowel.is_none() {
                                    current.silme = true;
                                    advance!();
                                    continue 'next_slice;
                                }
                            }

                            commit!();
                            continue 'same_slice;
                        }
                        &['s', 's'] => {
                            //  We cannot apply a Rincë for Essë. Commit this
                            //      tengwa and then try for a new one.
                            commit!();
                            continue 'same_slice;
                        }
                        &['x'] => {
                            //  This needs to be treated as if it were "cs".
                            commit!();

                            tengwa = Some(Glyph::new_cons(
                                TEMA_CALMA.single_dn, false,
                            ).with_silme());

                            advance!();
                            continue 'next_slice;
                        }
                        _ => {}
                    }

                    //  Look for a vowel, if we need one.
                    if current.vowel.is_none() {
                        if sub == ['y'] {
                            current.palatal = true;

                            advance!();
                            continue 'next_slice;
                        }

                        //  In a cluster of "hl" or "hr", the H is transcribed
                        //      as Halla instead of Hyarmen.
                        else if current.cons == Some(TENGWA_HYARMEN)
                            && matches!(sub, &['l', ..] | &['r', ..])
                        {
                            current.cons = Some(TENGWA_HALLA);
                            commit!();
                            continue 'same_slice;
                        }

                        //  If there is a diphthong, we need to commit the
                        //      current tengwa early, so that it is not misread
                        //      as a normal vowel.
                        else if get_diphthong(sub).is_some() {
                            //  If a vowel sound follows Órë, it turns to Rómen.
                            if current.cons == Some(TEMA_TINCO.single_sh)
                                && !current.silme
                            {
                                current.cons = Some(TENGWA_ROMEN);
                            }

                            commit!();
                            continue 'same_slice;
                        }

                        //  Otherwise, we are free to check for a normal vowel.
                        else if let Some((vowel, long)) = get_vowel(sub) {
                            current.vowel = Some(vowel);
                            current.long_vowel = long;

                            //  If a vowel sound follows Órë, it turns to Rómen.
                            if current.cons == Some(TEMA_TINCO.single_sh)
                                && !current.silme
                            {
                                current.cons = Some(TENGWA_ROMEN);
                            }

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
                                TEMA_CALMA.single_dn, false,
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
                        &['y', _, ..] => {
                            //  If we have no consonant, but the next character
                            //      is `Y`, the next consonant can ONLY be `Y`.
                            //      Cheat slightly to speed the process along.
                            len = 1;
                            sub = &sub[..1];
                        }
                        _ => {}
                    }

                    //  Look for a consonant.
                    if let Some(mut new) = get_consonant(sub) {
                        if sub == ['y'] {
                            new.palatal = true;
                        }

                        //  This letter is Medial or Final.
                        if prev!(..) {
                            //  Medial H is represented by Aha, not Hyarmen.
                            if new.cons == Some(TENGWA_HYARMEN) {
                                new.cons = Some(TEMA_CALMA.single_up);
                            }
                        }

                        //  This letter is Initial.
                        else {
                            //  Initial NGW is represented by Ñwalmë, not Ungwë.
                            if new.cons == Some(TEMA_QESSE.double_dn) {
                                new.cons = Some(TEMA_QESSE.double_sh);
                            }
                        }

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
        out
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Quenya2 {
    current: Option<Glyph>,
    previous: Option<Glyph>,
}

impl Quenya2 {
    // fn begin_new(&mut self, new: Glyph) -> &mut Glyph {
    //     self.commit();
    //     self.current.insert(new)
    // }

    // const fn prev_valid(&self) -> bool {
    //     self.previous.is_some()
    // }
}

impl TengwarMode for Quenya2 {
    fn finish_current(&mut self) -> Option<Token> {
        self.previous = self.current.take();
        self.previous.map(Token::Tengwa)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        if let Some(_current) = &mut self.current {
            //  A tengwa is currently being constructed. Try to continue it.

            todo!()
        } else {
            //  Try to find a new tengwa.

            if let Some(new) = get_consonant(chunk) {
                let len = chunk.len();

                self.previous = self.current.take();
                let new = self.current.insert(new);

                if chunk == ['y'] {
                    new.palatal = true;
                }

                if self.previous.is_some() {
                    //  This letter is Medial or Final.
                    if new.cons == Some(TENGWA_HYARMEN) {
                        new.cons = Some(TEMA_CALMA.single_up);
                    }

                    //  This letter is Initial.
                    else {
                        //  Initial NGW is represented by Ñwalmë, not Ungwë.
                        if new.cons == Some(TEMA_QESSE.double_dn) {
                            new.cons = Some(TEMA_QESSE.double_sh);
                        }
                    }
                }

                ParseAction::MatchedPart(len)
            } else {
                ParseAction::MatchedNone
            }
        }
    }
}
