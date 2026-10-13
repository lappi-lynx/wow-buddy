use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
use wow_buddy::models::gear_planner::GearPlannerData;
use wow_buddy::models::Faction;

#[test]
fn test_unknown_icon_returns_fallback() {
    let reg = AssetRegistry::load_embedded();
    assert_eq!(reg.item_icon("inv_does_not_exist"), ICON_FALLBACK);
}

#[test]
fn test_every_gear_icon_id_is_declared_in_manifest() {
    let reg = AssetRegistry::load_embedded();
    let gear = GearPlannerData::load_embedded();
    for faction in [Faction::Alliance, Faction::Horde] {
        for band in gear.bands_for("warrior", faction) {
            for w in &band.weapons {
                if let Some(id) = &w.icon_id {
                    assert!(reg.has(id), "icon_id {id} not declared in manifest.json");
                }
            }
        }
    }
}

#[test]
fn test_manifest_entries_have_a_source() {
    let reg = AssetRegistry::load_embedded();
    for (id, entry) in reg.iter() {
        assert!(!entry.source.is_empty(), "asset {id} missing source");
    }
}

#[test]
fn test_manifest_and_generated_icons_agree() {
    use std::collections::HashSet;
    use wow_buddy::assets::generated::ICONS;

    let reg = AssetRegistry::load_embedded();
    let generated: HashSet<&str> = ICONS.iter().map(|(id, _)| *id).collect();
    let declared: HashSet<String> = reg.iter().map(|(id, _)| id.clone()).collect();

    // declared-but-ungenerated would silently render the fallback
    for (id, _) in reg.iter() {
        assert!(
            generated.contains(id.as_str()),
            "declared asset {id} has no generated icon"
        );
    }
    // generated-but-undeclared is an attribution hole
    for (id, _) in ICONS.iter() {
        assert!(
            declared.contains(*id),
            "generated icon {id} is not declared in the manifest"
        );
    }
}
