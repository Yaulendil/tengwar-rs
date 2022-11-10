/// A diacritical vowel marker that may be rendered in an alternate "long" form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tehta {
    pub base: char,
    pub alternate: Option<char>,
    pub is_double: bool,
}

impl Tehta {
    pub const fn single(base: char) -> Self {
        Self { base, alternate: None, is_double: false }
    }

    pub const fn double(base: char) -> Self {
        Self { base, alternate: None, is_double: true }
    }

    pub const fn altern(base: char, alt: char) -> Self {
        Self { base, alternate: Some(alt), is_double: true }
    }

    pub const fn with_alt(mut self, alt: char) -> Self {
        self.alternate = Some(alt);
        self
    }

    pub const fn with_double(mut self) -> Self {
        self.is_double = true;
        self
    }

    pub const fn has_alt(&self) -> bool {
        self.alternate.is_some()
    }

    /// Returns `true` if the long variant of this tehta must be written with
    ///     the extended carrier.
    pub const fn needs_ara(&self) -> bool {
        !(self.is_double || self.has_alt())
    }
}
