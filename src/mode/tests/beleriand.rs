use crate::mode::beleriand::*;
use super::*;


const LONG: char = ANDAITH.base;


#[test]
fn words() {
    test_tengwar!(Beleriand, "axë"
        => [VOWEL_A, TENGWA_CALMA, TENGWA_SILME, VOWEL_E]
        == "axe"
        == "acsë"
        == "aksë"
        != "akzë"
        != "akssë"
    );

    let _edhellen = test_tengwar!(Beleriand, "edhellen"
        => [
            VOWEL_E, TENGWA_ANTO, // edh
            VOWEL_E, TENGWA_LAMBE, TENGWA_LAMBE, // ell
            VOWEL_E, TENGWA_ORE, // en
        ]
        == "eðellen"
        == "EÐELLEN"
        != "edellen"
        != "eθellen"
        != "eþellen"
        != "ethellen"
    );

    let _andaith = test_tengwar!(Beleriand, "andaith"
        => [
            VOWEL_A, TENGWA_ANDO, DC_OVER_LINE, // and
            VOWEL_A, TEHTA_Y.base, // ai
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
    let _parf = test_tengwar!(Beleriand, "parf"
        => [TENGWA_PARMA, VOWEL_A, TENGWA_ROMEN, TENGWA_AMPA]
        == "parv"
        != "parφ"
        != "parph"
    );

    //  Final F, after vowel.
    let _alaf = test_tengwar!(Beleriand, "alaf"
        => [VOWEL_A, TENGWA_LAMBE, VOWEL_A, TENGWA_AMPA]
        == "alav"
        != "alaφ"
        != "alaph"
    );

    //  Medial F, after consonant.
    let _alfirin = test_tengwar!(Beleriand, "alfirin"
        => [
            VOWEL_A, TENGWA_LAMBE, // al
            TENGWA_FORMEN, // ph
            VOWEL_I, TENGWA_ROMEN, // ir
            VOWEL_I, TENGWA_ORE, // in
        ]
        == "alphirin"
        == "alφirin"
        == "ALΦIRIN"
        != "alvirin"
    );

    //  Medial F, after vowel.
    let _aphadon = test_tengwar!(Beleriand, "aphadon"
        => [
            VOWEL_A, TENGWA_FORMEN, // aph
            VOWEL_A, TENGWA_ANDO, // ad
            VOWEL_O, TENGWA_ORE, // on
        ]
        == "afadon"
        == "aφadon"
        == "AΦADON"
        != "avadon"
    );

    let _telch = test_tengwar!(Beleriand, "telch"
        => [TENGWA_TINCO, VOWEL_E, TENGWA_LAMBE, TENGWA_AHA]
        == "telkh"
        != "telgh"
    );

    let _calen = test_tengwar!(Beleriand, "calen"
        => [TENGWA_CALMA, VOWEL_A, TENGWA_LAMBE, VOWEL_E, TENGWA_ORE]
        == "kalen"
        != "çalen"
    );

    let _hebin = test_tengwar!(Beleriand, "hebin"
        => [TENGWA_HYARMEN, VOWEL_E, TENGWA_UMBAR, VOWEL_I, TENGWA_ORE]
    );

    let grist = test_tengwar!(Beleriand, "grist"
        => [TENGWA_ANGA, TENGWA_ROMEN, VOWEL_I, TENGWA_SILME, TENGWA_TINCO]
    );
    test_tengwar!(Beleriand[nuquerna=true], "grist" == grist);

    let _acharn = test_tengwar!(Beleriand, "acharn"
        => [VOWEL_A, TENGWA_AHA, VOWEL_A, TENGWA_ROMEN, TENGWA_ORE]
        == "akharn"
        != "agharn"
    );

    let _wethrin = test_tengwar!(Beleriand, "wethrin"
        => [TENGWA_WILYA, VOWEL_E, TENGWA_THULE, TENGWA_ROMEN, VOWEL_I, TENGWA_ORE]
    );

    //  Doubled nasals.
    let _venn = test_tengwar!(Beleriand, "venn"
        => [TENGWA_AMPA, VOWEL_E, TENGWA_NUMEN]
    );
    let _namma = test_tengwar!(Beleriand, "namma"
        => [TENGWA_ORE, VOWEL_A, TENGWA_MALTA, VOWEL_A]
    );

    //  Softened sounds.
    let _rhuun = test_tengwar!(Beleriand, "rhûn"
        => [TENGWA_ARDA, VOWEL_U, LONG, TENGWA_ORE]
    );
    let _lhuug = test_tengwar!(Beleriand, "lhûg"
        => [TENGWA_ALDA, VOWEL_U, LONG, TENGWA_ANGA]
    );
    let _mhellyn = test_tengwar!(Beleriand, "mhellyn"
        => [TENGWA_VALA_HOOKED, VOWEL_E, TENGWA_LAMBE, TENGWA_LAMBE, VOWEL_Y, TENGWA_ORE]
    );

    //  Consonantal initial I and medial RH.
    let _iorhael = test_tengwar!(Beleriand, "iorhael"
        => [
            TENGWA_ARA, VOWEL_O, TENGWA_ROMEN, // jor
            TENGWA_HYARMEN, VOWEL_A, VOWEL_E, TENGWA_LAMBE, // hael
        ]
        == "jorhæl"
        != "yorhael"
    );

    //  Non-consonantal I in the same vowel cluster.
    let _dior = test_tengwar!(Beleriand, "dior"
        => [TENGWA_ANDO, VOWEL_I, VOWEL_O, TENGWA_ROMEN]
        != "djor"
        != "dyor"
    );

    //  Non-consonantal initial I.
    let _ithil = test_tengwar!(Beleriand, "ithil"
        => [VOWEL_I, TENGWA_THULE, VOWEL_I, TENGWA_LAMBE]
        != "jthil"
        != "ythil"
    );
}


#[test]
fn vowels() {
    //  Test all diphthongs.
    test_tengwar!(Beleriand, "ae" => [VOWEL_A, VOWEL_E] == "æ" == "Æ");
    test_tengwar!(Beleriand, "oe" => [VOWEL_O, VOWEL_E] == "œ" == "Œ");
    test_tengwar!(Beleriand, "ai" => [VOWEL_A, TEHTA_Y.base]);
    test_tengwar!(Beleriand, "ei" => [VOWEL_E, TEHTA_Y.base]);
    test_tengwar!(Beleriand, "ui" => [VOWEL_U, TEHTA_Y.base]);
    test_tengwar!(Beleriand, "au" => [VOWEL_A, MOD_LABIAL] == "aw");

    //  Test all vowels, alone.
    test_tengwar!(Beleriand, "a" => [VOWEL_A]);
    test_tengwar!(Beleriand, "e" => [VOWEL_E]);
    test_tengwar!(Beleriand, "i" => [VOWEL_I]);
    test_tengwar!(Beleriand, "o" => [VOWEL_O]);
    test_tengwar!(Beleriand, "u" => [VOWEL_U]);
    test_tengwar!(Beleriand, "y" => [VOWEL_Y]);
    test_tengwar!(Beleriand, "á" => [VOWEL_A, LONG] == "â" == "aa");
    test_tengwar!(Beleriand, "é" => [VOWEL_E, LONG] == "ê" == "ee");
    test_tengwar!(Beleriand, "í" => [VOWEL_I, LONG] == "î" == "ii");
    test_tengwar!(Beleriand, "ó" => [VOWEL_O, LONG] == "ô" == "oo");
    test_tengwar!(Beleriand, "ú" => [VOWEL_U, LONG] == "û" == "uu");
    test_tengwar!(Beleriand, "ý" => [VOWEL_Y, LONG] == "ŷ" == "yy");

    //  Test all vowels, before consonants.
    test_tengwar!(Beleriand, "ath" => [VOWEL_A, TENGWA_THULE]);
    test_tengwar!(Beleriand, "eth" => [VOWEL_E, TENGWA_THULE]);
    test_tengwar!(Beleriand, "ith" => [VOWEL_I, TENGWA_THULE]);
    test_tengwar!(Beleriand, "oth" => [VOWEL_O, TENGWA_THULE]);
    test_tengwar!(Beleriand, "uth" => [VOWEL_U, TENGWA_THULE]);
    test_tengwar!(Beleriand, "yth" => [VOWEL_Y, TENGWA_THULE]);
    test_tengwar!(Beleriand, "áth" => [VOWEL_A, LONG, TENGWA_THULE] == "âth" == "aath");
    test_tengwar!(Beleriand, "éth" => [VOWEL_E, LONG, TENGWA_THULE] == "êth" == "eeth");
    test_tengwar!(Beleriand, "íth" => [VOWEL_I, LONG, TENGWA_THULE] == "îth" == "iith");
    test_tengwar!(Beleriand, "óth" => [VOWEL_O, LONG, TENGWA_THULE] == "ôth" == "ooth");
    test_tengwar!(Beleriand, "úth" => [VOWEL_U, LONG, TENGWA_THULE] == "ûth" == "uuth");
    test_tengwar!(Beleriand, "ýth" => [VOWEL_Y, LONG, TENGWA_THULE] == "ŷth" == "yyth");
}
