use wow_buddy::models::{Faction, GameData};
use wow_buddy::views::race_selector::{effective_race_id, format_race_display_name};

#[test]
fn test_race_filtering_by_faction() {
    let data = GameData::load_embedded();
    let alliance_races = data.get_races_for_faction(Faction::Alliance);
    assert!(alliance_races.iter().any(|r| r.id == "human"));
    assert!(alliance_races.iter().any(|r| r.id == "skyborne"));
    assert!(!alliance_races.iter().any(|r| r.id == "orc"));
}

#[test]
fn test_effective_race_id_ignores_stale_cross_faction_selection() {
    let data = GameData::load_embedded();
    let alliance: Vec<_> = data
        .get_races_for_faction(Faction::Alliance)
        .into_iter()
        .cloned()
        .collect();
    let horde: Vec<_> = data
        .get_races_for_faction(Faction::Horde)
        .into_iter()
        .cloned()
        .collect();

    // валидный текущий выбор сохраняется
    assert_eq!(
        effective_race_id(&alliance, Some("dwarf")).as_deref(),
        Some("dwarf")
    );
    // застрявший id из списка другой фракции игнорируется -> первый из списка
    assert_eq!(
        effective_race_id(&alliance, Some("orc")).as_deref(),
        Some("human")
    );
    assert_eq!(
        effective_race_id(&horde, Some("human")).as_deref(),
        Some("skyborne")
    );
    // пустой выбор -> первый из списка
    assert_eq!(effective_race_id(&alliance, None).as_deref(), Some("human"));
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
