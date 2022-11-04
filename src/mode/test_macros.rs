//! Utility module for more easily writing more meaningful test code.

macro_rules! test_tengwar {
    ($mode:ty, $input:expr => [$($chars:expr),+ $(,)?]) => {{
        let expected: String = [$($chars),+].into_iter().collect();
        let received: String = <$mode>::transcribe($input);
        // let chars: Vec<char> = received.chars().collect();

        // assert_eq!(chars, [$($chars),+],
        assert_eq!(expected, received,
            "Transcription does not match expectation.\
            \n  Expected: {expected}\
            \n  Received: {received}",
        );
    }};
}
