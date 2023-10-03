use crate::mode::gondor::*;
use super::*;


#[test]
fn alt_a() {
    let adan = test_tengwar!(Gondor, "adan" => [
        TENGWA_ANDO, TEHTA_A.base, // ad
        TENGWA_NUMEN, TEHTA_A.base, // an
    ]);
    test_tengwar!(Gondor[alt_a=true], "adan" != adan => [
        TENGWA_ANDO, TEHTA_YANTA.base, // ad
        TENGWA_NUMEN, TEHTA_YANTA.base, // an
    ]);

    let edain = test_tengwar!(Gondor, "edain" => [
        TENGWA_ANDO, TEHTA_E.base, // ed
        CARRIER_DIPH_I, TEHTA_A.base, // ai
        TENGWA_NUMEN, // n
    ]);
    test_tengwar!(Gondor[alt_a=true], "edain" != edain => [
        TENGWA_ANDO, TEHTA_E.base, // ed
        CARRIER_DIPH_I, TEHTA_YANTA.base, // ai
        TENGWA_NUMEN, // n
    ]);
}


#[test]
fn normalization() {
    test_tengwar!(Gondor, "andûnië"
        //  Standard codepoints.
        => [
            TENGWA_ANDO, DC_OVER_LINE, TEHTA_A.base, // and
            TENGWA_NUMEN, TEHTA_U.base, TEHTA_U.base, // ûn
            CARRIER_SHORT, TEHTA_I.base, // i
            CARRIER_SHORT, TEHTA_E.base, // ë
        ]
        //  Combining diacritic equivalents.
        == "andûnië"
        == "ANDÛNIË"
    );
}


#[test]
fn nuquernar() {
    //  Check Silmë.
    let estel = test_tengwar!(Gondor, "estel" => [
        TENGWA_SILME, TEHTA_E.base, // es
        TENGWA_TINCO, // t
        TENGWA_LAMBE, TEHTA_E.base, // el
    ]);
    test_tengwar!(Gondor[nuquerna=true], "estel" != estel => [
        TENGWA_SILME_NUQ, TEHTA_E.base, // es
        TENGWA_TINCO, // t
        TENGWA_LAMBE, TEHTA_E.base, // el
    ]);

    //  Check Essë.
    let lossen = test_tengwar!(Gondor, "lossen"
        => [
            TENGWA_LAMBE, // l
            TENGWA_ESSE, TEHTA_O.base, // oss
            TENGWA_NUMEN, TEHTA_E.base, // en
        ]
        == "lozen"
        == "loßen"
    );
    test_tengwar!(Gondor[nuquerna=true], "lossen" != lossen
        => [
            TENGWA_LAMBE, // l
            TENGWA_ESSE_NUQ, TEHTA_O.base, // oss
            TENGWA_NUMEN, TEHTA_E.base, // en
        ]
        == "lozen"
        == "loßen"
    );

    //  Confirm lack of Nuquerna for a vowel on Ára.
    let iisa = test_tengwar!(Gondor, "ísa" => [
        CARRIER_LONG, TEHTA_I.base, // í
        TENGWA_SILME, // s
        CARRIER_SHORT, TEHTA_A.base, // a
    ]);
    test_tengwar!(Gondor[nuquerna=true], iisa == "iisa" == "ísa");
}


