use wow_buddy::components::class_tips_card::spec_cards;
use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::models::class_tips::ClassTipsData;

#[test]
fn test_spec_cards_returns_all_specs_or_empty() {
    let data = ClassTipsData::load_embedded();
    assert_eq!(spec_cards(&data, "druid").len(), 3);
    assert!(spec_cards(&data, "warrior").is_empty());
}

#[test]
fn test_tips_keys_exist_in_all_locales() {
    let i18n = I18nService::load_embedded();
    for lang in [Language::Ru, Language::En, Language::Fi] {
        for key in [
            "ui_tips_title", "ui_tips_empty", "ui_specs_info_title", "ui_specs_info_label",
        ] {
            assert_ne!(i18n.t(key, lang), key, "missing {key} for {lang:?}");
        }
    }
}

#[test]
fn test_css_rebuild_did_not_drop_existing_classes() {
    let css = include_str!("../assets/app.css");
    assert!(css.contains("border-l-2"));
    assert!(css.contains("bg-slate-950"));
}
