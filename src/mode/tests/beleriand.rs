use crate::mode::beleriand::*;
use super::*;


const LONG: char = ANDAITH.base();


#[test]
fn test_beleriand_words() {
    test_tengwar!(Beleriand, "axë" => [
        VOWEL_A, TENGWA_CALMA, TENGWA_SILME, VOWEL_E,
    ]);

    let edhellen = test_tengwar!(Beleriand, "edhellen" => [
        VOWEL_E, TENGWA_ANTO, // edh
        VOWEL_E, TENGWA_LAMBE, TENGWA_LAMBE, // ell
        VOWEL_E, TENGWA_ORE, // en
    ]);
    test_tengwar!(Beleriand, "eðellen" == edhellen);
    test_tengwar!(Beleriand, "EÐELLEN" == edhellen);
    test_tengwar!(Beleriand, "edellen" != edhellen);
    test_tengwar!(Beleriand, "eθellen" != edhellen);
    test_tengwar!(Beleriand, "eþellen" != edhellen);
    test_tengwar!(Beleriand, "ethellen" != edhellen);

    let andaith = test_tengwar!(Beleriand, "andaith" => [
        VOWEL_A, TENGWA_ANDO, DC_OVER_LINE, // and
        VOWEL_A, TEHTA_Y.base(), // ai
        TENGWA_THULE, // th
    ]);
    test_tengwar!(Beleriand, "andaiθ" == andaith);
    test_tengwar!(Beleriand, "ANDAIΘ" == andaith);
    test_tengwar!(Beleriand, "andaiþ" == andaith);
    test_tengwar!(Beleriand, "ANDAIÞ" == andaith);
    test_tengwar!(Beleriand, "andait" != andaith);
    test_tengwar!(Beleriand, "andaið" != andaith);
    test_tengwar!(Beleriand, "andaidh" != andaith);

    //  Final F, after consonant.
    let parf = test_tengwar!(Beleriand, "parf" => [
        TENGWA_PARMA, // p
        VOWEL_A, TENGWA_ROMEN, // ar
        TENGWA_AMPA, // v
    ]);
    test_tengwar!(Beleriand, "parv" == parf);
    test_tengwar!(Beleriand, "parφ" != parf);
    test_tengwar!(Beleriand, "parph" != parf);

    //  Final F, after vowel.
    let alaf = test_tengwar!(Beleriand, "alaf" => [
        VOWEL_A, TENGWA_LAMBE, // al
        VOWEL_A, TENGWA_AMPA, // av
    ]);
    test_tengwar!(Beleriand, "alav" == alaf);
    test_tengwar!(Beleriand, "alaφ" != alaf);
    test_tengwar!(Beleriand, "alaph" != alaf);

    //  Medial F, after consonant.
    let alfirin = test_tengwar!(Beleriand, "alfirin" => [
        VOWEL_A, TENGWA_LAMBE, // al
        TENGWA_FORMEN, // ph
        VOWEL_I, TENGWA_ROMEN, // ir
        VOWEL_I, TENGWA_ORE, // in
    ]);
    test_tengwar!(Beleriand, "alphirin" == alfirin);
    test_tengwar!(Beleriand, "alφirin" == alfirin);
    test_tengwar!(Beleriand, "ALΦIRIN" == alfirin);
    test_tengwar!(Beleriand, "alvirin" != alfirin);

    //  Medial F, after vowel.
    let aphadon = test_tengwar!(Beleriand, "aphadon" => [
        VOWEL_A, TENGWA_FORMEN, // aph
        VOWEL_A, TENGWA_ANDO, // ad
        VOWEL_O, TENGWA_ORE, // on
    ]);
    test_tengwar!(Beleriand, "afadon" == aphadon);
    test_tengwar!(Beleriand, "aφadon" == aphadon);
    test_tengwar!(Beleriand, "AΦADON" == aphadon);
    test_tengwar!(Beleriand, "avadon" != aphadon);

    let telch = test_tengwar!(Beleriand, "telch" => [
        TENGWA_TINCO, VOWEL_E, TENGWA_LAMBE, TENGWA_AHA,
    ]);
    test_tengwar!(Beleriand, "telkh" == telch);
    let calen = test_tengwar!(Beleriand, "calen" => [
        TENGWA_CALMA, VOWEL_A, TENGWA_LAMBE, VOWEL_E, TENGWA_ORE,
    ]);
    test_tengwar!(Beleriand, "kalen" == calen);

    let _hebin = test_tengwar!(Beleriand, "hebin" => [
        TENGWA_HYARMEN, VOWEL_E, TENGWA_UMBAR, VOWEL_I, TENGWA_ORE,
    ]);

    let grist = test_tengwar!(Beleriand, "grist" => [
        TENGWA_ANGA, TENGWA_ROMEN, VOWEL_I, TENGWA_SILME, TENGWA_TINCO,
    ]);
    test_tengwar!(Beleriand[nuquerna=true], "grist" == grist);

    let _acharn = test_tengwar!(Beleriand, "acharn" => [
        VOWEL_A, TENGWA_AHA, VOWEL_A, TENGWA_ROMEN, TENGWA_ORE,
    ]);
    let _wethrin = test_tengwar!(Beleriand, "wethrin" => [
        TENGWA_WILYA, VOWEL_E, TENGWA_THULE, TENGWA_ROMEN, VOWEL_I, TENGWA_ORE,
    ]);

    //  Doubled nasals.
    let _venn = test_tengwar!(Beleriand, "venn" => [
        TENGWA_AMPA, VOWEL_E, TENGWA_NUMEN,
    ]);
    let _namma = test_tengwar!(Beleriand, "namma" => [
        TENGWA_ORE, VOWEL_A, TENGWA_MALTA, VOWEL_A,
    ]);

    //  Softened sounds.
    let _rhuun = test_tengwar!(Beleriand, "rhûn" => [
        TENGWA_ARDA, VOWEL_U, LONG, TENGWA_ORE,
    ]);
    let _lhuug = test_tengwar!(Beleriand, "lhûg" => [
        TENGWA_ALDA, VOWEL_U, LONG, TENGWA_ANGA,
    ]);
    let _mhellyn = test_tengwar!(Beleriand, "mhellyn" => [
        TENGWA_VALA_HOOKED, VOWEL_E, TENGWA_LAMBE, TENGWA_LAMBE, VOWEL_Y, TENGWA_ORE,
    ]);

    //  Consonantal initial I.
    let iorhael = test_tengwar!(Beleriand, "iorhael" => [
        TENGWA_ARA, VOWEL_O, TENGWA_ROMEN,
        TENGWA_HYARMEN, VOWEL_A, VOWEL_E, TENGWA_LAMBE,
    ]);
    test_tengwar!(Beleriand, "jorhael" == iorhael);
    test_tengwar!(Beleriand, "yorhael" != iorhael);

    //  Non-consonantal I in the same vowel cluster.
    let _dior = test_tengwar!(Beleriand, "dior" => [
        TENGWA_ANDO, VOWEL_I, VOWEL_O, TENGWA_ROMEN,
    ]);
}


