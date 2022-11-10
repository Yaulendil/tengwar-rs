/// A diacritical vowel marker that may be rendered in an alternate "long" form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tehta {
    /// The primary form taken by this tehta.
    pub base: char,
    /// A secondary form that may be used to represent a long vowel.
    pub alternate: Option<char>,
    /// Indicates whether the primary form may be printed twice.
    pub can_double: bool,
}

impl Tehta {
    /// Define a tehta with only one shape. To lengthen it will require the
    ///     extended carrier.
    pub const fn single(base: char) -> Self {
        Self { base, alternate: None, can_double: false }
    }

    /// Define a tehta with one shape, which can be printed twice to indicate a
    ///     lengthened vowel.
    pub const fn double(base: char) -> Self {
        Self { base, alternate: None, can_double: true }
    }

    /// Define a tehta with two shapes. A lengthened vowel may be represented by
    ///     printing either the second shape once, or the first shape twice.
    pub const fn altern(base: char, alt: char) -> Self {
        Self { base, alternate: Some(alt), can_double: true }
    }

    /*pub const fn as_single(mut self) -> Self {
        self.alternate = None;
        self.can_double = false;
        self
    }*/

    /// Add a secondary form that may be used to represent a long vowel.
    pub const fn with_alt(mut self, alt: char) -> Self {
        self.alternate = Some(alt);
        self
    }

    /// Specify that the basic shape of this tehta may be printed twice to
    ///     represent a long vowel.
    pub const fn with_double(mut self) -> Self {
        self.can_double = true;
        self
    }

    /// Return `true` if the long variant of this tehta *must* be written with
    ///     the extended carrier.
    pub const fn needs_ara(&self) -> bool {
        !(self.can_double || self.alternate.is_some())
    }
}
