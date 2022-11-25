use crate::{characters::*, TengwarMode, ToTengwar, Transcriber};


/// This trait defines higher-level behavior for rendering Tengwar.
#[allow(unused_variables)]
pub trait Policy: Copy {
    /// Returns a boolean indicating whether a given character may form a
    ///     ligature with a [long carrier](CARRIER_LONG) that follows it.
    ///
    /// The ligature will be formed by emitting a Zero-Width Joiner between the
    ///     two characters.
    fn ligates_with_ara(base: char) -> bool { false }

    /// Returns a boolean indicating whether a [short carrier](CARRIER_SHORT)
    ///     may form a ligature with a given character that follows it.
    ///
    /// The ligature will be formed by replacing the short carrier character
    ///     with [a variant](CARRIER_SHORT_LIG).
    fn telco_ligates_with(base: char) -> bool { false }

    /// Determine whether two [`Glyph`]s can be joined by a zero-width joiner.
    fn ligature_valid<P: Policy>(
        prev: &Glyph<Self>,
        next: &Glyph<P>,
        level: u8,
    ) -> bool { false }

    /// Returns the "Nuquerna", or inverted, variant of a given character, if it
    ///     has one.
    ///
    /// The Nuquerna variant is used when a significant portion of a tengwa
    ///     extends above the center of the character, but a diacritical tehta
    ///     also needs to occupy that same space. The Nuquerna variant instead
    ///     extends downwards, leaving the space above free for the tehta.
    fn nuquerna(base: char) -> char { base }

    /// Check whether a tengwa has an inverted variant.
    fn nuquerna_valid(base: char) -> bool { false }

    /// Returns the appropriate "Sa-Rincë", or "S-hook", for a given character,
    ///     if it can host one, taking into account whether it is the final
    ///     character in a word.
    ///
    /// The Sa-Rincë is attached to indicate that a sibilant sound follows the
    ///     character. For a character at the end of a word, a more ornate
    ///     variant may be used.
    fn rince(base: char, is_final: bool) -> Rince {
        if is_final {
            if Self::rince_valid_final(base) {
                Rince::Final
            } else {
                Rince::Basic
            }
        } else {
            Rince::Basic
        }
    }

    /// Check whether a base tengwa is suitable to receive a sa-rincë.
    fn rince_valid(base: char) -> bool { false }

    /// Check whether a base tengwa is suitable to receive the alternate rincë.
    fn rince_valid_final(base: char) -> bool { false }

