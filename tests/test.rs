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
fn test_transcribe() {
    let mut tokens_saved = Vec::new();

    let token_iter = Tokenizer::<Quenya>::from_str("eleni");
    let token_saver = token_iter.inspect(|t| tokens_saved.push(*t));
    let transcriber = Transcriber::new(token_saver);

    assert_eq!(transcriber.count(), tokens_saved.len());
}
