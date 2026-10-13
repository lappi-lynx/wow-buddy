use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::faction_shield::faction_art_source;
use wow_buddy::models::Faction;

#[test]
fn test_faction_art_resolves_for_both_factions() {
    let reg = AssetRegistry::load_embedded();
    // Оба щита наполнены артом кита: возвращается data-URI.
    // Эмодзи из метаданных остаётся фоллбеком на случай отсутствия арта —
    // его значения покрыты `faction_picker_view_test`.
    assert!(faction_art_source(&reg, Faction::Alliance).starts_with("data:"));
    assert!(faction_art_source(&reg, Faction::Horde).starts_with("data:"));
}
