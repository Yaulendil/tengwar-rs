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


/// Convert an input into the Tengwar. If the input is a string literal, this
///     will transcribe it, returning a tuple with the input and output. If the
///     input is an already-transcribed tuple, this will resolve to a reference.
macro_rules! tengwar {
    ($mode:ty $([$($k:ident=$v:expr),*])?, $input:literal) => {{
        use $crate::ToTengwar;

        #[allow(unused_mut)]
        let mut iter = $input.transcriber::<$mode>();
        $($(iter.settings.$k = $v;)*)?
        ($input, iter.collect::<String>())
    }};
    ($mode:ty $([$($t:tt)*])?, $input:expr) => { &$input };
}


/// Test the output of the transcription process by specifying a mode, and some
///     values that should either match or differ.
macro_rules! test_tengwar {
    //  Save the conversion for later reuse.
    ($mode:ty $([$($t:tt)*])?, $lhs:tt $op:tt $rhs:tt as $bind:tt) => {
        let $bind = test_tengwar!($mode $([$($t)*])?, $lhs $op $rhs);
    };
    ($lhs:tt $op:tt $rhs:tt as $bind:tt) => {
        let $bind = test_tengwar!($lhs $op $rhs);
    };

    //  Allow chaining multiple test operations.
    ($mode:ty $([$($t:tt)*])?, $lhs:tt $op:tt $rhs:tt $($more:tt)+) => {{
        let lhs = tengwar!($mode $([$($t)*])?, $lhs);
        test_tengwar!($mode $([$($t)*])?, lhs $op $rhs);
        test_tengwar!($mode $([$($t)*])?, lhs $($more)+);
        lhs
    }};

    //  Specify that the input must transcribe to an exact sequence of `char`s.
    ($mode:ty $([$($t:tt)*])?, $lhs:tt => $rhs:tt) => {{
        let conversion = tengwar!($mode $([$($t)*])?, $lhs);
        let (input, output) = &conversion;
        let expected: String = $rhs.into_iter().collect();

        println!(
            // "[{file}:{line:0>3}] {mode}: {input:?} -> {output}{params}",
            "[{file}:{line:0>3}] {input:?} -> {output}{params}",
            file = file!(),
            line = line!(),
            // mode = stringify!($mode),
            params = concat!($("  [", params!($($t)*), "]")?),
        );

        assert_eq!(expected.as_str(), output.as_str(),
            "Transcription of {input:?} does not match expectation.\
            \n  Expected: {expected}\
            \n  Received: {output}",
        );

        conversion
    }};

    //  Specify that the input must transcribe to the same as another.
    ($mode:ty $([$($t:tt)*])?, $lhs:tt == $rhs:tt) => {{
        let lhs = tengwar!($mode $([$($t)*])?, $lhs);
        let rhs = tengwar!($mode $([$($t)*])?, $rhs);
        let (lhs_in, lhs_out) = &lhs;
        let (rhs_in, rhs_out) = &rhs;

        assert_eq!(lhs_out.as_str(), rhs_out.as_str(),
            "Transcription of {lhs_in:?} does not match that of {rhs_in:?}.\
            \n  {lhs_in:>w$}  (left): {lhs_out}\
            \n  {rhs_in:>w$} (right): {rhs_out}",
            w = lhs_in.chars().count().max(rhs_in.chars().count()),
        );

        lhs
    }};
    ($mode:ty $([$($t:tt)*])?, $lhs:tt != $rhs:tt) => {{
        let lhs = tengwar!($mode $([$($t)*])?, $lhs);
        let rhs = tengwar!($mode $([$($t)*])?, $rhs);
        let (lhs_in, lhs_out) = &lhs;
        let (rhs_in, rhs_out) = &rhs;

        assert_ne!(lhs_out.as_str(), rhs_out.as_str(),
            "Transcription of {lhs_in:?} wrongly matches that of {rhs_in:?}.\
            \n  {lhs_in:>w$}  (left): {lhs_out}\
            \n  {rhs_in:>w$} (right): {rhs_out}",
            w = lhs_in.chars().count().max(rhs_in.chars().count()),
        );

        lhs
    }};

    //  Specify that saved transcriptions must match.
    ($lhs:ident == $rhs:ident) => {{
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
    ($lhs:ident != $rhs:ident) => {{
        let (lhs_in, lhs_out) = &$lhs;
        let (rhs_in, rhs_out) = &$rhs;

        assert_ne!(lhs_out, rhs_out,
            "Output of `{lhs}` wrongly matches that of `{rhs}`.\
            \n  {lhs_in:>w$}  (left): {lhs_out}\
            \n  {rhs_in:>w$} (right): {rhs_out}",
            lhs = stringify!($lhs),
            rhs = stringify!($rhs),
            w = lhs_in.chars().count().max(rhs_in.chars().count()),
        );
    }};
}
