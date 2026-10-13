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

// NOTE: the manifest ↔ generated-table integrity check now lives in
// `tests/asset_art_registry_test.rs` (`test_manifest_tables_agree_by_kind`),
// because there are two tables (ICONS and ARTS) rather than one.
