use crate::mode::quenya::*;
use super::*;


#[test]
fn alt_a() {
    let rauca = test_tengwar!(Quenya, "rauca" == "rauka" => [
        TENGWA_ROMEN, // r
        CARRIER_DIPH_U, TEHTA_A.base, // au
        TENGWA_CALMA, TEHTA_A.base, // ca
    ]);
    test_tengwar!(Quenya[alt_a=true], "rauca" == "rauka" != rauca => [
        TENGWA_ROMEN, // r
        CARRIER_DIPH_U, TEHTA_YANTA.base, // au
        TENGWA_CALMA, TEHTA_YANTA.base, // ca
    ]);
}


#[test]
fn alt_rince() {
    //  Check final basic against final alternate on T.
    let otso = test_tengwar!(Quenya, "otso" == "otzo" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, SA_RINCE, TEHTA_O.base, // tso
    ]);
    test_tengwar!(Quenya[alt_rince=true], "otso" == "otzo" != otso => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, TEHTA_O.base, SA_RINCE_FINAL, // tso
    ]);

    //  Check nonfinal basic against nonfinal alternate on T.
    let otsor = test_tengwar!(Quenya, "otsor" == "otzor" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, SA_RINCE, TEHTA_O.base, // tso
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya[alt_rince=true], "otsor" == "otzor" == otsor);

    //  Check final basic against final alternate on K.
    let mixa = test_tengwar!(Quenya, "mixa"
        => [
            TENGWA_MALTA, TEHTA_I.base, // mi
            TENGWA_CALMA, SA_RINCE, TEHTA_A.base, // xa
        ]
        == "micsa"
        == "miksa"
        == "mikza"
    );
    test_tengwar!(Quenya[alt_rince=true], mixa
        == "mixa"
        == "micsa"
        == "miksa"
        == "mikza"
    );

    //  Check nonfinal basic against nonfinal alternate on K.
    let mixar = test_tengwar!(Quenya, "mixar"
        => [
            TENGWA_MALTA, TEHTA_I.base, // mi
            TENGWA_CALMA, SA_RINCE, TEHTA_A.base, // xa
            TENGWA_ORE, // r
        ]
        == "micsar"
        == "miksar"
        == "mikzar"
    );
    test_tengwar!(Quenya[alt_rince=true], mixar
        == "mixar"
        == "micsar"
        == "miksar"
        == "mikzar"
    );

    //  Confirm that a vowel interrupts rincë.
    let tas = test_tengwar!(Quenya, "tas" => [
        TENGWA_TINCO, TEHTA_A.base, // ta
        TENGWA_SILME, // s
    ]);
    test_tengwar!(Quenya[alt_rince=true], "tas" == tas);
    test_tengwar!(Quenya, "tsa" => [
        TENGWA_TINCO, SA_RINCE, TEHTA_A.base, // tsa
    ]);
    test_tengwar!(Quenya[alt_rince=true], "tsa" => [
        TENGWA_TINCO, TEHTA_A.base, SA_RINCE_FINAL, // tsa
    ]);
}


