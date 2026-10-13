use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::assets::zone::zone_art_source;

#[test]
fn test_zone_art_optional_and_seeded() {
    let reg = AssetRegistry::load_embedded();
    assert!(zone_art_source(&reg, "no_such_zone").is_none());
    assert!(zone_art_source(&reg, "shadowglen").unwrap().starts_with("data:"));
}
