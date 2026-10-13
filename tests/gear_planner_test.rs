use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::models::gear_planner::GearPlannerData;
use wow_buddy::models::Faction;
use wow_buddy::views::gear_planner::band_highlight;

#[test]
fn test_band_highlight_uses_containing_then_next() {
    let data = GearPlannerData::load_embedded();
    let bands = data.bands_for("warrior", Faction::Alliance);
    // 14 -> диапазон 12–16
    assert_eq!(band_highlight(bands, 14).as_deref(), Some(bands.iter().find(|b| b.label == "12–16").unwrap().id.as_str()));
    // 7 (разрыв) -> подсвечиваем следующий 12–16
    assert_eq!(band_highlight(bands, 7).as_deref(), Some(bands.iter().find(|b| b.label == "12–16").unwrap().id.as_str()));
}

#[test]
fn test_gear_planner_keys_exist_in_all_locales() {
    let i18n = I18nService::load_embedded();
    for lang in [Language::Ru, Language::En, Language::Fi] {
        for key in [
            "ui_gear_planner_title", "ui_gear_planner_subtitle", "ui_open_gear_planner",
            "ui_gear_current", "ui_gear_next_upgrade", "ui_gear_no_data",
        ] {
            assert_ne!(i18n.t(key, lang), key, "missing {key} for {lang:?}");
        }
    }
}

#[test]
fn test_empty_bands_show_no_highlight() {
    // класс+фракция без данных -> "no data", без подсветки и без паники
    assert_eq!(band_highlight(&[], 10), None);
}

#[test]
fn test_new_screen_class_is_present_in_precompiled_css() {
    // Tailwind прекомпилирован: класс, который вводит экран, обязан быть в собранном app.css
    let css = include_str!("../assets/app.css");
    assert!(css.contains("border-l-2"), "app.css not rebuilt — run the tailwind CLI");
}