#[test]
fn elision() {
    test_tengwar!(Quenya, "alda" => [
        CARRIER_SHORT, TEHTA_A.base, // a
        TENGWA_ALDA, TEHTA_A.base, // lda
    ]);
    test_tengwar!(Quenya, "ʒalda" => [
        TENGWA_ANNA, TEHTA_A.base, // ʒa
        TENGWA_ALDA, TEHTA_A.base, // lda
    ]);
    test_tengwar!(Quenya[elide_a=true], "alda" => [CARRIER_SHORT, TENGWA_ALDA]);
    test_tengwar!(Quenya[elide_a=true], "ʒalda" => [TENGWA_ANNA, TENGWA_ALDA]);

    test_tengwar!(Quenya, "calma" => [
        TENGWA_CALMA, TEHTA_A.base, // ca
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_A.base, // ma
    ]);
    test_tengwar!(Quenya[dot_plain=true], "calma" => [
        TENGWA_CALMA, TEHTA_A.base, // ca
        TENGWA_LAMBE, DC_UNDER_DOT_1, // l
        TENGWA_MALTA, TEHTA_A.base, // ma
    ]);
    test_tengwar!(Quenya[elide_a=true], "calma" => [
        TENGWA_CALMA, TENGWA_LAMBE, TENGWA_MALTA,
    ]);
    test_tengwar!(Quenya[dot_plain=true, elide_a=true], "calma" => [
        TENGWA_CALMA, TENGWA_LAMBE, DC_UNDER_DOT_1, TENGWA_MALTA,
    ]);

    test_tengwar!(Quenya, "hárar" => [
        TENGWA_HYARMEN, // h
        CARRIER_LONG, TEHTA_A.base, // á
        TENGWA_ROMEN, TEHTA_A.base, // ra
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya[elide_a=true], "hárar" => [
        TENGWA_HYARMEN, // h
        CARRIER_LONG, // á
        TENGWA_ROMEN, // ra
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya[dot_plain=true, elide_a=true], "hárar" => [
        TENGWA_HYARMEN, DC_UNDER_DOT_1, // h
        CARRIER_LONG, // á
        TENGWA_ROMEN, // ra
        TENGWA_ORE, DC_UNDER_DOT_1, // r
    ]);

    test_tengwar!(Quenya, "airë" => [
        TENGWA_YANTA, TEHTA_A.base, // ai
        TENGWA_ROMEN, TEHTA_E.base, // rë
    ]);
    test_tengwar!(Quenya[elide_a=true], "airë" => [
        TENGWA_YANTA, // ai
        TENGWA_ROMEN, TEHTA_E.base, // rë
    ]);

    test_tengwar!(Quenya, "aulë" => [
        TENGWA_URE, TEHTA_A.base, // au
        TENGWA_LAMBE, TEHTA_E.base, // lë
    ]);
    test_tengwar!(Quenya[elide_a=true], "aulë" => [
        TENGWA_URE, // au
        TENGWA_LAMBE, TEHTA_E.base, // lë
    ]);
}


#[test]
fn ligatures() {
    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=0], "ista" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=0], "ista" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=1], "ista" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=1], "ista" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=2], "ista" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        TENGWA_SILME, ZWJ, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=2], "ista" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        TENGWA_SILME, ZWJ, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=3], "ista" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        TENGWA_SILME, ZWJ, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=3], "ista" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        TENGWA_SILME, ZWJ, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    test_tengwar!(Quenya[ligate_zwj=0], "ohta" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_AHA, // h
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_zwj=1], "ohta" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_AHA, // h
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_zwj=2], "ohta" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_AHA, // h
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);
    test_tengwar!(Quenya[ligate_zwj=3], "ohta" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_AHA, ZWJ, // h
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    //  Test short ligatures thoroughly against regular tengwar.
    {
        {
            test_tengwar!(Quenya[ligate_short=true], "etë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_TINCO, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "epë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_PARMA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "ecë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_CALMA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "eqë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_QESSE, TEHTA_E.base,
            ]);
        }
        {
            test_tengwar!(Quenya[ligate_short=true], "endë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_ANDO, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "embë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_UMBAR, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "engë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_ANGA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "engwë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_UNGWE, TEHTA_E.base,
            ]);
        }
        {
            test_tengwar!(Quenya[ligate_short=true], "eþë" => [
                CARRIER_SHORT, TEHTA_E.base,
                TENGWA_THULE, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "efë" => [
                CARRIER_SHORT, TEHTA_E.base,
                TENGWA_FORMEN, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "ehë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_AHA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "ehwë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_HWESTA, TEHTA_E.base,
            ]);
        }
        {
            test_tengwar!(Quenya[ligate_short=true], "entë" => [
                CARRIER_SHORT, TEHTA_E.base,
                TENGWA_ANTO, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "empë" => [
                CARRIER_SHORT, TEHTA_E.base,
                TENGWA_AMPA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "encë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_ANCA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "enqë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_UNQUE, TEHTA_E.base,
            ]);
        }
        {
            test_tengwar!(Quenya[ligate_short=true], "enë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_NUMEN, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "emë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_MALTA, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "eñë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_NOLDO, TEHTA_E.base,
            ]);
            test_tengwar!(Quenya[ligate_short=true], "eñwë" => [
                CARRIER_SHORT_LIG, TEHTA_E.base,
                TENGWA_NWALME, TEHTA_E.base,
            ]);
        }
    }
}


