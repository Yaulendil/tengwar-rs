use crate::{characters::*, TengwarMode, ToTengwar, Transcriber};


/// This trait defines higher-level behavior for rendering Tengwar.
#[allow(unused_variables)]
pub trait Policy: Copy {
    // /// Define a new Policy state.
    // fn new() -> Self;

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

    fn rince_valid(base: char) -> bool { false }

    fn rince_valid_final(base: char) -> bool { false }

    /// Create a [`Transcriber`] using the given [`TengwarMode`].
    fn transcriber<M>(input: impl ToTengwar) -> Transcriber<M, Self>
        where M: TengwarMode + Default,
    {
        input.transcriber().set_policy()
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Standard;

impl Policy for Standard {
    // fn new() -> Self { Self }

    fn ligates_with_ara(base: char) -> bool {
        ligates_with_ara(base)
    }

    fn telco_ligates_with(base: char) -> bool {
        telco_ligates_with(base)
    }

    fn nuquerna(base: char) -> char {
        nuquerna(base)
    }

    fn nuquerna_valid(base: char) -> bool {
        nuquerna_valid(base)
    }

    fn rince_valid(base: char) -> bool {
        rince_valid(base)
    }

    fn rince_valid_final(base: char) -> bool {
        rince_valid_final(base)
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
