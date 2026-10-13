use wow_buddy::models::{GameData, Faction};
use wow_buddy::selection::SelectionState;

#[test]
fn test_incompatible_race_resets_on_faction_switch() {
    let data = GameData::load_embedded();
    let state = SelectionState { faction: Faction::Horde, race_id: "orc".into(), class_id: "warrior".into() };
    let next = state.after_faction_change(&data, Faction::Alliance);
    assert_eq!(next.faction, Faction::Alliance);
    assert_eq!(next.race_id, "human");
    assert_eq!(next.class_id, "warrior");
}

#[test]
fn test_dual_faction_skyborne_keeps_race_but_coerces_class() {
    let data = GameData::load_embedded();
    // Windshaper Shaman не может стать High Order Shaman — класс коэрцируется
    let state = SelectionState { faction: Faction::Horde, race_id: "skyborne".into(), class_id: "shaman".into() };
    let next = state.after_faction_change(&data, Faction::Alliance);
    assert_eq!(next.faction, Faction::Alliance);
    assert_eq!(next.race_id, "skyborne");
    assert_eq!(next.class_id, "warrior"); // первый валидный класс High Order
    assert!(data.validate_selection(next.faction, &next.race_id, &next.class_id));
}

#[test]
fn test_skyborne_mage_survives_horde_to_alliance() {
    let data = GameData::load_embedded();
    // Mage валиден только в Alliance, поэтому сценарий: Alliance -> Horde -> Alliance
    let state = SelectionState { faction: Faction::Alliance, race_id: "skyborne".into(), class_id: "mage".into() };
    let via_horde = state.after_faction_change(&data, Faction::Horde);
    assert_eq!(via_horde.race_id, "skyborne");
    assert_eq!(via_horde.class_id, "warrior"); // mage невалиден для Windshaper
    let back = via_horde.after_faction_change(&data, Faction::Alliance);
    assert_eq!(back.race_id, "skyborne");
    assert!(data.validate_selection(back.faction, &back.race_id, &back.class_id));
}

#[test]
fn test_same_faction_change_is_noop() {
    let data = GameData::load_embedded();
    let state = SelectionState { faction: Faction::Alliance, race_id: "dwarf".into(), class_id: "paladin".into() };
    let next = state.after_faction_change(&data, Faction::Alliance);
    assert_eq!(next.race_id, "dwarf");
    assert_eq!(next.class_id, "paladin");
}
