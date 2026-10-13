use wow_buddy::components::loot_quiz::{evaluate_loot_choice, LootChoice, LOOT_SCENARIO_ITEM};

#[test]
fn test_greed_correct_need_ninja() {
    assert!(evaluate_loot_choice(LootChoice::Greed));
    assert!(!evaluate_loot_choice(LootChoice::Need));
}

#[test]
fn test_scenario_item_pinned() {
    assert_eq!(LOOT_SCENARIO_ITEM, "Smite's Mighty Hammer");
}

#[test]
fn test_loot_quiz_keys_exist_in_all_locales() {
    let ru: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/ru.json")).unwrap();
    let en: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let fi: serde_json::Value = serde_json::from_str(include_str!("../assets/i18n/fi.json")).unwrap();
    for dict in [&ru, &en, &fi] {
        assert!(dict.get("ui_loot_scenario").is_some());
        assert!(dict.get("ui_loot_reset").is_some());
    }
}
