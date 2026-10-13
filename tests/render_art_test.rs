use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::race_tile::{class_render_source, race_emoji, race_render_source};
use wow_buddy::models::Faction;

#[test]
fn test_race_render_falls_back_to_emoji() {
    let reg = AssetRegistry::load_embedded();
    // человек: placeholder арта сидирован -> data-URI
    assert!(race_render_source(&reg, "human", Faction::Alliance).starts_with("data:"));
    // Skyborne: арта в ките нет -> определенный фоллбек-эмодзи
    assert_eq!(race_render_source(&reg, "skyborne", Faction::Horde), race_emoji("skyborne"));
}

#[test]
fn test_class_render_falls_back_to_emoji() {
    let reg = AssetRegistry::load_embedded();
    // классы ещё не наполнены -> пустая строка фоллбека (вью покажет текстовую метку)
    assert_eq!(class_render_source(&reg, "warrior"), "");
}
