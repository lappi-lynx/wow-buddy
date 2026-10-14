use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::views::about::REQUIRED_DISCLAIMER_LINE;

#[test]
fn test_disclaimer_is_verbatim_from_attribution() {
    let doc = include_str!("../assets/ATTRIBUTION.md");
    assert!(
        doc.contains(REQUIRED_DISCLAIMER_LINE),
        "About's disclaimer must be copied verbatim from ATTRIBUTION.md"
    );
    assert!(REQUIRED_DISCLAIMER_LINE
        .contains("in no way associated with or endorsed by Blizzard Entertainment"));
}

#[test]
fn test_about_i18n_keys_exist_in_all_locales() {
    let i18n = I18nService::load_embedded();
    for key in [
        "ui_about_title",
        "ui_about_offline",
        "ui_about_sources",
        "ui_about_version",
        "btn_about",
    ] {
        for lang in [Language::En, Language::Ru, Language::Fi] {
            assert_ne!(
                i18n.t(key, lang),
                key,
                "{key} missing for {lang:?}"
            );
        }
    }
}
