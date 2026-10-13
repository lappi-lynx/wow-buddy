use wow_buddy::assets::generated::{ARTS, ICONS};
use wow_buddy::assets::registry::AssetRegistry;
use std::collections::HashSet;

#[test]
fn test_art_uri_is_none_for_unknown_and_some_for_seeded_placeholder() {
    let reg = AssetRegistry::load_embedded();
    assert!(reg.art_uri("zone_art_does_not_exist").is_none());
    // хотя бы один арт объявлен и сгенерирован, чтобы обе ветки были проверены
    assert!(reg.art_uri("faction_alliance_shield").unwrap().starts_with("data:"));
}

#[test]
fn test_manifest_tables_agree_by_kind() {
    // declared -> generated: every manifest entry must exist in the table
    // matching its kind.
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

#[test]
fn test_generated_tables_agree_with_manifest_by_kind() {
    // generated -> declared: a generated entry with no manifest counterpart
    // would ship without attribution (an "attribution hole").
    let reg = AssetRegistry::load_embedded();
    for (id, _) in ICONS.iter() {
        let entry = reg
            .entry(id)
            .unwrap_or_else(|| panic!("generated icon {id} has no manifest entry"));
        assert_eq!(entry.kind, "item_icon", "icon {id} declared with wrong kind");
    }
    for (id, _) in ARTS.iter() {
        let entry = reg
            .entry(id)
            .unwrap_or_else(|| panic!("generated art {id} has no manifest entry"));
        assert_ne!(entry.kind, "item_icon", "art {id} declared as an icon");
    }
}
