use wow_buddy::assets::generated::{ARTS, ICONS};
use wow_buddy::assets::registry::AssetRegistry;
use std::collections::HashSet;

#[test]
fn test_art_uri_is_none_for_unknown_and_some_for_seeded_placeholder() {
    let reg = AssetRegistry::load_embedded();
    assert!(reg.art_uri("zone_art_does_not_exist").is_none());
    // в манифест кладём один placeholder-арт, чтобы обе ветки были проверены
    assert!(reg.art_uri("faction_alliance_shield").unwrap().starts_with("data:"));
}

#[test]
fn test_manifest_tables_agree_by_kind() {
    let reg = AssetRegistry::load_embedded();
    let icons: HashSet<&str> = ICONS.iter().map(|(id, _)| *id).collect();
    let arts: HashSet<&str> = ARTS.iter().map(|(id, _)| *id).collect();
    for (id, entry) in reg.iter() {
        if entry.kind == "item_icon" {
            assert!(icons.contains(id.as_str()), "icon {id} declared but not generated");
        } else {
            assert!(arts.contains(id.as_str()), "art {id} declared but not generated");
        }
    }
}
