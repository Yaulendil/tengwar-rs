use crate::mode::gondor::*;
use super::*;


#[test]
fn test_gondor_alt_a() {
    let _adan = test_tengwar!(Gondor, "adan" => [
        TENGWA_ANDO, TEHTA_A.base(), // ad
        TENGWA_NUMEN, TEHTA_A.base(), // an
    ]);
    let _adan_alt = test_tengwar!(Gondor[alt_a=true], "adan" => [
        TENGWA_ANDO, TEHTA_YANTA.base(), // ad
        TENGWA_NUMEN, TEHTA_YANTA.base(), // an
    ]);
    // test_tengwar!(adan != adan_alt);

    let _edain = test_tengwar!(Gondor, "edain" => [
        TENGWA_ANDO, TEHTA_E.base(), // ed
        CARRIER_DIPH_I, TEHTA_A.base(), // ai
        TENGWA_NUMEN, // n
    ]);
    let _edain_alt = test_tengwar!(Gondor[alt_a=true], "edain" => [
        TENGWA_ANDO, TEHTA_E.base(), // ed
        CARRIER_DIPH_I, TEHTA_YANTA.base(), // ai
        TENGWA_NUMEN, // n
    ]);
    // test_tengwar!(edain != edain_alt);
}


#[test]
fn test_gondor_nuquernar() {
    //  Check Silmë.
    let _estel = test_tengwar!(Gondor, "estel" => [
        TENGWA_SILME, TEHTA_E.base(), // es
        TENGWA_TINCO, // t
        TENGWA_LAMBE, TEHTA_E.base(), // el
    ]);
    let _estel_nuq = test_tengwar!(Gondor[nuquerna=true], "estel" => [
        TENGWA_SILME_NUQ, TEHTA_E.base(), // es
        TENGWA_TINCO, // t
        TENGWA_LAMBE, TEHTA_E.base(), // el
    ]);
    // test_tengwar!(estel != estel_nuq);

    //  Check Essë.
    let lossen = test_tengwar!(Gondor, "lossen" => [
        TENGWA_LAMBE, // l
        TENGWA_ESSE, TEHTA_O.base(), // oss
        TENGWA_NUMEN, TEHTA_E.base(), // en,
    ]);
    let lossen_nuq = test_tengwar!(Gondor[nuquerna=true], "lossen" => [
        TENGWA_LAMBE, // l
        TENGWA_ESSE_NUQ, TEHTA_O.base(), // oss
        TENGWA_NUMEN, TEHTA_E.base(), // en,
    ]);
    // test_tengwar!(lossen != lossen_nuq);
    test_tengwar!(Gondor, "lozen" == lossen);
    test_tengwar!(Gondor[nuquerna=true], "lozen" == lossen_nuq);

    //  Confirm lack of Nuquerna for a vowel on Ára.
    let iisa = test_tengwar!(Gondor, "ísa" => [
        CARRIER_LONG, TEHTA_I.base(), // í
        TENGWA_SILME, // s
        CARRIER_SHORT, TEHTA_A.base(), // í
    ]);
    test_tengwar!(Gondor[nuquerna=true], "ísa" == iisa);
}


