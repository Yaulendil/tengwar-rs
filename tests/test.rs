use tengwar::{*, characters::*, mode::*};


#[test]
fn test_regulars() {
    let mut t = TEMA_TINCO.new_tengwa();
    assert_eq!(*t.as_char(), TENGWA_TINCO);

    t.tyelle.stem_dn = false;
    assert_eq!(*t.as_char(), TENGWA_ORE);

    t.tyelle.stem_up = true;
    assert_eq!(*t.as_char(), TENGWA_THULE);

    t.tyelle.doubled = true;
    assert_eq!(*t.as_char(), TENGWA_ANTO);

    t.tema = &TEMA_PARMA;
    assert_eq!(*t.as_char(), TENGWA_AMPA);
}


#[test]
fn test_iteration() {
    //  Test switching style of A-tehta between Tokens.
    {
        // let mut ts: Transcriber<Quenya> = tscr!(Quenya; "vala");
        let mut ts = Quenya::default_transcriber("vala");

        ts.settings.alt_a = false;
        let glyph = *ts.next().expect("First Token is missing.")
            .glyph().expect("First Token is not a Glyph.");
        assert_eq!(glyph.base, Some(TENGWA_VALA));
        assert_eq!(glyph.tehta, Some(TEHTA_A));
        assert!(!glyph.long_cons);
        assert!(!glyph.tehta_alt);

        ts.settings.alt_a = true;
        let glyph = *ts.next().expect("Second Token is missing.")
            .glyph().expect("Second Token is not a Glyph.");
        assert_eq!(glyph.base, Some(TENGWA_LAMBE));
        assert_eq!(glyph.tehta, Some(TEHTA_YANTA));
        assert!(!glyph.long_cons);
        assert!(!glyph.tehta_alt);

        assert!(ts.next().is_none(), "Iterator is not yet exhausted.");
    }

    //  Test switching style of long vowels between Tokens.
    {
        use glyph::TehtaChar::*;

        // let mut ts: Transcriber<Quenya> = tscr!(Quenya; "téléré");
        let mut ts = Quenya::default_transcriber("téléré");

        ts.settings.vowels = VowelStyle::Separate;
        let glyph = *ts.next().expect("First Token is missing.")
            .glyph().expect("First Token is not a Glyph.");
        assert_eq!(glyph.base, Some(TENGWA_TINCO));
        assert_eq!(glyph.tehta, Some(TEHTA_E));
        assert!(!glyph.long_cons);
        assert!(glyph.tehta_alt);
        assert!(
            matches!(glyph.tehta_char(), Some(OnAraAfter(DC_OVER_ACUTE_1))),
            "First Token has an incorrect Vowel form.",
        );

        ts.settings.vowels = VowelStyle::Doubled;
        let glyph = *ts.next().expect("Second Token is missing.")
            .glyph().expect("Second Token is not a Glyph.");
        assert_eq!(glyph.base, Some(TENGWA_LAMBE));
        assert_eq!(glyph.tehta, Some(TEHTA_E));
        assert!(!glyph.long_cons);
        assert!(glyph.tehta_alt);
        assert!(
            matches!(glyph.tehta_char(), Some(OnTengwaTwice(DC_OVER_ACUTE_1))),
            "Second Token has an incorrect Vowel form.",
        );

        ts.settings.vowels = VowelStyle::Unique;
        let glyph = *ts.next().expect("Third Token is missing.")
            .glyph().expect("Third Token is not a Glyph.");
        assert_eq!(glyph.base, Some(TENGWA_ROMEN));
        assert_eq!(glyph.tehta, Some(TEHTA_E));
        assert!(!glyph.long_cons);
        assert!(glyph.tehta_alt);
        assert!(
            matches!(glyph.tehta_char(), Some(OnTengwaOnce(DC_OVER_ACUTE_2))),
            "Third Token has an incorrect Vowel form.",
        );

        assert!(ts.next().is_none(), "Iterator is not yet exhausted.");
    }
}


#[test]
fn test_transcribe() {
    let mut tokens_saved = Vec::new();

    let tokenizer = Tokenizer::<Quenya>::from_str("eleni");
    let token_saver = tokenizer.inspect(|t| tokens_saved.push(*t));
    let token_iter = TokenIter::new(token_saver);

    assert_eq!(token_iter.count(), tokens_saved.len());
}
