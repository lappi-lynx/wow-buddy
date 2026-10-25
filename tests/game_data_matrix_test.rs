use wow_buddy::models::{GameData, Faction};

fn assert_class_set(actual: &[String], expected: &[&str]) {
    let mut a: Vec<&str> = actual.iter().map(|s| s.as_str()).collect();
    a.sort_unstable();
    let mut e: Vec<&str> = expected.to_vec();
    e.sort_unstable();
    assert_eq!(a, e);
}

#[test]
fn test_alliance_matrix_matches_reviewed_beta() {
    let data = GameData::load_embedded();
    assert_class_set(data.classes_for_faction(Faction::Alliance, "human"),
        &["warrior", "paladin", "hunter", "rogue", "priest", "mage", "warlock"]);
    assert_class_set(data.classes_for_faction(Faction::Alliance, "dwarf"),
        &["warrior", "paladin", "hunter", "rogue", "priest", "shaman"]);
    assert_class_set(data.classes_for_faction(Faction::Alliance, "night_elf"),
        &["warrior", "hunter", "rogue", "priest", "druid"]);
    assert_class_set(data.classes_for_faction(Faction::Alliance, "gnome"),
        &["warrior", "rogue", "priest", "mage", "warlock"]);
    assert_class_set(data.classes_for_faction(Faction::Alliance, "skyborne"),
        &["warrior", "hunter", "rogue", "mage", "druid"]);
}

#[test]
fn test_horde_matrix_matches_reviewed_beta() {
    let data = GameData::load_embedded();
    assert_class_set(data.classes_for_faction(Faction::Horde, "orc"),
        &["warrior", "hunter", "rogue", "shaman", "mage", "warlock"]);
    assert_class_set(data.classes_for_faction(Faction::Horde, "undead"),
        &["warrior", "rogue", "priest", "mage", "warlock", "paladin"]);
    assert_class_set(data.classes_for_faction(Faction::Horde, "tauren"),
        &["warrior", "hunter", "shaman", "druid"]);
    assert_class_set(data.classes_for_faction(Faction::Horde, "troll"),
        &["warrior", "hunter", "rogue", "priest", "shaman", "mage", "warlock"]);
    assert_class_set(data.classes_for_faction(Faction::Horde, "skyborne"),
        &["warrior", "hunter", "rogue", "shaman", "druid"]);
}

#[test]
fn test_skyborne_faction_exclusive_classes() {
    let data = GameData::load_embedded();
    // Mage — только High Order (Alliance)
    assert!(data.classes_for_faction(Faction::Alliance, "skyborne").contains(&"mage".to_string()));
    assert!(!data.classes_for_faction(Faction::Horde, "skyborne").contains(&"mage".to_string()));
    // Shaman — только Windshaper (Horde)
    assert!(data.classes_for_faction(Faction::Horde, "skyborne").contains(&"shaman".to_string()));
    assert!(!data.classes_for_faction(Faction::Alliance, "skyborne").contains(&"shaman".to_string()));
    // Priest — недоступен ни в одной фракции Skyborne
    assert!(!data.classes_for_faction(Faction::Alliance, "skyborne").contains(&"priest".to_string()));
    assert!(!data.classes_for_faction(Faction::Horde, "skyborne").contains(&"priest".to_string()));
}

#[test]
fn test_verified_racial_abilities() {
    // Source: docs/wowhead-sources/racial-updates-2026-10-16.md
    let data = GameData::load_embedded();
    let skyborne = data.get_race_by_id("skyborne").unwrap();
    assert_eq!(skyborne.racial_ability_name, "Walk on Air");
    assert!(skyborne.racial_ability_desc.contains("90-second cooldown"));
    let orc = data.get_race_by_id("orc").unwrap();
    assert_eq!(orc.racial_ability_name, "Savage");
    assert!(orc.racial_ability_desc.contains("1% increased critical strike chance"));
    let gnome = data.get_race_by_id("gnome").unwrap();
    assert_eq!(gnome.racial_ability_name, "Expansive Mind");
    assert!(gnome.racial_ability_desc.contains("+5% to primary class resource"));
}
