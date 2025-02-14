// @generated
#![cfg(feature = "icu_locid_transform")]
type DataStruct = < :: icu_locid_transform :: provider :: LikelySubtagsForLanguageV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    locale.is_empty().then(|| &UND)
}
static UND: DataStruct = include!("und.rs.data");
