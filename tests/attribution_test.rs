use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::i18n::{I18nService, Language};

#[test]
fn test_required_disclaimer_is_shipped() {
    let doc = include_str!("../assets/ATTRIBUTION.md");
    assert!(doc.contains("World of Warcraft"));
    assert!(doc.contains("in no way associated with or endorsed by Blizzard Entertainment"));
}

#[test]
fn test_every_asset_has_a_source_and_license() {
    let reg = AssetRegistry::load_embedded();
    for (id, entry) in reg.iter() {
        assert!(!entry.source.is_empty(), "{id} missing source");
        assert!(entry.license.is_some(), "{id} missing license");
    }
}

#[test]
fn test_disclaimer_key_present_in_all_locales() {
    let i18n = I18nService::load_embedded();
    for lang in [Language::Ru, Language::En, Language::Fi] {
        assert_ne!(
            i18n.t("ui_attribution_disclaimer", lang),
            "ui_attribution_disclaimer"
        );
    }
}

#[test]
fn test_css_rebuild_did_not_drop_existing_classes() {
    // если пересборка Tailwind случилась, ранее существовавшие классы не должны исчезнуть
    let css = include_str!("../assets/app.css");
    assert!(css.contains("border-l-2"));
    assert!(css.contains("bg-slate-950"));
}
