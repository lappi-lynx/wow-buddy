use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::race_tile::{class_render_source, race_emoji, race_portraits};

#[test]
fn test_race_portraits_resolve_for_all_races() {
    let reg = AssetRegistry::load_embedded();
    for race in [
        "human", "dwarf", "night_elf", "gnome", "orc", "undead", "tauren", "troll", "skyborne",
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
fn test_unknown_race_keeps_emoji_fallback() {
    let reg = AssetRegistry::load_embedded();
    assert!(race_portraits(&reg, "ogre").is_none());
    assert_eq!(race_emoji("ogre"), "👤");
}

#[test]
fn test_class_render_resolves_for_all_classes() {
    let reg = AssetRegistry::load_embedded();
    for class in [
        "warrior", "paladin", "hunter", "rogue", "priest", "shaman", "mage", "warlock", "druid",
    ] {
        assert!(
            class_render_source(&reg, class).starts_with("data:"),
            "{class} icon must be embedded"
        );
    }
    // неизвестный класс -> пустая строка (вью покажет эмодзи-фоллбек)
    assert_eq!(class_render_source(&reg, "monk"), "");
}
