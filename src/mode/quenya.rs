use crate::{characters::*, Token};
use super::{ParseAction, TengwarMode};


pub const CARRIER_DIPH_I: char = TENGWA_YANTA;
pub const CARRIER_DIPH_U: char = TENGWA_URE;


pub const fn consonant_char(slice: &[char]) -> Option<char> {
    Some(match slice {
        //  Tincotéma.
        ['t']           /**/ => TEMA_TINCO.single_dn,
        ['d']
        | ['n', 'd']    /**/ => TEMA_TINCO.double_dn,
        ['t', 'h']
        | ['þ'] | ['θ'] /**/ => TEMA_TINCO.single_up,
        ['n', 't']      /**/ => TEMA_TINCO.double_up,
        ['n']           /**/ => TEMA_TINCO.double_sh,
        ['r']           /**/ => TEMA_TINCO.single_sh,

        //  Parmatéma.
        ['p']           /**/ => TEMA_PARMA.single_dn,
        ['b']
        | ['m', 'b']    /**/ => TEMA_PARMA.double_dn,
        ['f'] | ['φ']   /**/ => TEMA_PARMA.single_up,
        ['m', 'p']      /**/ => TEMA_PARMA.double_up,
        ['m']           /**/ => TEMA_PARMA.double_sh,
        ['v']           /**/ => TEMA_PARMA.single_sh,

        //  Calmatéma.
        ['c'] | ['k']   /**/ => TEMA_CALMA.single_dn,
        ['g']
        | ['n', 'g']    /**/ => TEMA_CALMA.double_dn,
        ['c', 'h']
        | ['k', 'h']    /**/ => TEMA_CALMA.single_up,
        ['n', 'c']      /**/ => TEMA_CALMA.double_up,
        ['ñ']           /**/ => TEMA_CALMA.double_sh,
        // ['y']           /**/ => TEMA_CALMA.single_sh, // Special case.

        //  NOTE: This sound vanished from Quenya very early. However, it may
        //      still be useful to have a way to write, either for representing
        //      very old texts or (more likely) to represent a "vanished"
        //      consonant (such as in "ʒalda").
        //  https://at.mansbjorkman.net/teng_quenya.htm#note_anna
        ['g', 'h']
        | ['ɣ'] | ['ʒ'] /**/ => TEMA_CALMA.single_sh,

        //  Qessetéma.
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

        //  Irregulars.
        // ['r']           /**/ => TENGWA_ROMEN, // Only with following vowel.
        ['r', 'd']      /**/ => TENGWA_ARDA,
        ['l']           /**/ => TENGWA_LAMBE,
        ['l', 'd']      /**/ => TENGWA_ALDA,
        ['s']           /**/ => TENGWA_SILME,
        ['s', 's']
        | ['z'] | ['ß'] /**/ => TENGWA_ESSE,

        ['h']           /**/ => TENGWA_HYARMEN,

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
        ['a', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_A)),
        ['o', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_O)),
        ['u', 'i'] => Some(Glyph::new_both(CARRIER_DIPH_I, TEHTA_U)),

        ['a', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_A)),
        ['e', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_E)),
        ['i', 'u'] => Some(Glyph::new_both(CARRIER_DIPH_U, TEHTA_I)),

        _ => None,
    }
}


pub const fn get_tehta(slice: &[char]) -> Option<(Tehta, bool)> {
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


pub const fn get_vowel_glyph(slice: &[char]) -> Option<Glyph> {
    if let Some(glyph) = get_diphthong(slice) {
        Some(glyph)
    } else if let Some((tehta, alt)) = get_tehta(slice) {
        Some(Glyph::new_vowel(tehta, alt))
    } else {
        None
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
        self.previous.map(Token::Glyph)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        macro_rules! finish {
            ($glyph:expr) => {finish!($glyph, 0)};
            ($glyph:expr, $len:expr) => {replace!($glyph, None, $len)}
        }
        macro_rules! replace {
            ($old:expr, $new:expr, $len:expr) => {{
                let finished = $old;
                self.current = $new;
                self.previous = Some(finished);

                ParseAction::MatchedToken {
                    token: Token::Glyph(finished),
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

            match &current.tehta {
                Some(_) => ParseAction::MatchedNone,
                None => match chunk {
                    ['y', ..] if !current.palatal => {
                        current.palatal = true;
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
                        finish!(*current)
                    }
                    ['l' | 'r'] => {
                        //  The H in softened HL and HR is represented by Halla,
                        //      not Hyarmen.
                        current.replace_base(TENGWA_HYARMEN, TENGWA_HALLA);
                        finish!(*current)
                    }
                    _ => {
                        if let Some(new) = get_diphthong(chunk) {
                            //  A following vowel sound changes Órë to Rómen.
                            current.replace_base(TENGWA_ORE, TENGWA_ROMEN);

                            //  Set the diphthong as the current glyph, while
                            //      returning the glyph that was in progress.
                            replace!(*current, Some(new), chunk.len())
                        } else if let Some((tehta, alt)) = get_tehta(chunk) {
                            //  A following vowel sound changes Órë to Rómen.
                            current.replace_base(TENGWA_ORE, TENGWA_ROMEN);
                            current.tehta = Some(tehta);
                            current.tehta_alt = alt;

                            //  In the Classical mode, a vowel tehta is the last
                            //      modification that can be made to a tengwa.
                            //      Nothing else after the vowel can affect the
                            //      glyph, so finish it now.
                            finish!(*current, chunk.len())
                        } else {
                            ParseAction::MatchedNone
                        }
                    }
                }
            }
        } else {
            //  Try to find a new glyph.

            //  Check for special cases.
            if initial && ['n', 'w'] == chunk {
                self.current = Some(Glyph::new_base(TENGWA_NWALME));
                ParseAction::MatchedPart(2)
            } else if let ['x', ..] = chunk {
                self.current = Some(Glyph::new_base(TENGWA_CALMA).with_rince(true));
                ParseAction::MatchedPart(1)
            } else if let ['y', ..] = chunk {
                self.current = Some(Glyph::new_base(TENGWA_ANNA).with_palatal(true));
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
                    new.replace_base(TENGWA_ANGA, TENGWA_NOLDO);

                    //  Initial NGW is represented by Ñwalmë, not Ungwë.
                    new.replace_base(TENGWA_UNGWE, TENGWA_NWALME);
                } else {
                    //  Medial H is represented by Aha, not Hyarmen.
                    new.replace_base(TENGWA_HYARMEN, TENGWA_AHA);
                }

                self.current = Some(new);
                ParseAction::MatchedPart(chunk.len())
            }

            //  Check for a vowel or diphthong.
            else if let Some(new) = get_vowel_glyph(chunk) {
                self.previous = Some(new);
                ParseAction::MatchedToken {
                    token: Token::Glyph(new),
                    len: chunk.len(),
                }
            } else {
                ParseAction::MatchedNone
            }
        }
    }
}
