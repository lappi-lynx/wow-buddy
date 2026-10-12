use wow_buddy::models::{Faction, GameData};
use wow_buddy::views::race_selector::format_race_display_name;

#[test]
fn test_race_filtering_by_faction() {
    let data = GameData::load_embedded();
    let alliance_races = data.get_races_for_faction(Faction::Alliance);
    assert!(alliance_races.iter().any(|r| r.id == "human"));
    assert!(alliance_races.iter().any(|r| r.id == "skyborne"));
    assert!(!alliance_races.iter().any(|r| r.id == "orc"));
}

#[test]
fn test_skyborne_variant_naming() {
    let data = GameData::load_embedded();
    let skyborne = data.get_race_by_id("skyborne").expect("Skyborne present");
    assert_eq!(
        format_race_display_name(skyborne, Faction::Alliance),
        "High Order Skyborne"
    );
    assert_eq!(
        format_race_display_name(skyborne, Faction::Horde),
        "Windshaper Skyborne"
    );
}
