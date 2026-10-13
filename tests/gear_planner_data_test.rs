use wow_buddy::models::gear_planner::GearPlannerData;
use wow_buddy::models::Faction;

#[test]
fn test_band_for_level_hits_containing_band() {
    let data = GearPlannerData::load_embedded();
    assert_eq!(data.band_for_level("warrior", Faction::Alliance, 3).unwrap().label, "1–6");
    assert_eq!(data.band_for_level("warrior", Faction::Alliance, 14).unwrap().label, "12–16");
    assert_eq!(data.band_for_level("warrior", Faction::Alliance, 20).unwrap().label, "20–24");
}

#[test]
fn test_level_in_gap_has_no_band_but_a_next_one() {
    let data = GearPlannerData::load_embedded();
    // 7 попадает в разрыв между 1–6 и 12–16
    assert!(data.band_for_level("warrior", Faction::Alliance, 7).is_none());
    assert_eq!(data.next_band_for_level("warrior", Faction::Alliance, 7).unwrap().label, "12–16");
}

#[test]
fn test_unknown_class_is_empty_not_panic() {
    let data = GearPlannerData::load_embedded();
    assert!(data.bands_for("nonexistent", Faction::Alliance).is_empty());
    assert!(data.band_for_level("nonexistent", Faction::Alliance, 10).is_none());
}

#[test]
fn test_warrior_weapon_path_from_kb() {
    let data = GearPlannerData::load_embedded();
    let band = data.band_for_level("warrior", Faction::Alliance, 20).unwrap();
    let weapon = band.weapons.iter().find(|w| w.level == 20).unwrap();
    assert_eq!(weapon.name, "Smite's Mighty Hammer");
    assert_eq!(weapon.item_id, Some(7230));
}

#[test]
fn test_horde_path_differs_from_alliance() {
    let data = GearPlannerData::load_embedded();
    let alliance = data.bands_for("warrior", Faction::Alliance);
    let horde = data.bands_for("warrior", Faction::Horde);
    assert!(!horde.is_empty());
    // фракционные пути не совпадают по источникам
    assert_ne!(
        alliance.iter().map(|b| b.armor.source.clone()).collect::<Vec<_>>(),
        horde.iter().map(|b| b.armor.source.clone()).collect::<Vec<_>>()
    );
}

#[test]
fn test_missing_provenance_is_reported_not_ignored() {
    let data = GearPlannerData::load_embedded();
    // стартовый срез намеренно оставляет запись "starter club" без provenance —
    // она должна перечисляться, а не молча проходить
    let missing = data.bands_missing_provenance();
    assert!(!missing.is_empty(), "expected at least one record flagged as missing provenance");
    assert!(missing.iter().any(|m| m.contains("starter club")));
}