#[test]
fn words() {
    test_tengwar!(Gondor, "axë"
        => [TENGWA_QESSE, SA_RINCE, TEHTA_A.base, CARRIER_SHORT, TEHTA_E.base]
        == "axe"
        == "acsë"
        == "aksë"
        == "akzë"
        != "akssë"
    );

    let _edhellen = test_tengwar!(Gondor, "edhellen"
        => [
            TENGWA_ANTO, TEHTA_E.base, // edh
            TENGWA_LAMBE, DC_UNDER_LINE_H, TEHTA_E.base, // ell
            TENGWA_NUMEN, TEHTA_E.base, // en
        ]
        == "eðellen"
        == "EÐELLEN"
        != "edellen"
        != "eθellen"
        != "eþellen"
        != "ethellen"
    );

    let _andaith = test_tengwar!(Gondor, "andaith"
        => [
            TENGWA_ANDO, DC_OVER_LINE, TEHTA_A.base, // and
            CARRIER_DIPH_I, TEHTA_A.base, // ai
            TENGWA_THULE, // th
        ]
        == "andaiθ"
        == "ANDAIΘ"
        == "andaiþ"
        == "ANDAIÞ"
        != "andait"
        != "andaið"
        != "andaidh"
    );

    //  Final F, after consonant.
    let _parf = test_tengwar!(Gondor, "parf"
        => [TENGWA_PARMA, TENGWA_ROMEN, TEHTA_A.base, TENGWA_AMPA]
        == "parv"
        != "parφ"
        != "parph"
    );

    //  Final F, after vowel.
    let _alaf = test_tengwar!(Gondor, "alaf"
        => [TENGWA_LAMBE, TEHTA_A.base, TENGWA_AMPA, TEHTA_A.base]
        == "alav"
        != "alaφ"
        != "alaph"
    );

    //  Medial F, after consonant.
    let _alfirin = test_tengwar!(Gondor, "alfirin"
        => [
            TENGWA_LAMBE, TEHTA_A.base, // al
            TENGWA_FORMEN, // ph
            TENGWA_ROMEN, TEHTA_I.base, // ir
            TENGWA_NUMEN, TEHTA_I.base, // in
        ]
        == "alphirin"
        == "alφirin"
        == "ALΦIRIN"
        != "alvirin"
    );

    //  Medial F, after vowel.
    let _aphadon = test_tengwar!(Gondor, "aphadon"
        => [
            TENGWA_FORMEN, TEHTA_A.base, // aph
            TENGWA_ANDO, TEHTA_A.base, // ad
            TENGWA_NUMEN, TEHTA_O.base, // on
        ]
        == "afadon"
        == "aφadon"
        == "AΦADON"
        != "avadon"
    );

    let _telch = test_tengwar!(Gondor, "telch"
        => [TENGWA_TINCO, TENGWA_LAMBE, TEHTA_E.base, TENGWA_HWESTA]
        == "telkh"
        != "telgh"
    );

    let _calen = test_tengwar!(Gondor, "calen"
        => [TENGWA_QESSE, TENGWA_LAMBE, TEHTA_A.base, TENGWA_NUMEN, TEHTA_E.base]
        == "kalen"
        != "çalen"
    );

    let _hebin = test_tengwar!(Gondor, "hebin" => [
        TENGWA_HYARMEN, // h
        TENGWA_UMBAR, TEHTA_E.base, // eb
        TENGWA_NUMEN, TEHTA_I.base, // in
    ]);
    let _grist = test_tengwar!(Gondor, "grist" => [
        TENGWA_UNGWE, // g
        TENGWA_ROMEN, // r
        TENGWA_SILME, TEHTA_I.base, // is
        TENGWA_TINCO, // t
    ]);

    let _acharn = test_tengwar!(Gondor, "acharn"
        => [TENGWA_HWESTA, TEHTA_A.base, TENGWA_ROMEN, TEHTA_A.base, TENGWA_NUMEN]
        == "akharn"
        != "agharn"
    );

    let _wethrin = test_tengwar!(Gondor, "wethrin" => [
        TENGWA_WILYA, // w
        TENGWA_THULE, TEHTA_E.base, // eth
        TENGWA_ROMEN, // r
        TENGWA_NUMEN, TEHTA_I.base, // in
    ]);

    //  Doubled nasals.
    let _venn = test_tengwar!(Gondor, "venn" => [
        TENGWA_AMPA, // v
        TENGWA_NUMEN, MOD_NASAL, TEHTA_E.base, // enn
    ]);
    let _namma = test_tengwar!(Gondor, "namma" => [
        TENGWA_NUMEN, // n
        TENGWA_MALTA, MOD_NASAL, TEHTA_A.base, // amm
        CARRIER_SHORT, TEHTA_A.base, // a
    ]);

    //  Softened sounds.
    let _rhuun = test_tengwar!(Gondor, "rhûn" => [
        TENGWA_ARDA, // rh
        TENGWA_NUMEN, TEHTA_U.base, TEHTA_U.base, // ûn
    ]);
    let _lhuug = test_tengwar!(Gondor, "lhûg" => [
        TENGWA_ALDA, // lh
        TENGWA_UNGWE, TEHTA_U.base, TEHTA_U.base, // ûg
    ]);
    let _mhellyn = test_tengwar!(Gondor, "mhellyn" => [
        TENGWA_MALTA_HOOKED, // mh
        TENGWA_LAMBE, MOD_LONG_CONS, TEHTA_E.base, // ell
        TENGWA_NUMEN, TEHTA_Y.base, // yn
    ]);

    //  Consonantal initial I and medial RH.
    let _iorhael = test_tengwar!(Gondor, "iorhael"
        => [
            TENGWA_YANTA, // i
            TENGWA_ROMEN, TEHTA_O.base, // or
            TENGWA_HYARMEN, // h
            CARRIER_DIPH_E, TEHTA_A.base, // ae
            TENGWA_LAMBE, // l
        ]
        == "jorhæl"
        != "yorhael"
    );

    //  Non-consonantal I in the same vowel cluster.
    let _dior = test_tengwar!(Gondor, "dior"
        => [TENGWA_ANDO, CARRIER_SHORT, TEHTA_I.base, TENGWA_ORE, TEHTA_O.base]
        != "djor"
        != "dyor"
    );

    //  Non-consonantal initial I.
    let _ithil = test_tengwar!(Gondor, "ithil"
        => [TENGWA_THULE, TEHTA_I.base, TENGWA_LAMBE, TEHTA_I.base]
        != "jthil"
        != "ythil"
    );
}


