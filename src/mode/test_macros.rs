//! Utility module for more easily writing more meaningful test code.
#![cfg(test)]


macro_rules! test_tengwar {
    ($mode:ty, $input:expr => [$($chars:tt)*] as $bind:ident) => {
        let $bind = test_tengwar!($mode, $input => [$($chars)*]);
    };
    ($mode:ty, $input:expr => [$($chars:expr),* $(,)?]) => {{
        let expected: String = [$($chars),*].into_iter().collect();
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

macro_rules! pre_long {
    ($tehta:expr) => {
        #[cfg(not(feature = "long-vowel-unique"))]
        if cfg!(feature = "long-vowel-double") {
            $tehta.long()
        } else {
            CARRIER_LONG
        }
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
