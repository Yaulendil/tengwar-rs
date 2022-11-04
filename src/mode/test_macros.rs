//! Utility module for more easily writing more meaningful test code.

macro_rules! test_tengwar {
    ($mode:ty, $input:expr => [$($chars:expr),+ $(,)?]) => {{
        let expected: String = [$($chars),+].into_iter().collect();
        let received: String = <$mode>::transcribe($input);
        // let chars: Vec<char> = received.chars().collect();

        // assert_eq!(chars, [$($chars),+],
        assert_eq!(expected, received,
            "Transcription of {input:?} does not match expectation.\
            \n  Expected: {expected}\
            \n  Received: {received}",
            input = $input,
        );

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