#[test]
fn normalization() {
    test_tengwar!(Quenya, "ñávëa"
        //  Standard codepoints.
        => [
            TENGWA_NOLDO, // ñ
            CARRIER_LONG, TEHTA_A.base, // á
            TENGWA_VALA, TEHTA_E.base, // vë
            CARRIER_SHORT, TEHTA_A.base, // a
        ]
        //  Combining diacritic equivalents.
        == "ñávëa"
        == "ÑÁVËA"
    );

    test_tengwar!(Quenya, "ñólë"
        //  Standard codepoints.
        => [
            TENGWA_NOLDO, TEHTA_O.base, TEHTA_O.base, // ñó
            TENGWA_LAMBE, TEHTA_E.base, // lë
        ]
        //  Combining diacritic equivalents.
        == "ñólë" // Acute accent (U+0301).
        == "ñólë" // Acute tone mark (U+0341).
        == "ÑÓLË"
    );
}


#[test]
fn numerals() {
    use numeral::*;

    //  Test Base-12.
    test_tengwar!(Quenya, "0" => [NUM_0, BASE_12_DOT]);
    test_tengwar!(Quenya, "1" => [NUM_1, BASE_12_DOT]);
    test_tengwar!(Quenya, "9" => [NUM_9, BASE_12_DOT]);
    test_tengwar!(Quenya, "10" => [NUM_A, BASE_12_DOT]);
    test_tengwar!(Quenya, "11" => [NUM_B, BASE_12_DOT]);
    // test_tengwar!(Quenya, "12" => [NUM_C, BASE_12_DOT]);
    test_tengwar!(Quenya, "12" => [
        NUM_0, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_1, BASE_12_DOT,
    ]);
    test_tengwar!(Quenya, "24" => [
        NUM_0, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_2, BASE_12_DOT,
    ]);
    test_tengwar!(Quenya, "25" => [
        NUM_1, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_2, BASE_12_DOT,
    ]);
    test_tengwar!(Quenya, "144" => [
        NUM_0, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_0, BASE_12_DOT,
        NUM_1, BASE_12_DOT,
    ]);
    test_tengwar!(Quenya, "171" => [
        NUM_3, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_2, BASE_12_DOT,
        NUM_1, BASE_12_DOT,
    ]);

    //  Test Base-10.
    test_tengwar!(Quenya, "#1" => [NUM_1, BASE_10_DOT]);
    test_tengwar!(Quenya, "#9" => [NUM_9, BASE_10_DOT]);
    test_tengwar!(Quenya, "#10" => [
        NUM_0, MOD_UNITS, BASE_10_DOT,
        NUM_1, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#20" => [
        NUM_0, MOD_UNITS, BASE_10_DOT,
        NUM_2, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#21" => [
        NUM_1, MOD_UNITS, BASE_10_DOT,
        NUM_2, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#100" => [
        NUM_0, MOD_UNITS, BASE_10_DOT,
        NUM_0, BASE_10_DOT,
        NUM_1, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#123" => [
        NUM_3, MOD_UNITS, BASE_10_DOT,
        NUM_2, BASE_10_DOT,
        NUM_1, BASE_10_DOT,
    ]);

    //  Test leading zeroes.
    //  TODO: Implement support, or decide not to.
    test_tengwar!(Quenya, "000" => [NUM_0, /*NUM_0, NUM_0,*/ BASE_12_DOT]);
    test_tengwar!(Quenya, "001" => [NUM_1, /*NUM_0, NUM_0,*/ BASE_12_DOT]);

    //  Test negatives.
    test_tengwar!(Quenya, "#-0" => [
        /*Numeral::PREF_NEG_OUT,*/
        NUM_0, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#-1" => [
        Numeral::PREF_NEG_OUT,
        NUM_1, BASE_10_DOT,
    ]);
    test_tengwar!(Quenya, "#-10" => [
        Numeral::PREF_NEG_OUT,
        NUM_0, MOD_UNITS, BASE_10_DOT,
        NUM_1, BASE_10_DOT,
    ]);

    //  Test ordinals.
    test_tengwar!(Quenya, "#1@" => [
        NUM_1, BASE_10_DOT,
        Numeral::SUFF_ORD_OUT,
    ]);

    //  Test negative ordinals.
    test_tengwar!(Quenya, "#-1@" => [
        Numeral::PREF_NEG_OUT,
        NUM_1, BASE_10_DOT,
        Numeral::SUFF_ORD_OUT,
    ]);

    //  Test sequence indices (one digit).
    test_tengwar!(Quenya, "/#" => ['/', SUFF_SEQ_IN]);
    test_tengwar!(Quenya, "0#" => [NUM_0, BASE_12_DOT, SUFF_SEQ_IN]);
    test_tengwar!(Quenya, "1#" => [SEQUENCE[0]]);
    test_tengwar!(Quenya, "9#" => [SEQUENCE[8]]);
    test_tengwar!(Quenya, ":#" => [PUNCT_DOT_2, SUFF_SEQ_IN]);

    //  Test sequence indices (two digits).
    test_tengwar!(Quenya, "00#" => [NUM_0, BASE_12_DOT, SUFF_SEQ_IN]);
    test_tengwar!(Quenya, "01#" => [SEQUENCE[0]]);
    test_tengwar!(Quenya, "09#" => [SEQUENCE[8]]);
    test_tengwar!(Quenya, "10#" => [SEQUENCE[9]]);
    test_tengwar!(Quenya, "11#" => [SEQUENCE[10]]);
    test_tengwar!(Quenya, "24#" => [SEQUENCE[23]]);
    test_tengwar!(Quenya, "25#" => [
        NUM_1, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_2, BASE_12_DOT,
        SUFF_SEQ_IN,
    ]);
    test_tengwar!(Quenya, "30#" => [
        NUM_6, MOD_UNITS, /*BASE_12_DOT,*/
        NUM_2, BASE_12_DOT,
        SUFF_SEQ_IN,
    ]);
}


