use crate::mode::gondor::*;
use super::*;


#[test]
fn test_gondor() {
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
