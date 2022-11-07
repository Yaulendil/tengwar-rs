//! Utility module for more easily writing more meaningful test code.
#![cfg(test)]


macro_rules! test_tengwar {
    ($mode:ty, $input:expr => [$($chars:tt)*] as $bind:ident) => {
        let $bind = test_tengwar!($mode, $input => [$($chars)*]);
    };
    ($mode:ty, $input:expr => $expected:expr, as $bind:ident) => {
        let $bind = test_tengwar!($mode, $input => $expected);
    };
    ($mode:ty, $input:expr => [$($chars:tt)*]) => {
        test_tengwar!($mode, $input => ([$($chars)*]))
    };
    ($mode:ty, $input:expr => $expected:expr) => {{
        let expected: String = $expected.into_iter().collect();
        let received: String = <$mode>::transcribe($input);

        assert_eq!(expected, received,
            "Transcription of {input:?} does not match expectation.\
            \n  Expected: {expected}\
            \n  Received: {received}",
            input = $input,
        );

        // eprintln!("{received}");
        ($input, received)
    }};
    ($mode:ty, $input:tt == $expected:expr) => {{
        let (original, expected) = &$expected;
        let received: String = <$mode>::transcribe($input);

        assert_eq!(expected, &received,
            "Transcription of {new:?} does not match that of {old:?}.\
            \n  {old:>w$} (expected): {expected}\
            \n  {new:>w$} (received): {received}",
            new = $input,
            old = original,
            w = $input.chars().count().max(original.chars().count()),
        );
    }};
    ($mode:ty, $input:tt != $expected:expr) => {{
        let (original, expected) = &$expected;
        let received: String = <$mode>::transcribe($input);

        assert_ne!(expected, &received,
            "Transcription of {new:?} matches that of {old:?}, but should not.\
            \n  {old:>w$} (expected): {expected}\
            \n  {new:>w$} (received): {received}",
            new = $input,
            old = original,
            w = $input.chars().count().max(original.chars().count()),
        );
    }};
}


macro_rules! nuq {
    ($tengwa:expr) => {
        if cfg!(feature = "nuquernar") {
            nuquerna($tengwa)
        } else {
            $tengwa
        }
    };
}

impl crate::characters::Tehta {
    pub const fn pre_long(&self) -> char {
        match self {
            Self::Single(_) => crate::characters::CARRIER_LONG,
            Self::Double(c) => *c,
            Self::Altern(_, _) => unreachable!(),
        }
    }
}

#[cfg(not(feature = "long-vowel-unique"))]
macro_rules! pre_long {($tehta:expr) => {$tehta.pre_long()}}
#[cfg(feature = "long-vowel-unique")]
macro_rules! pre_long {
    (TEHTA_A) => { $crate::characters::CARRIER_LONG };
    (TEHTA_I) => { $crate::characters::CARRIER_LONG };
    (TEHTA_Y) => { $crate::characters::CARRIER_LONG };
    ($tehta:expr) => {
        //  NOTE: Leaving this branch empty would refuse to compile. Apparently
        //      having something here, even it it will never be included in the
        //      build, makes it valid in the positions needed.
        #[cfg(not(feature = "long-vowel-unique"))]
        $tehta.pre_long()
    };
}

macro_rules! rince {
    () => {
        if cfg!(feature = "alt-rince") {
            SA_RINCE_FINAL
        } else {
            SA_RINCE
        }
    };
}
