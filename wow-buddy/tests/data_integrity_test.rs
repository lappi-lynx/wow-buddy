use wow_buddy::models::{Faction, GameData};

#[test]
fn test_starter_zones_coverage() {
    let data = GameData::load_embedded();
    assert_eq!(data.starter_zones.len(), 7); // 6 classic + Zephras Isle
    assert!(data.races.iter().any(|r| r.id == "skyborne" && r.is_dual_faction));
}

#[test]
fn test_faction_filtering_and_skyborne_presence() {
    let data = GameData::load_embedded();
    let alliance_races = data.get_races_for_faction(Faction::Alliance);
    let horde_races = data.get_races_for_faction(Faction::Horde);

    assert_eq!(alliance_races.len(), 5);
    assert_eq!(horde_races.len(), 5);

    assert!(alliance_races.iter().any(|r| r.id == "human"));
    assert!(alliance_races.iter().any(|r| r.id == "skyborne"));
    assert!(!alliance_races.iter().any(|r| r.id == "orc"));

    assert!(horde_races.iter().any(|r| r.id == "orc"));
    assert!(horde_races.iter().any(|r| r.id == "skyborne"));
    assert!(!horde_races.iter().any(|r| r.id == "human"));
}

#[test]
fn test_starter_dungeons_presence() {
    let data = GameData::load_embedded();
    assert!(data.dungeons.iter().any(|d| d.id == "deadmines"));
    assert!(data.dungeons.iter().any(|d| d.id == "ragefire_chasm"));
}