#[test]
fn nuquernar() {
    //  Check Silmë.
    let silme = test_tengwar!(Quenya, "silmë" => [
        TENGWA_SILME, TEHTA_I.base, // si
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_E.base, // më
    ]);
    test_tengwar!(Quenya[nuquerna=true], "silmë" != silme => [
        TENGWA_SILME_NUQ, TEHTA_I.base, // si
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_E.base, // më
    ]);

    //  Check Essë.
    let esse = test_tengwar!(Quenya, "essë"
        => [CARRIER_SHORT, TEHTA_E.base, TENGWA_ESSE, TEHTA_E.base]
        == "eze"
        == "eße"
        != "ese"
    );
    test_tengwar!(Quenya[nuquerna=true], "essë" != esse
        => [CARRIER_SHORT, TEHTA_E.base, TENGWA_ESSE_NUQ, TEHTA_E.base]
        == "eze"
        == "eße"
        != "ese"
    );

    //  Confirm lack of Nuquerna for a vowel on Ára.
    let siila = test_tengwar!(Quenya, "síla" => [
        TENGWA_SILME, // s
        CARRIER_LONG, TEHTA_I.base, // í
        TENGWA_LAMBE, TEHTA_A.base, // la
    ]);
    test_tengwar!(Quenya[nuquerna=true], "síla" == siila);
}


