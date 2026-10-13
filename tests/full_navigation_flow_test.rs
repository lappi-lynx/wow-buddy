use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::models::gear_planner::GearPlannerData;
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

#[test]
fn test_gear_planner_data_covers_seeded_class_and_reports_gaps() {
    let _ = Route::GearPlanner {};
    let gear = GearPlannerData::load_embedded();
    // сидированный класс имеет данные по обеим фракциям
    assert!(!gear.bands_for("warrior", Faction::Alliance).is_empty());
    assert!(!gear.bands_for("warrior", Faction::Horde).is_empty());
    // несидированный класс честно пуст (экран покажет "no data", не панику)
    assert!(gear.bands_for("priest", Faction::Alliance).is_empty());
}

#[test]
fn test_art_missing_never_breaks_screens() {
    let reg = AssetRegistry::load_embedded();
    // отсутствие арта — штатное состояние: хелперы возвращают фоллбек, не паникуют
    assert!(reg.art_uri("nope").is_none());
    let _ = wow_buddy::components::race_tile::race_render_source(&reg, "skyborne", Faction::Horde);
}