#[test]
fn test_beleriand_vowels() {
    //  Test all diphthongs.
    test_tengwar!(Beleriand, "ae" => [VOWEL_A, VOWEL_E]);
    test_tengwar!(Beleriand, "oe" => [VOWEL_O, VOWEL_E]);
    test_tengwar!(Beleriand, "ai" => [VOWEL_A, TEHTA_Y.base()]);
    test_tengwar!(Beleriand, "ei" => [VOWEL_E, TEHTA_Y.base()]);
    test_tengwar!(Beleriand, "ui" => [VOWEL_U, TEHTA_Y.base()]);
    test_tengwar!(Beleriand, "au" => [VOWEL_A, MOD_LABIAL] as au);
    test_tengwar!(Beleriand, "aw" == au);

    //  Test all vowels, alone.
    test_tengwar!(Beleriand, "a" => [VOWEL_A]);
    test_tengwar!(Beleriand, "e" => [VOWEL_E]);
    test_tengwar!(Beleriand, "i" => [VOWEL_I]);
    test_tengwar!(Beleriand, "o" => [VOWEL_O]);
    test_tengwar!(Beleriand, "u" => [VOWEL_U]);
    test_tengwar!(Beleriand, "y" => [VOWEL_Y]);
    test_tengwar!(Beleriand, "á" => [VOWEL_A, LONG] as aa);
    test_tengwar!(Beleriand, "é" => [VOWEL_E, LONG] as ee);
    test_tengwar!(Beleriand, "í" => [VOWEL_I, LONG] as ii);
    test_tengwar!(Beleriand, "ó" => [VOWEL_O, LONG] as oo);
    test_tengwar!(Beleriand, "ú" => [VOWEL_U, LONG] as uu);
    test_tengwar!(Beleriand, "ý" => [VOWEL_Y, LONG] as yy);
    test_tengwar!(Beleriand, "â" == aa);
    test_tengwar!(Beleriand, "ê" == ee);
    test_tengwar!(Beleriand, "î" == ii);
    test_tengwar!(Beleriand, "ô" == oo);
    test_tengwar!(Beleriand, "û" == uu);
    test_tengwar!(Beleriand, "ŷ" == yy);
    test_tengwar!(Beleriand, "aa" == aa);
    test_tengwar!(Beleriand, "ee" == ee);
    test_tengwar!(Beleriand, "ii" == ii);
    test_tengwar!(Beleriand, "oo" == oo);
    test_tengwar!(Beleriand, "uu" == uu);
    test_tengwar!(Beleriand, "yy" == yy);

    //  Test all vowels, before consonants.
    test_tengwar!(Beleriand, "ath" => [VOWEL_A, TENGWA_THULE]);
    test_tengwar!(Beleriand, "eth" => [VOWEL_E, TENGWA_THULE]);
    test_tengwar!(Beleriand, "ith" => [VOWEL_I, TENGWA_THULE]);
    test_tengwar!(Beleriand, "oth" => [VOWEL_O, TENGWA_THULE]);
    test_tengwar!(Beleriand, "uth" => [VOWEL_U, TENGWA_THULE]);
    test_tengwar!(Beleriand, "yth" => [VOWEL_Y, TENGWA_THULE]);
    test_tengwar!(Beleriand, "áth" => [VOWEL_A, LONG, TENGWA_THULE] as aath);
    test_tengwar!(Beleriand, "éth" => [VOWEL_E, LONG, TENGWA_THULE] as eeth);
    test_tengwar!(Beleriand, "íth" => [VOWEL_I, LONG, TENGWA_THULE] as iith);
    test_tengwar!(Beleriand, "óth" => [VOWEL_O, LONG, TENGWA_THULE] as ooth);
    test_tengwar!(Beleriand, "úth" => [VOWEL_U, LONG, TENGWA_THULE] as uuth);
    test_tengwar!(Beleriand, "ýth" => [VOWEL_Y, LONG, TENGWA_THULE] as yyth);
    test_tengwar!(Beleriand, "âth" == aath);
    test_tengwar!(Beleriand, "êth" == eeth);
    test_tengwar!(Beleriand, "îth" == iith);
    test_tengwar!(Beleriand, "ôth" == ooth);
    test_tengwar!(Beleriand, "ûth" == uuth);
    test_tengwar!(Beleriand, "ŷth" == yyth);
    test_tengwar!(Beleriand, "aath" == aath);
    test_tengwar!(Beleriand, "eeth" == eeth);
    test_tengwar!(Beleriand, "iith" == iith);
    test_tengwar!(Beleriand, "ooth" == ooth);
    test_tengwar!(Beleriand, "uuth" == uuth);
    test_tengwar!(Beleriand, "yyth" == yyth);
}