    /// Create a [`Transcriber`] using the given [`TengwarMode`].
    fn transcriber<M>(input: impl ToTengwar) -> Transcriber<M, Self>
        where M: TengwarMode + Default,
    {
        input.transcriber().set_policy()
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub struct NoPolicy;
impl Policy for NoPolicy {}


#[derive(Clone, Copy, Debug, Default)]
pub struct Standard;

impl Policy for Standard {
    fn ligates_with_ara(base: char) -> bool {
        match base {
            TENGWA_TINCO..=TENGWA_ALDA => true,

            TENGWA_SILME => true,
            // TENGWA_ESSE => true,

            TENGWA_HYARMEN..=TENGWA_URE => true,
            TENGWA_OSSE_REV..=TENGWA_OSSE => true,
            TENGWA_ANNA_OPEN..=TENGWA_WAIA => true,
            _ => false,
        }
    }

    fn telco_ligates_with(base: char) -> bool {
        match Tengwa::either_from(base) {
            Tengwa::Regular(tengwa) => tengwa.tema.left || !tengwa.tyelle.stem_up,
            Tengwa::Irregular(char) => match char {
                TENGWA_ROMEN..=TENGWA_ESSE_NUQ => true,
                // TENGWA_HWESTA_SINDARINWA | TENGWA_URE => true,
                TENGWA_ARA | TENGWA_TELCO => true,

                TENGWA_ANNA_OPEN => true,
                TENGWA_MALTA_HOOKED => true,
                TENGWA_VALA_HOOKED => true,
                // TENGWA_WAIA => true,
                _ => false,
            }
        }
    }

    fn ligature_valid<P: Policy>(
        prev: &Glyph<Self>,
        next: &Glyph<P>,
        level: u8,
    ) -> bool {
        const L_SILME: u8 = 2;
        const L_SILME_MORE: u8 = 3;
        const L_REGULARS: u8 = 3;

        if level == 0 { return false; }

        let lhs = *prev.parts().rhs();
        let rhs = *next.parts().lhs();
        let tengwar = (&lhs.tengwa, &rhs.tengwa);
        let tehtar = (lhs.tehta, rhs.tehta);

        match tengwar {
            (Some(Tengwa::Irregular(TENGWA_SILME)), Some(rhs)) => {
                //  Left tengwa is Silmë.

                //  Ligatures of Silmë are very compact. The tehtar may make it
                //      too crowded. Determine whether this is the case.
                let too_crowded: bool = match tehtar {
                    //  Two single dots are okay.
                    (Some(DC_OVER_DOT_1), Some(DC_OVER_DOT_1)) => false,

                    /*//  A single dot paired with another is okay.
                    (Some(DC_OVER_DOT_1), Some(_)) => false,
                    (Some(_), Some(DC_OVER_DOT_1)) => false,*/

                    //  Two more complex tehtar would be too much.
                    (Some(_), Some(_)) => true,
                    _ => false,
                };

                if too_crowded {
                    false
                } else {
                    match rhs {
                        //  Allow ligation with any regular.
                        _ if level < L_SILME => false,
                        Tengwa::Regular(_) => true,

                        //  Allow ligation with select irregulars.
                        _ if level < L_SILME_MORE => false,
                        Tengwa::Irregular(TENGWA_SILME) => true,
                        Tengwa::Irregular(TENGWA_ESSE) => true,
                        Tengwa::Irregular(TENGWA_ROMEN) => true,
                        Tengwa::Irregular(TENGWA_ARDA) => true,
                        Tengwa::Irregular(TENGWA_LAMBE) => true,
                        Tengwa::Irregular(TENGWA_ALDA) => true,
                        Tengwa::Irregular(TENGWA_HALLA) => true,
                        Tengwa::Irregular(TENGWA_MALTA_HOOKED) => true,
                        Tengwa::Irregular(TENGWA_VALA_HOOKED) => true,
                        Tengwa::Irregular(TENGWA_LOWDHAM_HW) => true,

                        //  Do not allow ligation with anything else.
                        Tengwa::Irregular(_) => false,
                    }
                }
            }
            (Some(Tengwa::Regular(lhs)), Some(Tengwa::Regular(rhs))) => {
                //  Both tengwar are regular. Allow ligation between two regular
                //      tengwar, joining their stems, if they have shapes
                //      approximating `dp`.
                L_REGULARS <= level
                    && (lhs.tema.left && lhs.tyelle.is_ascending())
                    && (!rhs.tema.left && rhs.tyelle.is_descending())
            }
            _ => false,
        }
    }

    fn nuquerna(base: char) -> char {
        match base {
            TENGWA_SILME => TENGWA_SILME_NUQ,
            TENGWA_ESSE => TENGWA_ESSE_NUQ,
            other => other,
        }
    }

    fn nuquerna_valid(base: char) -> bool {
        base == TENGWA_SILME || base == TENGWA_ESSE
    }

    fn rince_valid(base: char) -> bool {
        match base {
            TENGWA_ROMEN | TENGWA_ARDA
            | TENGWA_SILME | TENGWA_SILME_NUQ
            | TENGWA_ESSE | TENGWA_ESSE_NUQ => false,
            _ => true,
        }
    }

    fn rince_valid_final(base: char) -> bool {
        match base {
            TENGWA_LAMBE | TENGWA_ALDA | TENGWA_HYARMEN => true,
            tengwa if TEMA_TINCO.contains(tengwa) => true,
            tengwa if TEMA_PARMA.contains(tengwa) => true,
            //  NOTE: The left-bow Témar CAN support the alternate, but are
            //      written with the basic form in canonical sources.
            // tengwa if TEMA_CALMA.contains(tengwa) => true,
            // tengwa if TEMA_QESSE.contains(tengwa) => true,
            _ => false,
        }
    }
}


/*pub trait IterPolicyChange<P: Policy> {
    type NewIter<Q: Policy>: Iterator<Item=Token<Q>>;

    fn change_policy<Q: Policy>(self) -> Self::NewIter<Q>;
}

impl<I: Iterator<Item=Token<P>>, P: Policy> IterPolicyChange<P> for I {
    type NewIter<Q: Policy> = Map<I, fn(Token<P>) -> Token<Q>>;

    fn change_policy<Q: Policy>(self) -> Self::NewIter<Q> {
        self.map(Token::<P>::change_policy::<Q>)
    }
}*/
