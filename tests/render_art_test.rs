use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::race_tile::{class_render_source, race_emoji, race_portraits};

#[test]
fn test_race_portraits_resolve_for_all_classic_races() {
    let reg = AssetRegistry::load_embedded();
    for race in [
        "human", "dwarf", "night_elf", "gnome", "orc", "undead", "tauren", "troll",
    ] {
        let (male, female) = race_portraits(&reg, race)
            .unwrap_or_else(|| panic!("{race} must have male+female portraits"));
        assert!(male.starts_with("data:"), "{race} male portrait must be embedded");
        assert!(
            female.starts_with("data:"),
            "{race} female portrait must be embedded"
        );
    }
}

#[test]
fn test_skyborne_has_no_portraits_and_keeps_emoji() {
    let reg = AssetRegistry::load_embedded();
    // Skyborne — кастомная раса Forever: портретов в ките нет, остаётся эмодзи
    assert!(race_portraits(&reg, "skyborne").is_none());
    assert_eq!(race_emoji("skyborne"), "🪶");
}

#[test]
fn test_class_render_falls_back_to_emoji() {
    let reg = AssetRegistry::load_embedded();
    // классы ещё не наполнены -> пустая строка фоллбека (вью покажет текстовую метку)
    assert_eq!(class_render_source(&reg, "warrior"), "");
}
