use wow_buddy::models::{GameData, Faction};
use wow_buddy::router::Route;
use wow_buddy::selection::SelectionState;

#[test]
fn test_all_routes_defined() {
    let routes = vec![
        Route::FactionPicker {},
        Route::RaceSelector {},
        Route::DressingRoom {},
        Route::ProgressionGuide {},
        Route::DungeonCodex {},
    ];
    assert_eq!(routes.len(), 5);
}

#[test]
fn test_onboarding_lifecycle_end_to_end() {
    let data = GameData::load_embedded();

    // Alliance → human → hunter (новая комбинация 1.60.1) — валидна
    let mut state = SelectionState { faction: Faction::Alliance, race_id: "human".into(), class_id: "hunter".into() };
    assert!(data.validate_selection(state.faction, &state.race_id, &state.class_id));

    // Переключение на Орду: человек сбрасывается на орка
    state = state.after_faction_change(&data, Faction::Horde);
    assert_eq!(state.race_id, "orc");
    assert!(data.validate_selection(state.faction, &state.race_id, &state.class_id));

    // Undead Paladin (новая комбинация 1.60.1) валиден в Орде
    state = SelectionState { faction: Faction::Horde, race_id: "undead".into(), class_id: "paladin".into() };
    assert!(data.validate_selection(state.faction, &state.race_id, &state.class_id));

    // Skyborne переживает переключение фракции, класс коэрцируется при необходимости
    state = SelectionState { faction: Faction::Horde, race_id: "skyborne".into(), class_id: "shaman".into() };
    state = state.after_faction_change(&data, Faction::Alliance);
    assert_eq!(state.race_id, "skyborne");
    assert_eq!(state.class_id, "warrior");
    assert!(data.validate_selection(state.faction, &state.race_id, &state.class_id));
}
