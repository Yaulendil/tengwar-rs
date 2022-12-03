//! Utility module for more easily writing more meaningful test code.
#![cfg(test)]


macro_rules! params {
    () => { "" };
    ($k:ident=$v:expr) => {
        concat!(stringify!($k), "=", stringify!($v))
    };
    ($k:ident=$v:expr $(, $kn:ident=$vn:expr)+ $(,)?) => {
        concat!(params!($k=$v), $(", ", params!($kn=$vn)),+)
    };
    ($($t:tt)*) => { stringify!($($t)*) };
}


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
            // "[{file}:{line:0>3}] {mode}: {input:?} -> {received}{params}",
            "[{file}:{line:0>3}] {input:?} -> {received}{params}",
            file = file!(),
            line = line!(),
            // mode = stringify!($mode),
            input = $input,
            params = concat!($("  [", params!($($t)*), "]")?),
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

    ($lhs:ident == $rhs:expr) => {{
        let (lhs_in, lhs_out) = &$lhs;
        let (rhs_in, rhs_out) = &$rhs;

        assert_eq!(lhs_out, rhs_out,
            "Output of `{lhs}` does not match that of `{rhs}`.\
            \n  {lhs_in:>w$}  (left): {lhs_out}\
            \n  {rhs_in:>w$} (right): {rhs_out}",
            lhs = stringify!($lhs),
            rhs = stringify!($rhs),
            w = lhs_in.chars().count().max(rhs_in.chars().count()),
        );
    }};
    ($lhs:ident != $rhs:expr) => {{
        let (lhs_in, lhs_out) = &$lhs;
        let (rhs_in, rhs_out) = &$rhs;

        assert_ne!(lhs_out, rhs_out,
            "Output of `{lhs}` matches that of `{rhs}`, but should not.\
            \n  {lhs_in:>w$}  (left): {lhs_out}\
            \n  {rhs_in:>w$} (right): {rhs_out}",
            lhs = stringify!($lhs),
            rhs = stringify!($rhs),
            w = lhs_in.chars().count().max(rhs_in.chars().count()),
        );
    }};
}
