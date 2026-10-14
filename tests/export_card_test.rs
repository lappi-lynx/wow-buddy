use wow_buddy::components::export_card::build_export_string;
use wow_buddy::models::Faction;

#[test]
fn test_export_string_alliance_skyborne() {
    let s = build_export_string(Faction::Alliance, "High Order Skyborne", "Mage");
    assert_eq!(s, "FACTION: Alliance | RACE: High Order Skyborne | CLASS: Mage");
}

#[test]
fn test_export_string_horde_orc() {
    let s = build_export_string(Faction::Horde, "Orc", "Warrior");
    assert_eq!(s, "FACTION: Horde | RACE: Orc | CLASS: Warrior");
}

#[test]
fn test_faction_display_names_are_explicit_labels() {
    // The export contract must not depend on `{:?}` Debug output; these are
    // the explicit, stable labels used by `build_export_string`.
    assert_eq!(Faction::Alliance.display_name(), "Alliance");
    assert_eq!(Faction::Horde.display_name(), "Horde");
}

#[test]
fn test_export_keys_exist_in_all_locales() {
    let ru: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/ru.json")).unwrap();
    let en: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let fi: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/fi.json")).unwrap();
    for dict in [&ru, &en, &fi] {
        for key in ["ui_export_title", "ui_export_plan", "ui_export_copied", "ui_export_hint"] {
            assert!(dict.get(key).is_some(), "missing key {key}");
        }
    }
    assert_eq!(
        en.get("ui_export_plan").and_then(|v| v.as_str()),
        Some("Copy my character plan")
    );
}
