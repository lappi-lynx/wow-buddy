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
fn test_export_keys_exist_in_all_locales() {
    let ru: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/ru.json")).unwrap();
    let en: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let fi: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/fi.json")).unwrap();
    for dict in [&ru, &en, &fi] {
        for key in ["ui_export_title", "ui_export_btn", "ui_export_copied", "ui_export_hint"] {
            assert!(dict.get(key).is_some(), "missing key {key}");
        }
    }
}