#[test]
fn vowels() {
    //  Test all diphthongs.
    test_tengwar!(Gondor, "ae" => [CARRIER_DIPH_E, TEHTA_A.base] == "æ" == "Æ");
    test_tengwar!(Gondor, "oe" => [CARRIER_DIPH_E, TEHTA_O.base] == "œ" == "Œ");
    test_tengwar!(Gondor, "ai" => [CARRIER_DIPH_I, TEHTA_A.base]);
    test_tengwar!(Gondor, "ei" => [CARRIER_DIPH_I, TEHTA_E.base]);
    test_tengwar!(Gondor, "ui" => [CARRIER_DIPH_I, TEHTA_U.base]);
    test_tengwar!(Gondor, "au" => [CARRIER_DIPH_U, TEHTA_A.base] == "aw");

    //  Test all vowels, alone.
    test_tengwar!(Gondor, "a" => [CARRIER_SHORT, TEHTA_A.base]);
    test_tengwar!(Gondor, "e" => [CARRIER_SHORT, TEHTA_E.base]);
    test_tengwar!(Gondor, "i" => [CARRIER_SHORT, TEHTA_I.base]);
    test_tengwar!(Gondor, "o" => [CARRIER_SHORT, TEHTA_O.base]);
    test_tengwar!(Gondor, "u" => [CARRIER_SHORT, TEHTA_U.base]);
    test_tengwar!(Gondor, "y" => [CARRIER_SHORT, TEHTA_Y.base]);
    test_tengwar!(Gondor, "á" => [CARRIER_LONG, TEHTA_A.base] == "â" == "ā" == "aa");
    test_tengwar!(Gondor, "é" => [CARRIER_LONG, TEHTA_E.base] == "ê" == "ē" == "ee");
    test_tengwar!(Gondor, "í" => [CARRIER_LONG, TEHTA_I.base] == "î" == "ī" == "ii");
    test_tengwar!(Gondor, "ó" => [CARRIER_LONG, TEHTA_O.base] == "ô" == "ō" == "oo");
    test_tengwar!(Gondor, "ú" => [CARRIER_LONG, TEHTA_U.base] == "û" == "ū" == "uu");
    test_tengwar!(Gondor, "ý" => [CARRIER_LONG, TEHTA_Y.base] == "ŷ" == "ȳ" == "yy");

    //  Test all vowels, before consonants.
    test_tengwar!(Gondor, "ath" => [TENGWA_THULE, TEHTA_A.base]);
    test_tengwar!(Gondor, "eth" => [TENGWA_THULE, TEHTA_E.base]);
    test_tengwar!(Gondor, "ith" => [TENGWA_THULE, TEHTA_I.base]);
    test_tengwar!(Gondor, "oth" => [TENGWA_THULE, TEHTA_O.base]);
    test_tengwar!(Gondor, "uth" => [TENGWA_THULE, TEHTA_U.base]);
    test_tengwar!(Gondor, "yth" => [TENGWA_THULE, TEHTA_Y.base]);
    test_tengwar!(Gondor, "áth" => [CARRIER_LONG, TEHTA_A.base, TENGWA_THULE] == "âþ" == "āþ" == "aaþ");
    test_tengwar!(Gondor, "éth" => [TENGWA_THULE, TEHTA_E.base, TEHTA_E.base] == "êþ" == "ēþ" == "eeþ");
    test_tengwar!(Gondor, "íth" => [CARRIER_LONG, TEHTA_I.base, TENGWA_THULE] == "îþ" == "īþ" == "iiþ");
    test_tengwar!(Gondor, "óth" => [TENGWA_THULE, TEHTA_O.base, TEHTA_O.base] == "ôþ" == "ōþ" == "ooþ");
    test_tengwar!(Gondor, "úth" => [TENGWA_THULE, TEHTA_U.base, TEHTA_U.base] == "ûþ" == "ūþ" == "uuþ");
    test_tengwar!(Gondor, "ýth" => [CARRIER_LONG, TEHTA_Y.base, TENGWA_THULE] == "ŷþ" == "ȳþ" == "yyþ");

    //  Test alternate styles of long vowels.
    test_tengwar!(Gondor[vowels=Separate], "ath" => [TENGWA_THULE, TEHTA_A.base]);
    test_tengwar!(Gondor[vowels=Separate], "eth" => [TENGWA_THULE, TEHTA_E.base]);
    test_tengwar!(Gondor[vowels=Separate], "ith" => [TENGWA_THULE, TEHTA_I.base]);
    test_tengwar!(Gondor[vowels=Separate], "oth" => [TENGWA_THULE, TEHTA_O.base]);
    test_tengwar!(Gondor[vowels=Separate], "uth" => [TENGWA_THULE, TEHTA_U.base]);
    test_tengwar!(Gondor[vowels=Separate], "yth" => [TENGWA_THULE, TEHTA_Y.base]);
    test_tengwar!(Gondor[vowels=Doubled], "ath" => [TENGWA_THULE, TEHTA_A.base]);
    test_tengwar!(Gondor[vowels=Doubled], "eth" => [TENGWA_THULE, TEHTA_E.base]);
    test_tengwar!(Gondor[vowels=Doubled], "ith" => [TENGWA_THULE, TEHTA_I.base]);
    test_tengwar!(Gondor[vowels=Doubled], "oth" => [TENGWA_THULE, TEHTA_O.base]);
    test_tengwar!(Gondor[vowels=Doubled], "uth" => [TENGWA_THULE, TEHTA_U.base]);
    test_tengwar!(Gondor[vowels=Doubled], "yth" => [TENGWA_THULE, TEHTA_Y.base]);
    test_tengwar!(Gondor[vowels=Unique], "ath" => [TENGWA_THULE, TEHTA_A.base]);
    test_tengwar!(Gondor[vowels=Unique], "eth" => [TENGWA_THULE, TEHTA_E.base]);
    test_tengwar!(Gondor[vowels=Unique], "ith" => [TENGWA_THULE, TEHTA_I.base]);
    test_tengwar!(Gondor[vowels=Unique], "oth" => [TENGWA_THULE, TEHTA_O.base]);
    test_tengwar!(Gondor[vowels=Unique], "uth" => [TENGWA_THULE, TEHTA_U.base]);
    test_tengwar!(Gondor[vowels=Unique], "yth" => [TENGWA_THULE, TEHTA_Y.base]);

    test_tengwar!(Gondor[vowels=Separate], "áth" => [CARRIER_LONG, TEHTA_A.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Separate], "éth" => [CARRIER_LONG, TEHTA_E.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Separate], "íth" => [CARRIER_LONG, TEHTA_I.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Separate], "óth" => [CARRIER_LONG, TEHTA_O.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Separate], "úth" => [CARRIER_LONG, TEHTA_U.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Separate], "ýth" => [CARRIER_LONG, TEHTA_Y.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Doubled], "áth" => [CARRIER_LONG, TEHTA_A.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Doubled], "éth" => [TENGWA_THULE, TEHTA_E.base, TEHTA_E.base]);
    test_tengwar!(Gondor[vowels=Doubled], "íth" => [CARRIER_LONG, TEHTA_I.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Doubled], "óth" => [TENGWA_THULE, TEHTA_O.base, TEHTA_O.base]);
    test_tengwar!(Gondor[vowels=Doubled], "úth" => [TENGWA_THULE, TEHTA_U.base, TEHTA_U.base]);
    test_tengwar!(Gondor[vowels=Doubled], "ýth" => [CARRIER_LONG, TEHTA_Y.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Unique], "áth" => [CARRIER_LONG, TEHTA_A.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Unique], "éth" => [TENGWA_THULE, TEHTA_E.alternate.unwrap()]);
    test_tengwar!(Gondor[vowels=Unique], "íth" => [CARRIER_LONG, TEHTA_I.base, TENGWA_THULE]);
    test_tengwar!(Gondor[vowels=Unique], "óth" => [TENGWA_THULE, TEHTA_O.alternate.unwrap()]);
    test_tengwar!(Gondor[vowels=Unique], "úth" => [TENGWA_THULE, TEHTA_U.alternate.unwrap()]);
    test_tengwar!(Gondor[vowels=Unique], "ýth" => [CARRIER_LONG, TEHTA_Y.base, TENGWA_THULE]);
}
