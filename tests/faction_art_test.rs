use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::faction_shield::faction_art_source;
use wow_buddy::models::Faction;

#[test]
fn test_faction_art_falls_back_to_emoji_when_absent() {
    let reg = AssetRegistry::load_embedded();
    // Horde ещё не наполнен — обязан вернуться эмодзи из метаданных
    assert_eq!(faction_art_source(&reg, Faction::Horde), "⚡");
    // Alliance имеет placeholder-арт — возвращается data-URI
    assert!(faction_art_source(&reg, Faction::Alliance).starts_with("data:"));
}