#[test]
fn words() {
    test_tengwar!(Quenya, "eleni sílar"
        => [
            CARRIER_SHORT, TEHTA_E.base, // e
            TENGWA_LAMBE, TEHTA_E.base, // le
            TENGWA_NUMEN, TEHTA_I.base, // ni
            ' ',
            TENGWA_SILME, // s
            CARRIER_LONG, TEHTA_I.base, // í
            TENGWA_LAMBE, TEHTA_A.base, // la
            TENGWA_ORE, // r
        ]
        == "Eleni Sílar"
        == "Elënï Sílär"
        == "ELËNÏ SÍLÄR"
        == "ELeNi SiiLaR"
        == "ELENI SIILAR"
    );

    test_tengwar!(Quenya, "Elen síla lúmenn' omentielvo :" => [
        CARRIER_SHORT, TEHTA_E.base, // e
        TENGWA_LAMBE, TEHTA_E.base, // le
        TENGWA_NUMEN, // n
        ' ',
        TENGWA_SILME, // s
        CARRIER_LONG, TEHTA_I.base, // í
        TENGWA_LAMBE, TEHTA_A.base, // la
        ' ',
        TENGWA_LAMBE, TEHTA_U.base, TEHTA_U.base, // lú
        TENGWA_MALTA, TEHTA_E.base, // me
        TENGWA_NUMEN, DC_UNDER_LINE_H, // nn
        PUNCT_DOT_1, // '
        ' ',
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_MALTA, TEHTA_E.base, // me
        TENGWA_ANTO, TEHTA_I.base, // nti
        CARRIER_SHORT, TEHTA_E.base, // e
        TENGWA_LAMBE, // l
        TENGWA_VALA, TEHTA_O.base, // vo
        ' ', PUNCT_DOT_2,
    ]);

    test_tengwar!(Quenya, "helcaraxë"
        => [
            TENGWA_HYARMEN, TEHTA_E.base, // he
            TENGWA_LAMBE, // l
            TENGWA_CALMA, TEHTA_A.base, // ca
            TENGWA_ROMEN, TEHTA_A.base, // ra
            TENGWA_CALMA, SA_RINCE, TEHTA_E.base, // xë
        ]
        == "helkaracse"
        == "helkarakse"
    );

    let _quenya = test_tengwar!(Quenya, "quenya"
        => [TENGWA_QESSE, TEHTA_E.base, TENGWA_NUMEN, MOD_PALATAL, TEHTA_A.base]
        == "qenya"
        == "kwenya"
        == "cwenya"
        != "çwenya"
        != "kuenya"
        != "cuenya"
        != "çuenya"
    );

    let _aha = test_tengwar!(Quenya, "aha"
        => [CARRIER_SHORT, TEHTA_A.base, TENGWA_AHA, TEHTA_A.base]
        //  The following two are not the same sound as above, but were an
        //      archaic sound, spelled the same as the modern breath-H.
        == "acha"
        == "akha"
        != "agha"
    );

    let _nahta = test_tengwar!(Quenya, "nahta"
        => [TENGWA_NUMEN, TEHTA_A.base, TENGWA_AHA, TENGWA_TINCO, TEHTA_A.base]
        //  The following two ARE the same sound; This is the only cluster that
        //      is still pronounced this way.
        == "nachta"
        == "nakhta"
        != "naghta"
    );

    let hyarmen = test_tengwar!(Quenya, "hyarmen"
        => [
            TENGWA_HYARMEN, MOD_PALATAL, TEHTA_A.base, // hya
            TENGWA_ORE, // r
            TENGWA_MALTA, TEHTA_E.base, // me
            TENGWA_NUMEN, // n
        ]
        != "chyarmen"
    );
    let _khyarmen = test_tengwar!(Quenya, "khyarmen"
        => [
            TENGWA_AHA, MOD_PALATAL, TEHTA_A.base, // khya
            TENGWA_ORE, // r
            TENGWA_MALTA, TEHTA_E.base, // me
            TENGWA_NUMEN, // n
        ]
        == "chyarmen"
        != hyarmen
    );

    let he = test_tengwar!(Quenya, "hë" => [TENGWA_HYARMEN, TEHTA_E.base]);
    let khe = test_tengwar!(Quenya, "khë" => [TENGWA_AHA, TEHTA_E.base]);
    test_tengwar!(Quenya, "chë" == khe);
    test_tengwar!(Quenya, "chë" != he);
    test_tengwar!(Quenya, "ghë" != he);
    test_tengwar!(he != khe);

    let _hwesta = test_tengwar!(Quenya, "hwesta"
        => [TENGWA_HWESTA, TEHTA_E.base, TENGWA_SILME, TENGWA_TINCO, TEHTA_A.base]
        != "chwesta"
        != "khwesta"
        != "ghwesta"
    );

    let _aara = test_tengwar!(Quenya, "ára"
        => [CARRIER_LONG, TEHTA_A.base, TENGWA_ROMEN, TEHTA_A.base]
        == "aara" // ASCII spelling.
    );

    //  Archaic TH (> S).
    let _thuule = test_tengwar!(Quenya, "þúlë"
        => [TENGWA_THULE, TEHTA_U.base, TEHTA_U.base, TENGWA_LAMBE, TEHTA_E.base]
        == "thuule" // ASCII spelling.
        == "θúlë"
        == "ΘÚLË"
        == "ÞÚLË"
        != "súlë"
    );

    let _calma = test_tengwar!(Quenya, "calma"
        => [TENGWA_CALMA, TEHTA_A.base, TENGWA_LAMBE, TENGWA_MALTA, TEHTA_A.base]
        == "kalma"
        != "qalma"
        != "çalma"
    );

    //  Initial and final N.
    let _nuumen = test_tengwar!(Quenya, "númen"
        => [
            TENGWA_NUMEN, TEHTA_U.base, TEHTA_U.base, // nú
            TENGWA_MALTA, TEHTA_E.base, // me
            TENGWA_NUMEN, // n
        ]
        == "nuumen" // ASCII spelling.
        != "ñuumen"
        != "ngúmen"
    );

    //  Initial NG (> N).
    let _ngoldo = test_tengwar!(Quenya, "ñoldo"
        => [TENGWA_NOLDO, TEHTA_O.base, TENGWA_ALDA, TEHTA_O.base]
        == "ngoldo" // ASCII spelling.
        == "ÑOLDO"
        != "noldo"
    );

    //  Initial NG, appearing medially due to concatenation.
    let _etya_ngoldo = test_tengwar!(Quenya, "etyañoldo"
        => [
            CARRIER_SHORT, TEHTA_E.base, // e
            TENGWA_TINCO, MOD_PALATAL, TEHTA_A.base, // tya
            TENGWA_NOLDO, TEHTA_O.base, // ño
            TENGWA_ALDA, TEHTA_O.base, // ldo
        ]
        == r"etya\ ngoldo" // ASCII spelling.
        == r"etya\ ñoldo"
        != r"etyangoldo"
    );

    //  Initial NGW (> NW).
    let _ngwalme = test_tengwar!(Quenya, "ñwalmë"
        => [TENGWA_NWALME, TEHTA_A.base, TENGWA_LAMBE, TENGWA_MALTA, TEHTA_E.base]
        == "ngwalme" // ASCII spelling.
        == "nwalmë"
    );

    //  Medial NG.
    let _anga = test_tengwar!(Quenya, "anga"
        => [CARRIER_SHORT, TEHTA_A.base, TENGWA_ANGA, TEHTA_A.base]
        != "aña"
        != "ana"
    );

    //  Medial NGW.
    let _ungwe = test_tengwar!(Quenya, "ungwë"
        => [CARRIER_SHORT, TEHTA_U.base, TENGWA_UNGWE, TEHTA_E.base]
        == "ungwe" // ASCII spelling.
        != "uñwë"
        != "unwë"
    );

    test_tengwar!(Quenya, "hrívë" => [
        TENGWA_HALLA, // h
        TENGWA_ROMEN, CARRIER_LONG, TEHTA_I.base, // rí
        TENGWA_VALA, TEHTA_E.base, // vë
    ]);
    test_tengwar!(Quenya, "hlócë" => [
        TENGWA_HALLA, // h
        TENGWA_LAMBE, TEHTA_O.base, TEHTA_O.base, // ló
        TENGWA_CALMA, TEHTA_E.base, // cë
    ]);
}


