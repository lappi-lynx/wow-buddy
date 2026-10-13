use wow_buddy::components::class_tips_card::spec_cards;
use wow_buddy::models::class_tips::ClassTipsData;

#[test]
fn test_spec_cards_returns_all_specs_or_empty() {
    let data = ClassTipsData::load_embedded();
    assert_eq!(spec_cards(&data, "druid").len(), 3);
    assert!(spec_cards(&data, "not_a_class").is_empty());
}

#[test]
fn test_tips_keys_exist_in_each_locale_file() {
    // Raw dictionaries, not I18nService::t — `t` falls back to English,
    // which would hide a missing ru/fi key.
    for (name, raw) in [
        ("ru", include_str!("../assets/i18n/ru.json")),
        ("en", include_str!("../assets/i18n/en.json")),
        ("fi", include_str!("../assets/i18n/fi.json")),
    ] {
        let dict: serde_json::Value = serde_json::from_str(raw).unwrap();
        for key in [
            "ui_tips_title",
            "ui_tips_empty",
            "ui_specs_info_title",
            "ui_specs_info_label",
        ] {
            let value = dict.get(key).and_then(|v| v.as_str()).unwrap_or("");
            assert!(!value.is_empty(), "missing {key} in {name}.json");
        }
    }
}

#[test]
fn test_css_rebuild_did_not_drop_existing_classes() {
    let css = include_str!("../assets/app.css");
    assert!(css.contains("border-l-2"));
    assert!(css.contains("bg-slate-950"));
}
