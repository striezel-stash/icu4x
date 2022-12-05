// @generated
#![cfg(feature = "icu_relativetime")]
type DataStruct = < :: icu_relativetime :: provider :: LongWeekRelativeTimeFormatDataV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: litemap::LiteMap<&str, &DataStruct, &[(&str, &DataStruct)]> =
    litemap::LiteMap::from_sorted_store_unchecked(&[
        ("ar", AR_AR_EG),
        ("ar-EG", AR_AR_EG),
        ("bn", BN),
        ("ccp", CCP),
        ("en", EN_EN_001_EN_ZA),
        ("en-001", EN_EN_001_EN_ZA),
        ("en-ZA", EN_EN_001_EN_ZA),
        ("es", ES_ES_AR),
        ("es-AR", ES_ES_AR),
        ("fil", FIL),
        ("fr", FR),
        ("ja", JA),
        ("ru", RU),
        ("sr", SR_SR_CYRL),
        ("sr-Cyrl", SR_SR_CYRL),
        ("sr-Latn", SR_LATN),
        ("th", TH),
        ("tr", TR),
        ("und", UND),
    ]);
static AR_AR_EG: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 27u8, 0u8, 48u8, 0u8, 216u8, 167u8, 217u8, 132u8,
                    216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8, 136u8, 216u8, 185u8, 32u8,
                    216u8, 167u8, 217u8, 132u8, 217u8, 133u8, 216u8, 167u8, 216u8, 182u8, 217u8,
                    138u8, 217u8, 135u8, 216u8, 176u8, 216u8, 167u8, 32u8, 216u8, 167u8, 217u8,
                    132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8, 136u8, 216u8, 185u8,
                    216u8, 167u8, 217u8, 132u8, 216u8, 163u8, 216u8, 179u8, 216u8, 168u8, 217u8,
                    136u8, 216u8, 185u8, 32u8, 216u8, 167u8, 217u8, 132u8, 217u8, 130u8, 216u8,
                    167u8, 216u8, 175u8, 217u8, 133u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"),
            index: 7u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل أسبوع واحد"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل أسبوعين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  أسابيع"),
            index: 7u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع\u{64b}ا"),
            index: 7u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("قبل  أسبوع"),
            index: 7u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"),
            index: 9u8,
        }),
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال أسبوع واحد"),
            index: 255u8,
        }),
        two: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال أسبوعين"),
            index: 255u8,
        }),
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  أسابيع"),
            index: 9u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع\u{64b}ا"),
            index: 9u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("خلال  أسبوع"),
            index: 9u8,
        },
    },
};
static BN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 50u8, 0u8, 224u8, 166u8, 151u8, 224u8,
                    166u8, 164u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8, 167u8,
                    141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8, 166u8, 185u8, 224u8,
                    166u8, 143u8, 224u8, 166u8, 135u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8,
                    170u8, 224u8, 167u8, 141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8,
                    166u8, 185u8, 224u8, 166u8, 170u8, 224u8, 166u8, 176u8, 224u8, 167u8, 135u8,
                    224u8, 166u8, 176u8, 32u8, 224u8, 166u8, 184u8, 224u8, 166u8, 170u8, 224u8,
                    167u8, 141u8, 224u8, 166u8, 164u8, 224u8, 166u8, 190u8, 224u8, 166u8, 185u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হ আগে"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" সপ\u{9cd}ত\u{9be}হে"),
            index: 0u8,
        },
    },
};
static CCP: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 41u8, 0u8, 74u8, 0u8, 240u8, 145u8, 132u8, 137u8,
                    240u8, 145u8, 132u8, 172u8, 240u8, 145u8, 132u8, 163u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 152u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 131u8, 240u8, 145u8, 132u8,
                    179u8, 240u8, 145u8, 132u8, 134u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8,
                    167u8, 240u8, 145u8, 132u8, 162u8, 240u8, 145u8, 132u8, 172u8, 32u8, 240u8,
                    145u8, 132u8, 165u8, 240u8, 145u8, 132u8, 155u8, 240u8, 145u8, 132u8, 180u8,
                    240u8, 145u8, 132u8, 150u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄥𑄛\u{11134}𑄖 𑄃𑄉𑄬"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄥𑄛\u{11134}𑄖 𑄃𑄉𑄬"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄥𑄛\u{11134}𑄖𑄠\u{11134}"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 𑄥𑄛\u{11134}𑄖𑄠\u{11134}"),
            index: 0u8,
        },
    },
};
static EN_EN_001_EN_ZA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 119u8, 101u8, 101u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8, 119u8,
                    101u8, 101u8, 107u8, 110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 101u8, 101u8,
                    107u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" week ago"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" weeks ago"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  week"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("in  weeks"),
            index: 3u8,
        },
    },
};
static ES_ES_AR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 27u8, 0u8, 108u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 110u8, 97u8, 32u8, 112u8, 97u8, 115u8, 97u8, 100u8, 97u8,
                    101u8, 115u8, 116u8, 97u8, 32u8, 115u8, 101u8, 109u8, 97u8, 110u8, 97u8, 108u8,
                    97u8, 32u8, 112u8, 114u8, 195u8, 179u8, 120u8, 105u8, 109u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 110u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  semana"),
            index: 5u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("hace  semanas"),
            index: 5u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  semana"),
            index: 10u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dentro de  semanas"),
            index: 10u8,
        },
    },
};
static FIL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 19u8, 0u8, 34u8, 0u8, 110u8, 97u8, 107u8, 97u8,
                    108u8, 105u8, 112u8, 97u8, 115u8, 32u8, 110u8, 97u8, 32u8, 108u8, 105u8, 110u8,
                    103u8, 103u8, 111u8, 115u8, 97u8, 32u8, 108u8, 105u8, 110u8, 103u8, 103u8,
                    111u8, 110u8, 103u8, 32u8, 105u8, 116u8, 111u8, 115u8, 117u8, 115u8, 117u8,
                    110u8, 111u8, 100u8, 32u8, 110u8, 97u8, 32u8, 108u8, 105u8, 110u8, 103u8,
                    103u8, 111u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" linggo ang nakalipas"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" (na) linggo ang nakalipas"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  linggo"),
            index: 3u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("sa  (na) linggo"),
            index: 3u8,
        },
    },
};
static FR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 20u8, 0u8, 33u8, 0u8, 108u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 100u8, 101u8, 114u8, 110u8,
                    105u8, 195u8, 168u8, 114u8, 101u8, 99u8, 101u8, 116u8, 116u8, 101u8, 32u8,
                    115u8, 101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 108u8, 97u8, 32u8, 115u8,
                    101u8, 109u8, 97u8, 105u8, 110u8, 101u8, 32u8, 112u8, 114u8, 111u8, 99u8,
                    104u8, 97u8, 105u8, 110u8, 101u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a  semaine"),
            index: 7u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("il y a  semaines"),
            index: 7u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dans  semaine"),
            index: 5u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("dans  semaines"),
            index: 5u8,
        },
    },
};
static JA: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 12u8, 0u8, 229u8, 133u8, 136u8, 233u8,
                    128u8, 177u8, 228u8, 187u8, 138u8, 233u8, 128u8, 177u8, 230u8, 157u8, 165u8,
                    233u8, 128u8, 177u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 週間前"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" 週間後"),
            index: 0u8,
        },
    },
};
static RU: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 58u8, 0u8, 208u8, 189u8, 208u8, 176u8,
                    32u8, 208u8, 191u8, 209u8, 128u8, 208u8, 190u8, 209u8, 136u8, 208u8, 187u8,
                    208u8, 190u8, 208u8, 185u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8,
                    208u8, 181u8, 208u8, 187u8, 208u8, 181u8, 208u8, 189u8, 208u8, 176u8, 32u8,
                    209u8, 141u8, 209u8, 130u8, 208u8, 190u8, 208u8, 185u8, 32u8, 208u8, 189u8,
                    208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 208u8, 187u8, 208u8, 181u8, 208u8,
                    189u8, 208u8, 176u8, 32u8, 209u8, 129u8, 208u8, 187u8, 208u8, 181u8, 208u8,
                    180u8, 209u8, 131u8, 209u8, 142u8, 209u8, 137u8, 208u8, 181u8, 208u8, 185u8,
                    32u8, 208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 208u8, 187u8,
                    208u8, 181u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" неделю назад"),
            index: 0u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" недели назад"),
            index: 0u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" недель назад"),
            index: 0u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" недели назад"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("через  неделю"),
            index: 11u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("через  недели"),
            index: 11u8,
        }),
        many: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("через  недель"),
            index: 11u8,
        }),
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("через  недели"),
            index: 11u8,
        },
    },
};
static SR_LATN: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 15u8, 0u8, 26u8, 0u8, 112u8, 114u8, 111u8, 197u8,
                    161u8, 108u8, 101u8, 32u8, 110u8, 101u8, 100u8, 101u8, 108u8, 106u8, 101u8,
                    111u8, 118u8, 101u8, 32u8, 110u8, 101u8, 100u8, 101u8, 108u8, 106u8, 101u8,
                    115u8, 108u8, 101u8, 100u8, 101u8, 196u8, 135u8, 101u8, 32u8, 110u8, 101u8,
                    100u8, 101u8, 108u8, 106u8, 101u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  nedelje"),
            index: 4u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  nedelje"),
            index: 4u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("pre  nedelja"),
            index: 4u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  nedelju"),
            index: 3u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  nedelje"),
            index: 3u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("za  nedelja"),
            index: 3u8,
        },
    },
};
static SR_SR_CYRL: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 25u8, 0u8, 44u8, 0u8, 208u8, 191u8, 209u8, 128u8,
                    208u8, 190u8, 209u8, 136u8, 208u8, 187u8, 208u8, 181u8, 32u8, 208u8, 189u8,
                    208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 153u8, 208u8, 181u8, 208u8,
                    190u8, 208u8, 178u8, 208u8, 181u8, 32u8, 208u8, 189u8, 208u8, 181u8, 208u8,
                    180u8, 208u8, 181u8, 209u8, 153u8, 208u8, 181u8, 209u8, 129u8, 208u8, 187u8,
                    208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 155u8, 208u8, 181u8, 32u8,
                    208u8, 189u8, 208u8, 181u8, 208u8, 180u8, 208u8, 181u8, 209u8, 153u8, 208u8,
                    181u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  недеље"),
            index: 7u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  недеље"),
            index: 7u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("пре  недеља"),
            index: 7u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  недељу"),
            index: 5u8,
        }),
        two: None,
        few: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  недеље"),
            index: 5u8,
        }),
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("за  недеља"),
            index: 5u8,
        },
    },
};
static TH: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 42u8, 0u8, 72u8, 0u8, 224u8, 184u8, 170u8, 224u8,
                    184u8, 177u8, 224u8, 184u8, 155u8, 224u8, 184u8, 148u8, 224u8, 184u8, 178u8,
                    224u8, 184u8, 171u8, 224u8, 185u8, 140u8, 224u8, 184u8, 151u8, 224u8, 184u8,
                    181u8, 224u8, 185u8, 136u8, 224u8, 185u8, 129u8, 224u8, 184u8, 165u8, 224u8,
                    185u8, 137u8, 224u8, 184u8, 167u8, 224u8, 184u8, 170u8, 224u8, 184u8, 177u8,
                    224u8, 184u8, 155u8, 224u8, 184u8, 148u8, 224u8, 184u8, 178u8, 224u8, 184u8,
                    171u8, 224u8, 185u8, 140u8, 224u8, 184u8, 153u8, 224u8, 184u8, 181u8, 224u8,
                    185u8, 137u8, 224u8, 184u8, 170u8, 224u8, 184u8, 177u8, 224u8, 184u8, 155u8,
                    224u8, 184u8, 148u8, 224u8, 184u8, 178u8, 224u8, 184u8, 171u8, 224u8, 185u8,
                    140u8, 224u8, 184u8, 171u8, 224u8, 184u8, 153u8, 224u8, 185u8, 137u8, 224u8,
                    184u8, 178u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(
                " ส\u{e31}ปดาห\u{e4c}ท\u{e35}\u{e48}ผ\u{e48}านมา",
            ),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("ในอ\u{e35}ก  ส\u{e31}ปดาห\u{e4c}"),
            index: 16u8,
        },
    },
};
static TR: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 20u8, 0u8, 103u8, 101u8, 195u8, 167u8,
                    101u8, 110u8, 32u8, 104u8, 97u8, 102u8, 116u8, 97u8, 98u8, 117u8, 32u8, 104u8,
                    97u8, 102u8, 116u8, 97u8, 103u8, 101u8, 108u8, 101u8, 99u8, 101u8, 107u8, 32u8,
                    104u8, 97u8, 102u8, 116u8, 97u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hafta önce"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hafta önce"),
            index: 0u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: Some(::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hafta sonra"),
            index: 0u8,
        }),
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed(" hafta sonra"),
            index: 0u8,
        },
    },
};
static UND: &DataStruct = &::icu_relativetime::provider::RelativeTimePatternDataV1 {
    relatives: unsafe {
        #[allow(unused_unsafe)]
        ::zerovec::ZeroMap::from_parts_unchecked(
            unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[255u8, 0u8, 1u8]) },
            unsafe {
                ::zerovec::VarZeroVec::from_bytes_unchecked(&[
                    3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 9u8, 0u8, 18u8, 0u8, 108u8, 97u8, 115u8, 116u8,
                    32u8, 119u8, 101u8, 101u8, 107u8, 116u8, 104u8, 105u8, 115u8, 32u8, 119u8,
                    101u8, 101u8, 107u8, 110u8, 101u8, 120u8, 116u8, 32u8, 119u8, 101u8, 101u8,
                    107u8,
                ])
            },
        )
    },
    past: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("- w"),
            index: 1u8,
        },
    },
    future: ::icu_relativetime::provider::PluralRulesCategoryMapping {
        zero: None,
        one: None,
        two: None,
        few: None,
        many: None,
        other: ::icu_relativetime::provider::SingularSubPattern {
            pattern: alloc::borrow::Cow::Borrowed("+ w"),
            index: 1u8,
        },
    },
};