#[test]
fn vowels() {
    //  Test all diphthongs.
    test_tengwar!(Quenya, "ai" => [CARRIER_DIPH_I, TEHTA_A.base]);
    test_tengwar!(Quenya, "oi" => [CARRIER_DIPH_I, TEHTA_O.base]);
    test_tengwar!(Quenya, "ui" => [CARRIER_DIPH_I, TEHTA_U.base]);
    test_tengwar!(Quenya, "au" => [CARRIER_DIPH_U, TEHTA_A.base]);
    test_tengwar!(Quenya, "eu" => [CARRIER_DIPH_U, TEHTA_E.base]);
    test_tengwar!(Quenya, "iu" => [CARRIER_DIPH_U, TEHTA_I.base]);

    //  Test all vowels, alone.
    test_tengwar!(Quenya, "a" => [CARRIER_SHORT, TEHTA_A.base]);
    test_tengwar!(Quenya, "e" => [CARRIER_SHORT, TEHTA_E.base]);
    test_tengwar!(Quenya, "i" => [CARRIER_SHORT, TEHTA_I.base]);
    test_tengwar!(Quenya, "o" => [CARRIER_SHORT, TEHTA_O.base]);
    test_tengwar!(Quenya, "u" => [CARRIER_SHORT, TEHTA_U.base]);
    test_tengwar!(Quenya, "á" => [CARRIER_LONG, TEHTA_A.base] == "aa");
    test_tengwar!(Quenya, "é" => [CARRIER_LONG, TEHTA_E.base] == "ee");
    test_tengwar!(Quenya, "í" => [CARRIER_LONG, TEHTA_I.base] == "ii");
    test_tengwar!(Quenya, "ó" => [CARRIER_LONG, TEHTA_O.base] == "oo");
    test_tengwar!(Quenya, "ú" => [CARRIER_LONG, TEHTA_U.base] == "uu");

    //  Test all vowels, after consonants.
    test_tengwar!(Quenya, "la" => [TENGWA_LAMBE, TEHTA_A.base]);
    test_tengwar!(Quenya, "le" => [TENGWA_LAMBE, TEHTA_E.base]);
    test_tengwar!(Quenya, "li" => [TENGWA_LAMBE, TEHTA_I.base]);
    test_tengwar!(Quenya, "lo" => [TENGWA_LAMBE, TEHTA_O.base]);
    test_tengwar!(Quenya, "lu" => [TENGWA_LAMBE, TEHTA_U.base]);
    test_tengwar!(Quenya, "lá" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_A.base] == "laa");
    test_tengwar!(Quenya, "lé" => [TENGWA_LAMBE, TEHTA_E.base, TEHTA_E.base] == "lee");
    test_tengwar!(Quenya, "lí" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_I.base] == "lii");
    test_tengwar!(Quenya, "ló" => [TENGWA_LAMBE, TEHTA_O.base, TEHTA_O.base] == "loo");
    test_tengwar!(Quenya, "lú" => [TENGWA_LAMBE, TEHTA_U.base, TEHTA_U.base] == "luu");

    test_tengwar!(Quenya[vowels=Separate], "lá" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_A.base]);
    test_tengwar!(Quenya[vowels=Separate], "lé" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_E.base]);
    test_tengwar!(Quenya[vowels=Separate], "lí" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_I.base]);
    test_tengwar!(Quenya[vowels=Separate], "ló" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_O.base]);
    test_tengwar!(Quenya[vowels=Separate], "lú" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_U.base]);

    test_tengwar!(Quenya[vowels=Doubled], "lá" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_A.base]);
    test_tengwar!(Quenya[vowels=Doubled], "lé" => [TENGWA_LAMBE, TEHTA_E.base, TEHTA_E.base]);
    test_tengwar!(Quenya[vowels=Doubled], "lí" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_I.base]);
    test_tengwar!(Quenya[vowels=Doubled], "ló" => [TENGWA_LAMBE, TEHTA_O.base, TEHTA_O.base]);
    test_tengwar!(Quenya[vowels=Doubled], "lú" => [TENGWA_LAMBE, TEHTA_U.base, TEHTA_U.base]);

    test_tengwar!(Quenya[vowels=Unique], "lá" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_A.base]);
    test_tengwar!(Quenya[vowels=Unique], "lé" => [TENGWA_LAMBE, TEHTA_E.alternate.unwrap()]);
    test_tengwar!(Quenya[vowels=Unique], "lí" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_I.base]);
    test_tengwar!(Quenya[vowels=Unique], "ló" => [TENGWA_LAMBE, TEHTA_O.alternate.unwrap()]);
    test_tengwar!(Quenya[vowels=Unique], "lú" => [TENGWA_LAMBE, TEHTA_U.alternate.unwrap()]);
}