#[test]
fn test_gondor_words() {
    test_tengwar!(Gondor, "axë" => [
        TENGWA_CALMA, TEHTA_A.base(), SA_RINCE, // ax
        CARRIER_SHORT, TEHTA_E.base(), // ë
    ]);

    let edhellen = test_tengwar!(Gondor, "edhellen" => [
        TENGWA_ANTO, TEHTA_E.base(), // edh
        TENGWA_LAMBE, DC_UNDER_LINE_H, TEHTA_E.base(), // ell
        TENGWA_NUMEN, TEHTA_E.base(), // en
    ]);
    test_tengwar!(Gondor, "eðellen" == edhellen);
    test_tengwar!(Gondor, "EÐELLEN" == edhellen);
    test_tengwar!(Gondor, "edellen" != edhellen);
    test_tengwar!(Gondor, "eθellen" != edhellen);
    test_tengwar!(Gondor, "eþellen" != edhellen);
    test_tengwar!(Gondor, "ethellen" != edhellen);

    let andaith = test_tengwar!(Gondor, "andaith" => [
        TENGWA_ANDO, DC_OVER_LINE, TEHTA_A.base(), // and
        CARRIER_DIPH_I, TEHTA_A.base(), // ai
        TENGWA_THULE, // th
    ]);
    test_tengwar!(Gondor, "andaiθ" == andaith);
    test_tengwar!(Gondor, "ANDAIΘ" == andaith);
    test_tengwar!(Gondor, "andaiþ" == andaith);
    test_tengwar!(Gondor, "ANDAIÞ" == andaith);
    test_tengwar!(Gondor, "andait" != andaith);
    test_tengwar!(Gondor, "andaið" != andaith);
    test_tengwar!(Gondor, "andaidh" != andaith);

    //  Final F, after consonant.
    let parf = test_tengwar!(Gondor, "parf" => [
        TENGWA_PARMA, // p
        TENGWA_ROMEN, TEHTA_A.base(), // ar
        TENGWA_AMPA, // v
    ]);
    test_tengwar!(Gondor, "parv" == parf);
    test_tengwar!(Gondor, "parφ" != parf);
    test_tengwar!(Gondor, "parph" != parf);

    //  Final F, after vowel.
    let alaf = test_tengwar!(Gondor, "alaf" => [
        TENGWA_LAMBE, TEHTA_A.base(), // al
        TENGWA_AMPA, TEHTA_A.base(), // av
    ]);
    test_tengwar!(Gondor, "alav" == alaf);
    test_tengwar!(Gondor, "alaφ" != alaf);
    test_tengwar!(Gondor, "alaph" != alaf);

    //  Medial F, after consonant.
    let alfirin = test_tengwar!(Gondor, "alfirin" => [
        TENGWA_LAMBE, TEHTA_A.base(), // al
        TENGWA_FORMEN, // ph
        TENGWA_ROMEN, TEHTA_I.base(), // ir
        TENGWA_NUMEN, TEHTA_I.base(), // in
    ]);
    test_tengwar!(Gondor, "alphirin" == alfirin);
    test_tengwar!(Gondor, "alφirin" == alfirin);
    test_tengwar!(Gondor, "ALΦIRIN" == alfirin);
    test_tengwar!(Gondor, "alvirin" != alfirin);

    //  Medial F, after vowel.
    let aphadon = test_tengwar!(Gondor, "aphadon" => [
        TENGWA_FORMEN, TEHTA_A.base(), // aph
        TENGWA_ANDO, TEHTA_A.base(), // ad
        TENGWA_NUMEN, TEHTA_O.base(), // on
    ]);
    test_tengwar!(Gondor, "afadon" == aphadon);
    test_tengwar!(Gondor, "aφadon" == aphadon);
    test_tengwar!(Gondor, "AΦADON" == aphadon);
    test_tengwar!(Gondor, "avadon" != aphadon);

    let telch = test_tengwar!(Gondor, "telch" => [
        TENGWA_TINCO, // t
        TENGWA_LAMBE, TEHTA_E.base(), // el
        TENGWA_HWESTA, // ch
    ]);
    test_tengwar!(Gondor, "telkh" == telch);

    let calen = test_tengwar!(Gondor, "calen" => [
        TENGWA_QESSE, // c
        TENGWA_LAMBE, TEHTA_A.base(), // al
        TENGWA_NUMEN, TEHTA_E.base(), // en
    ]);
    test_tengwar!(Gondor, "kalen" == calen);

    let _hebin = test_tengwar!(Gondor, "hebin" => [
        TENGWA_HYARMEN, // h
        TENGWA_UMBAR, TEHTA_E.base(), // eb
        TENGWA_NUMEN, TEHTA_I.base(), // in
    ]);
    let _grist = test_tengwar!(Gondor, "grist" => [
        TENGWA_UNGWE, // g
        TENGWA_ROMEN, // r
        TENGWA_SILME, TEHTA_I.base(), // is
        TENGWA_TINCO, // t
    ]);
    let _acharn = test_tengwar!(Gondor, "acharn" => [
        TENGWA_HWESTA, TEHTA_A.base(), // ach
        TENGWA_ROMEN, TEHTA_A.base(), // ar
        TENGWA_NUMEN, // n
    ]);
    let _wethrin = test_tengwar!(Gondor, "wethrin" => [
        TENGWA_WILYA, // w
        TENGWA_THULE, TEHTA_E.base(), // eth
        TENGWA_ROMEN, // r
        TENGWA_NUMEN, TEHTA_I.base(), // in
    ]);

    //  Doubled nasals.
    let _venn = test_tengwar!(Gondor, "venn" => [
        TENGWA_AMPA, // v
        TENGWA_NUMEN, MOD_NASAL, TEHTA_E.base(), // enn
    ]);
    let _namma = test_tengwar!(Gondor, "namma" => [
        TENGWA_NUMEN, // n
        TENGWA_MALTA, MOD_NASAL, TEHTA_A.base(), // amm
        CARRIER_SHORT, TEHTA_A.base(), // a
    ]);

    let (test_rhuun, test_lhuug);

    #[cfg(not(any(feature = "long-vowel-double", feature = "long-vowel-unique")))] {
        test_rhuun = [TENGWA_ARDA, CARRIER_LONG, TEHTA_U.base(), TENGWA_NUMEN];
        test_lhuug = [TENGWA_ALDA, CARRIER_LONG, TEHTA_U.base(), TENGWA_UNGWE];
    }
    #[cfg(all(feature = "long-vowel-double", not(feature = "long-vowel-unique")))] {
        test_rhuun = [TENGWA_ARDA, TENGWA_NUMEN, TEHTA_U.base(), TEHTA_U.base()];
        test_lhuug = [TENGWA_ALDA, TENGWA_UNGWE, TEHTA_U.base(), TEHTA_U.base()];
    }
    #[cfg(feature = "long-vowel-unique")] {
        test_rhuun = [TENGWA_ARDA, TENGWA_NUMEN, TEHTA_U.long()];
        test_lhuug = [TENGWA_ALDA, TENGWA_UNGWE, TEHTA_U.long()];
    }

    //  Softened sounds.
    let _rhuun = test_tengwar!(Gondor, "rhûn" => test_rhuun);
    let _lhuug = test_tengwar!(Gondor, "lhûg" => test_lhuug);
    let _mhellyn = test_tengwar!(Gondor, "mhellyn" => [
        TENGWA_MALTA_HOOKED, // mh
        TENGWA_LAMBE, MOD_LONG_CONS, TEHTA_E.base(), // ell
        TENGWA_NUMEN, TEHTA_Y.base(), // yn
    ]);

    //  Consonantal initial I.
    let iorhael = test_tengwar!(Gondor, "iorhael" => [
        TENGWA_YANTA, // i
        TENGWA_ROMEN, TEHTA_O.base(), // or
        TENGWA_HYARMEN, // h
        CARRIER_DIPH_E, TEHTA_A.base(), // ae
        TENGWA_LAMBE, // l
    ]);
    test_tengwar!(Gondor, "jorhael" == iorhael);
    test_tengwar!(Gondor, "yorhael" != iorhael);

    //  Non-consonantal I in the same vowel cluster.
    let _dior = test_tengwar!(Gondor, "dior" => [
        TENGWA_ANDO, // d
        CARRIER_SHORT, TEHTA_I.base(), // i
        TENGWA_ORE, TEHTA_O.base(), // or
    ]);
}


