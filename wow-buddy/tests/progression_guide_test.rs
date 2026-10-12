use wow_buddy::models::legacy::get_starter_legacy_preset;
use wow_buddy::models::GameData;
use wow_buddy::views::progression_guide::get_starter_zone_checklist;

#[test]
fn test_legacy_points_total_is_16() {
    let preset = get_starter_legacy_preset();
    let total_points: u8 = preset.perks.iter().map(|p| p.points).sum();
    assert_eq!(total_points, 16);
}

#[test]
fn test_starter_zone_checklist_items() {
    let data = GameData::load_embedded();
    let zone = data.get_starter_zone("northshire").expect("Northshire exists");
    let checklist = get_starter_zone_checklist(zone);
    assert_eq!(checklist.len(), 3);
    assert!(checklist[0].contains("Marshal McBride"));
    assert!(checklist[1].contains("2 and 4"));
    assert!(checklist[2].contains("5 silver"));
}
