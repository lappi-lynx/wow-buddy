use wow_buddy::i18n::{I18nService, Language, LanguageStorage};

#[test]
fn test_translation_resolution() {
    let i18n = I18nService::load_embedded();
    assert_eq!(i18n.t("alliance_name", Language::Ru), "Альянс");
    assert_eq!(i18n.t("alliance_name", Language::En), "Alliance");
    assert_eq!(i18n.t("alliance_name", Language::Fi), "Liittouma");
}

#[test]
fn test_default_language_is_english() {
    assert_eq!(Language::default(), Language::En);
}

#[test]
fn test_flexible_language_catalog_for_dropdown() {
    let available = Language::all();
    assert!(available.contains(&Language::En));
    assert!(available.contains(&Language::Ru));
    assert!(available.contains(&Language::Fi));

    assert_eq!(Language::from_code("en"), Some(Language::En));
    assert_eq!(Language::from_code("ru"), Some(Language::Ru));
    assert_eq!(Language::from_code("fi"), Some(Language::Fi));
    assert_eq!(Language::from_code("unknown"), None);

    assert_eq!(Language::En.code(), "en");
    assert_eq!(Language::En.label(), "English");
    assert_eq!(Language::Ru.label(), "Русский");
    assert_eq!(Language::Fi.label(), "Suomi");
}

#[test]
fn test_fallback_to_english_for_missing_translation() {
    let i18n = I18nService::load_embedded();
    // Non-existent key falls back to key itself
    assert_eq!(i18n.t("non_existent_key_xyz", Language::En), "non_existent_key_xyz");
}

#[test]
fn test_language_storage_persistence() {
    let mut storage = LanguageStorage::in_memory();
    assert_eq!(storage.get_language(), Language::En); // default

    storage.set_language(Language::Fi);
    assert_eq!(storage.get_language(), Language::Fi);

    storage.set_language(Language::Ru);
    assert_eq!(storage.get_language(), Language::Ru);
}
