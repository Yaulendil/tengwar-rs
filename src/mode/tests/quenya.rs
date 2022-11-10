use crate::mode::quenya::*;
use super::*;


#[test]
fn alt_a() {
    let rauca = test_tengwar!(Quenya, "rauca" => [
        TENGWA_ROMEN, // r
        CARRIER_DIPH_U, TEHTA_A.base, // au
        TENGWA_CALMA, TEHTA_A.base, // ca
    ]);
    test_tengwar!(Quenya, "rauka" == rauca);

    let rauca_alt = test_tengwar!(Quenya[alt_a=true], "rauca" => [
        TENGWA_ROMEN, // r
        CARRIER_DIPH_U, TEHTA_YANTA.base, // au
        TENGWA_CALMA, TEHTA_YANTA.base, // ca
    ]);
    test_tengwar!(Quenya[alt_a=true], "rauka" == rauca_alt);
}


#[test]
fn alt_rince() {
    //  Check final basic against final alternate on T.
    let otso = test_tengwar!(Quenya, "otso" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, TEHTA_O.base, SA_RINCE, // tso
    ]);
    let otso_alt = test_tengwar!(Quenya[alt_rince=true], "otso" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, TEHTA_O.base, SA_RINCE_FINAL, // tso
    ]);
    test_tengwar!(Quenya, "otzo" == otso);
    test_tengwar!(Quenya[alt_rince=true], "otzo" == otso_alt);

    //  Check nonfinal basic against nonfinal alternate on T.
    let otsor = test_tengwar!(Quenya, "otsor" => [
        CARRIER_SHORT, TEHTA_O.base, // o
        TENGWA_TINCO, TEHTA_O.base, SA_RINCE, // tso
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya, "otzor" == otsor);
    test_tengwar!(Quenya[alt_rince=true], "otsor" == otsor);
    test_tengwar!(Quenya[alt_rince=true], "otzor" == otsor);

    //  Check final basic against final alternate on K.
    let mixa = test_tengwar!(Quenya, "mixa" => [
        TENGWA_MALTA, TEHTA_I.base, // mi
        TENGWA_CALMA, TEHTA_A.base, SA_RINCE, // xa
    ]);
    test_tengwar!(Quenya, "micsa" == mixa);
    test_tengwar!(Quenya, "miksa" == mixa);
    test_tengwar!(Quenya, "mikza" == mixa);
    test_tengwar!(Quenya[alt_rince=true], "mixa" == mixa);
    test_tengwar!(Quenya[alt_rince=true], "micsa" == mixa);
    test_tengwar!(Quenya[alt_rince=true], "miksa" == mixa);
    test_tengwar!(Quenya[alt_rince=true], "mikza" == mixa);

    //  Check nonfinal basic against nonfinal alternate on K.
    let mixar = test_tengwar!(Quenya, "mixar" => [
        TENGWA_MALTA, TEHTA_I.base, // mi
        TENGWA_CALMA, TEHTA_A.base, SA_RINCE, // xa
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya, "micsar" == mixar);
    test_tengwar!(Quenya, "miksar" == mixar);
    test_tengwar!(Quenya, "mikzar" == mixar);
    test_tengwar!(Quenya[alt_rince=true], "mixar" == mixar);
    test_tengwar!(Quenya[alt_rince=true], "micsar" == mixar);
    test_tengwar!(Quenya[alt_rince=true], "miksar" == mixar);
    test_tengwar!(Quenya[alt_rince=true], "mikzar" == mixar);
}


#[test]
fn ligatures() {
    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=false], "istar" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=false], "istar" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
        TENGWA_ORE, // r
    ]);

    test_tengwar!(Quenya[ligate_short=false, ligate_zwj=true], "istar" => [
        CARRIER_SHORT, TEHTA_I.base, // i
        ZWJ, TENGWA_SILME, // s
        ZWJ, TENGWA_TINCO, TEHTA_A.base, // ta
        ZWJ, TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya[ligate_short=true, ligate_zwj=true], "istar" => [
        CARRIER_SHORT_LIG, TEHTA_I.base, // i
        ZWJ, TENGWA_SILME, // s
        ZWJ, TENGWA_TINCO, TEHTA_A.base, // ta
        ZWJ, TENGWA_ORE, // r
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
fn nuquernar() {
    //  Check Silmë.
    let _silme = test_tengwar!(Quenya, "silmë" => [
        TENGWA_SILME, TEHTA_I.base, // si
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_E.base, // më
    ]);
    let _silme_nuq = test_tengwar!(Quenya[nuquerna=true], "silmë" => [
        TENGWA_SILME_NUQ, TEHTA_I.base, // si
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_E.base, // më
    ]);
    // test_tengwar!(silme != silme_nuq);

    //  Check Essë.
    let esse = test_tengwar!(Quenya, "essë" => [
        CARRIER_SHORT, TEHTA_E.base, // e
        TENGWA_ESSE, TEHTA_E.base, // ssë
    ]);
    let esse_nuq = test_tengwar!(Quenya[nuquerna=true], "essë" => [
        CARRIER_SHORT, TEHTA_E.base, // e
        TENGWA_ESSE_NUQ, TEHTA_E.base, // ssë
    ]);
    // test_tengwar!(esse != esse_nuq);
    test_tengwar!(Quenya, "eze" == esse);
    test_tengwar!(Quenya[nuquerna=true], "eze" == esse_nuq);

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
    let eleni_silar = test_tengwar!(Quenya, "eleni sílar" => [
        CARRIER_SHORT, TEHTA_E.base, // e
        TENGWA_LAMBE, TEHTA_E.base, // le
        TENGWA_NUMEN, TEHTA_I.base, // ni
        ' ',
        TENGWA_SILME, // s
        CARRIER_LONG, TEHTA_I.base, // í
        TENGWA_LAMBE, TEHTA_A.base, // la
        TENGWA_ORE, // r
    ]);
    test_tengwar!(Quenya, "Eleni Sílar" == eleni_silar);
    test_tengwar!(Quenya, "Elënï Sílär" == eleni_silar);
    test_tengwar!(Quenya, "ELËNÏ SÍLÄR" == eleni_silar);
    test_tengwar!(Quenya, "ELeNi SiiLaR" == eleni_silar);
    test_tengwar!(Quenya, "ELENI SIILAR" == eleni_silar);

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

    let helcaraxe = test_tengwar!(Quenya, "helcaraxë" => [
        TENGWA_HYARMEN, TEHTA_E.base, // he
        TENGWA_LAMBE, // l
        TENGWA_CALMA, TEHTA_A.base, // ca
        TENGWA_ROMEN, TEHTA_A.base, // ra
        TENGWA_CALMA, TEHTA_E.base, SA_RINCE, // xë
    ]);
    test_tengwar!(Quenya, "helkarakse" == helcaraxe);

    let quenya = test_tengwar!(Quenya, "quenya" => [
        TENGWA_QESSE, TEHTA_E.base, // que
        TENGWA_NUMEN, MOD_PALATAL, TEHTA_A.base, // nya
    ]);
    test_tengwar!(Quenya, "qenya" == quenya);
    test_tengwar!(Quenya, "kwenya" == quenya);
    test_tengwar!(Quenya, "cwenya" == quenya);
    test_tengwar!(Quenya, "kuenya" != quenya);
    test_tengwar!(Quenya, "cuenya" != quenya);

    let _aha = test_tengwar!(Quenya, "aha" => [
        CARRIER_SHORT, TEHTA_A.base, // a
        TENGWA_AHA, TEHTA_A.base, // ha
    ]);

    let _hyarmen = test_tengwar!(Quenya, "hyarmen" => [
        TENGWA_HYARMEN, MOD_PALATAL, TEHTA_A.base, // hya
        TENGWA_ORE, // r
        TENGWA_MALTA, TEHTA_E.base, // me
        TENGWA_NUMEN, // n
    ]);

    let _hwesta = test_tengwar!(Quenya, "hwesta" => [
        TENGWA_HWESTA, TEHTA_E.base, // hwe
        TENGWA_SILME, // s
        TENGWA_TINCO, TEHTA_A.base, // ta
    ]);

    let ara = test_tengwar!(Quenya, "ára" => [
        CARRIER_LONG, TEHTA_A.base, // á
        TENGWA_ROMEN, TEHTA_A.base, // ra
    ]);
    test_tengwar!(Quenya, "aara" == ara); // ASCII spelling.

    //  Archaic TH (> S).
    let thuule = test_tengwar!(Quenya, "þúlë" => [
        TENGWA_THULE, TEHTA_U.base, TEHTA_U.base, // þú
        TENGWA_LAMBE, TEHTA_E.base, // lë
    ]);
    test_tengwar!(Quenya, "thuule" == thuule); // ASCII spelling.
    test_tengwar!(Quenya, "θúlë" == thuule);
    test_tengwar!(Quenya, "ΘÚLË" == thuule);
    test_tengwar!(Quenya, "ÞÚLË" == thuule);
    test_tengwar!(Quenya, "súlë" != thuule);

    let calma = test_tengwar!(Quenya, "calma" => [
        TENGWA_CALMA, TEHTA_A.base, // ca
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_A.base, // ma
    ]);
    test_tengwar!(Quenya, "kalma" == calma);

    //  Initial and final N.
    let nuumen = test_tengwar!(Quenya, "númen" => [
        TENGWA_NUMEN, TEHTA_U.base, TEHTA_U.base, // nú
        TENGWA_MALTA, TEHTA_E.base, // me
        TENGWA_NUMEN, // n
    ]);
    test_tengwar!(Quenya, "nuumen" == nuumen); // ASCII spelling.
    test_tengwar!(Quenya, "ngúmen" != nuumen);

    //  Initial NG (> N).
    let ngoldo = test_tengwar!(Quenya, "ñoldo" => [
        TENGWA_NOLDO, TEHTA_O.base, // ño
        TENGWA_ALDA, TEHTA_O.base, // ldo
    ]);
    test_tengwar!(Quenya, "ngoldo" == ngoldo); // ASCII spelling.
    test_tengwar!(Quenya, "ÑOLDO" == ngoldo);
    test_tengwar!(Quenya, "noldo" != ngoldo);

    //  Initial NGW (> NW).
    let ngwalme = test_tengwar!(Quenya, "ñwalmë" => [
        TENGWA_NWALME, TEHTA_A.base, // ñwa
        TENGWA_LAMBE, // l
        TENGWA_MALTA, TEHTA_E.base, // më
    ]);
    test_tengwar!(Quenya, "ngwalme" == ngwalme); // ASCII spelling.
    test_tengwar!(Quenya, "nwalmë" != ngwalme);

    //  Medial NG.
    let anga = test_tengwar!(Quenya, "anga" => [
        CARRIER_SHORT, TEHTA_A.base, // a
        TENGWA_ANGA, TEHTA_A.base, // nga
    ]);
    test_tengwar!(Quenya, "aña" != anga);
    test_tengwar!(Quenya, "ana" != anga);

    //  Medial NGW.
    let ungwe = test_tengwar!(Quenya, "ungwë" => [
        CARRIER_SHORT, TEHTA_U.base, // u
        TENGWA_UNGWE, TEHTA_E.base, // ngwë
    ]);
    test_tengwar!(Quenya, "ungwe" == ungwe); // ASCII spelling.
    test_tengwar!(Quenya, "uñwë" != ungwe);
    test_tengwar!(Quenya, "unwë" != ungwe);

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
    test_tengwar!(Quenya, "á" => [CARRIER_LONG, TEHTA_A.base] as aa);
    test_tengwar!(Quenya, "é" => [CARRIER_LONG, TEHTA_E.base] as ee);
    test_tengwar!(Quenya, "í" => [CARRIER_LONG, TEHTA_I.base] as ii);
    test_tengwar!(Quenya, "ó" => [CARRIER_LONG, TEHTA_O.base] as oo);
    test_tengwar!(Quenya, "ú" => [CARRIER_LONG, TEHTA_U.base] as uu);
    test_tengwar!(Quenya, "aa" == aa);
    test_tengwar!(Quenya, "ee" == ee);
    test_tengwar!(Quenya, "ii" == ii);
    test_tengwar!(Quenya, "oo" == oo);
    test_tengwar!(Quenya, "uu" == uu);

    //  Test all vowels, after consonants.
    test_tengwar!(Quenya, "la" => [TENGWA_LAMBE, TEHTA_A.base]);
    test_tengwar!(Quenya, "le" => [TENGWA_LAMBE, TEHTA_E.base]);
    test_tengwar!(Quenya, "li" => [TENGWA_LAMBE, TEHTA_I.base]);
    test_tengwar!(Quenya, "lo" => [TENGWA_LAMBE, TEHTA_O.base]);
    test_tengwar!(Quenya, "lu" => [TENGWA_LAMBE, TEHTA_U.base]);
    test_tengwar!(Quenya, "lá" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_A.base] as laa);
    test_tengwar!(Quenya, "lé" => [TENGWA_LAMBE, TEHTA_E.base, TEHTA_E.base] as lee);
    test_tengwar!(Quenya, "lí" => [TENGWA_LAMBE, CARRIER_LONG, TEHTA_I.base] as lii);
    test_tengwar!(Quenya, "ló" => [TENGWA_LAMBE, TEHTA_O.base, TEHTA_O.base] as loo);
    test_tengwar!(Quenya, "lú" => [TENGWA_LAMBE, TEHTA_U.base, TEHTA_U.base] as luu);
    test_tengwar!(Quenya, "laa" == laa);
    test_tengwar!(Quenya, "lee" == lee);
    test_tengwar!(Quenya, "lii" == lii);
    test_tengwar!(Quenya, "loo" == loo);
    test_tengwar!(Quenya, "luu" == luu);

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
