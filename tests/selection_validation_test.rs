use wow_buddy::models::{GameData, Faction};

#[test]
fn test_class_available_for_race() {
    let data = GameData::load_embedded();
    assert!(data.is_class_available_for_race(Faction::Alliance, "human", "warrior"));
    assert!(data.is_class_available_for_race(Faction::Alliance, "human", "hunter"));
    assert!(data.is_class_available_for_race(Faction::Alliance, "skyborne", "mage"));
    assert!(!data.is_class_available_for_race(Faction::Horde, "skyborne", "mage"));
    assert!(data.is_class_available_for_race(Faction::Horde, "skyborne", "shaman"));
    assert!(!data.is_class_available_for_race(Faction::Alliance, "skyborne", "shaman"));
    assert!(!data.is_class_available_for_race(Faction::Alliance, "human", "shaman"));
    assert!(!data.is_class_available_for_race(Faction::Alliance, "skyborne", "priest"));
}

#[test]
fn test_validate_selection() {
    let data = GameData::load_embedded();
    assert!(data.validate_selection(Faction::Alliance, "human", "warrior"));
    assert!(data.validate_selection(Faction::Alliance, "human", "hunter"));
    assert!(data.validate_selection(Faction::Horde, "undead", "paladin"));
    assert!(!data.validate_selection(Faction::Alliance, "orc", "warrior"));
    assert!(!data.validate_selection(Faction::Alliance, "undead", "paladin"));
    assert!(data.validate_selection(Faction::Alliance, "skyborne", "mage"));
    assert!(!data.validate_selection(Faction::Horde, "skyborne", "mage"));
}

#[test]
fn test_defaults_per_faction_and_race() {
    let data = GameData::load_embedded();
    assert_eq!(data.default_race_for_faction(Faction::Alliance).id, "human");
    assert_eq!(data.default_race_for_faction(Faction::Horde).id, "orc");
    assert_eq!(data.default_class_for_race(Faction::Alliance, "human").unwrap().id, "warrior");
    assert_eq!(data.default_class_for_race(Faction::Alliance, "skyborne").unwrap().id, "warrior");
    assert!(data.default_class_for_race(Faction::Alliance, "nonexistent").is_none());
}