#[test]
fn test_gondor_vowels() {
    //  Test all diphthongs.
    test_tengwar!(Gondor, "ae" => [CARRIER_DIPH_E, TEHTA_A.base()]);
    test_tengwar!(Gondor, "oe" => [CARRIER_DIPH_E, TEHTA_O.base()]);
    test_tengwar!(Gondor, "ai" => [CARRIER_DIPH_I, TEHTA_A.base()]);
    test_tengwar!(Gondor, "ei" => [CARRIER_DIPH_I, TEHTA_E.base()]);
    test_tengwar!(Gondor, "ui" => [CARRIER_DIPH_I, TEHTA_U.base()]);
    test_tengwar!(Gondor, "au" => [CARRIER_DIPH_U, TEHTA_A.base()] as au);
    test_tengwar!(Gondor, "aw" == au);

    //  Test all vowels, alone.
    test_tengwar!(Gondor, "a" => [CARRIER_SHORT, TEHTA_A.base()]);
    test_tengwar!(Gondor, "e" => [CARRIER_SHORT, TEHTA_E.base()]);
    test_tengwar!(Gondor, "i" => [CARRIER_SHORT, TEHTA_I.base()]);
    test_tengwar!(Gondor, "o" => [CARRIER_SHORT, TEHTA_O.base()]);
    test_tengwar!(Gondor, "u" => [CARRIER_SHORT, TEHTA_U.base()]);
    test_tengwar!(Gondor, "y" => [CARRIER_SHORT, TEHTA_Y.base()]);
    test_tengwar!(Gondor, "á" => [CARRIER_LONG, TEHTA_A.base()] as aa);
    test_tengwar!(Gondor, "é" => [CARRIER_LONG, TEHTA_E.base()] as ee);
    test_tengwar!(Gondor, "í" => [CARRIER_LONG, TEHTA_I.base()] as ii);
    test_tengwar!(Gondor, "ó" => [CARRIER_LONG, TEHTA_O.base()] as oo);
    test_tengwar!(Gondor, "ú" => [CARRIER_LONG, TEHTA_U.base()] as uu);
    test_tengwar!(Gondor, "ý" => [CARRIER_LONG, TEHTA_Y.base()] as yy);
    test_tengwar!(Gondor, "â" == aa);
    test_tengwar!(Gondor, "ê" == ee);
    test_tengwar!(Gondor, "î" == ii);
    test_tengwar!(Gondor, "ô" == oo);
    test_tengwar!(Gondor, "û" == uu);
    test_tengwar!(Gondor, "ŷ" == yy);
    test_tengwar!(Gondor, "aa" == aa);
    test_tengwar!(Gondor, "ee" == ee);
    test_tengwar!(Gondor, "ii" == ii);
    test_tengwar!(Gondor, "oo" == oo);
    test_tengwar!(Gondor, "uu" == uu);
    test_tengwar!(Gondor, "yy" == yy);

    //  Test all vowels, after consonants.
    test_tengwar!(Gondor, "ath" => [TENGWA_THULE, TEHTA_A.base()]);
    test_tengwar!(Gondor, "eth" => [TENGWA_THULE, TEHTA_E.base()]);
    test_tengwar!(Gondor, "ith" => [TENGWA_THULE, TEHTA_I.base()]);
    test_tengwar!(Gondor, "oth" => [TENGWA_THULE, TEHTA_O.base()]);
    test_tengwar!(Gondor, "uth" => [TENGWA_THULE, TEHTA_U.base()]);
    test_tengwar!(Gondor, "yth" => [TENGWA_THULE, TEHTA_Y.base()]);

    let (test_aath, test_eeth, test_iith, test_ooth, test_uuth, test_yyth);

    #[cfg(not(any(feature = "long-vowel-double", feature = "long-vowel-unique")))] {
        test_aath = [CARRIER_LONG, TEHTA_A.base(), TENGWA_THULE];
        test_eeth = [CARRIER_LONG, TEHTA_E.base(), TENGWA_THULE];
        test_iith = [CARRIER_LONG, TEHTA_I.base(), TENGWA_THULE];
        test_ooth = [CARRIER_LONG, TEHTA_O.base(), TENGWA_THULE];
        test_uuth = [CARRIER_LONG, TEHTA_U.base(), TENGWA_THULE];
        test_yyth = [CARRIER_LONG, TEHTA_Y.base(), TENGWA_THULE];
    }
    #[cfg(all(feature = "long-vowel-double", not(feature = "long-vowel-unique")))] {
        test_aath = [CARRIER_LONG, TEHTA_A.base(), TENGWA_THULE];
        test_eeth = [TENGWA_THULE, TEHTA_E.base(), TEHTA_E.base()];
        test_iith = [CARRIER_LONG, TEHTA_I.base(), TENGWA_THULE];
        test_ooth = [TENGWA_THULE, TEHTA_O.base(), TEHTA_O.base()];
        test_uuth = [TENGWA_THULE, TEHTA_U.base(), TEHTA_U.base()];
        test_yyth = [CARRIER_LONG, TEHTA_Y.base(), TENGWA_THULE];
    }
    #[cfg(feature = "long-vowel-unique")] {
        test_aath = [CARRIER_LONG, TEHTA_A.base(), TENGWA_THULE];
        test_eeth = [TENGWA_THULE, TEHTA_E.long()];
        test_iith = [CARRIER_LONG, TEHTA_I.base(), TENGWA_THULE];
        test_ooth = [TENGWA_THULE, TEHTA_O.long()];
        test_uuth = [TENGWA_THULE, TEHTA_U.long()];
        test_yyth = [CARRIER_LONG, TEHTA_Y.base(), TENGWA_THULE];
    }

    test_tengwar!(Gondor, "áth" => test_aath, as aath);
    test_tengwar!(Gondor, "éth" => test_eeth, as eeth);
    test_tengwar!(Gondor, "íth" => test_iith, as iith);
    test_tengwar!(Gondor, "óth" => test_ooth, as ooth);
    test_tengwar!(Gondor, "úth" => test_uuth, as uuth);
    test_tengwar!(Gondor, "ýth" => test_yyth, as yyth);

    test_tengwar!(Gondor, "âth" == aath);
    test_tengwar!(Gondor, "êth" == eeth);
    test_tengwar!(Gondor, "îth" == iith);
    test_tengwar!(Gondor, "ôth" == ooth);
    test_tengwar!(Gondor, "ûth" == uuth);
    test_tengwar!(Gondor, "ŷth" == yyth);

    test_tengwar!(Gondor, "aath" == aath);
    test_tengwar!(Gondor, "eeth" == eeth);
    test_tengwar!(Gondor, "iith" == iith);
    test_tengwar!(Gondor, "ooth" == ooth);
    test_tengwar!(Gondor, "uuth" == uuth);
    test_tengwar!(Gondor, "yyth" == yyth);
}
