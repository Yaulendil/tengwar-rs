//! Utility module for more easily writing more meaningful test code.
#![cfg(test)]


macro_rules! test_tengwar {
    (#$mode:ty $([$($k:ident=$v:expr),*])?, $input:expr) => {{
        use $crate::ToTengwar;

        #[allow(unused_mut)]
        let mut iter = $input.transcriber::<$mode>();
        $($(iter.settings.$k = $v;)*)?
        iter.collect()
    }};

    ($mode:ty $([$($t:tt)*])?, $input:expr => [$($chars:tt)*] as $bind:ident) => {
        let $bind = test_tengwar!(
            $mode $([$($t)*])?,
            $input => [$($chars)*]
        );
    };
    ($mode:ty $([$($t:tt)*])?, $input:expr => $expected:expr, as $bind:ident) => {
        let $bind = test_tengwar!(
            $mode $([$($t)*])?,
            $input => $expected
        );
    };

    ($mode:ty $([$($t:tt)*])?, $input:expr => $expected:expr) => {{
        let expected: String = $expected.into_iter().collect();
        let received: String = test_tengwar!(#$mode $([$($t)*])?, $input);

        println!(
            "[{file}:{line:0>3}] {mode}: {input:?} -> {received}",
            file = file!(),
            line = line!(),
            mode = stringify!($mode),
            input = $input,
        );

        assert_eq!(expected, received,
            "Transcription of {input:?} does not match expectation.\
            \n  Expected: {expected}\
            \n  Received: {received}",
            input = $input,
        );

        ($input, received)
    }};

    ($mode:ty $([$($t:tt)*])?, $input:tt == $expected:expr) => {{
        let (original, expected) = &$expected;
        let received: String = test_tengwar!(#$mode $([$($t)*])?, $input);

        assert_eq!(expected, &received,
            "Transcription of {new:?} does not match that of {old:?}.\
            \n  {old:>w$} (expected): {expected}\
            \n  {new:>w$} (received): {received}",
            new = $input,
            old = original,
            w = $input.chars().count().max(original.chars().count()),
        );
    }};
    ($mode:ty $([$($t:tt)*])?, $input:tt != $expected:expr) => {{
        let (original, expected) = &$expected;
        let received: String = test_tengwar!(#$mode $([$($t)*])?, $input);

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
