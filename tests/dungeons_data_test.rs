use std::collections::BTreeSet;

use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
use wow_buddy::models::dungeon::DungeonsData;

#[test]
fn dungeons_json_is_well_formed() {
    let data = DungeonsData::load_embedded();
    let ids: Vec<&str> = data.dungeons.iter().map(|d| d.id.as_str()).collect();
    let unique: BTreeSet<&str> = ids.iter().copied().collect();
    assert_eq!(unique.len(), ids.len(), "duplicate dungeon ids");
    for want in [
        "deadmines",
        "wailing_caverns",
        "shadowfang_keep",
        "hall_of_thanes",
        "ragefire_chasm",
    ] {
        assert!(unique.contains(want), "missing dungeon {want}");
    }
    for d in data.dungeons.iter() {
        assert!(d.min_level <= d.max_level, "{}: min/max inverted", d.id);
        assert!(!d.rules.is_empty(), "{}: no rules", d.id);
        assert!(!d.level_range.is_empty(), "{}: no level range", d.id);
        assert!(!d.estimated_time.is_empty(), "{}: no clear time", d.id);
    }
}

#[test]
fn every_recommended_level_cites_a_known_source() {
    let data = DungeonsData::load_embedded();
    for d in data.dungeons.iter() {
        if let Some(rec) = &d.recommended_avg_level {
            assert!(
                d.sources.contains_key(&rec.source_id),
                "{}: recommended level cites unknown source {}",
                d.id,
                rec.source_id
            );
        }
    }
}

#[test]
fn every_loot_row_is_complete_and_its_icon_resolves() {
    let reg = AssetRegistry::load_embedded();
    let data = DungeonsData::load_embedded();
    for d in data.dungeons.iter() {
        for loot in d.loot.iter() {
            assert_ne!(loot.item_id, 0, "{}: loot row without an item id", d.id);
            assert!(!loot.name.is_empty(), "{}: loot row without a name", d.id);
            assert!(!loot.icon_id.is_empty(), "{}: loot row without an icon", d.id);
            assert_ne!(
                reg.item_icon(&loot.icon_id),
                ICON_FALLBACK,
                "{}: {} has no embedded bytes",
                d.id,
                loot.icon_id
            );
        }
    }
}

#[test]
fn deadmines_pilot_content_is_complete() {
    let data = DungeonsData::load_embedded();
    let dm = data
        .dungeons
        .iter()
        .find(|d| d.id == "deadmines")
        .expect("deadmines entry");
    assert!(
        dm.recommended_avg_level.is_some(),
        "deadmines: no recommended level"
    );
    assert!(dm.mobs.len() >= 5, "deadmines: mobs {}", dm.mobs.len());
    assert!(
        dm.watch_out.len() >= 3,
        "deadmines: watch_out {}",
        dm.watch_out.len()
    );
    assert!(dm.loot.len() >= 3, "deadmines: loot {}", dm.loot.len());
    assert!(dm.prep.is_some(), "deadmines: no prep");
}

#[test]
fn all_dungeons_content_complete() {
    let data = DungeonsData::load_embedded();
    for d in data.dungeons.iter() {
        assert!(d.recommended_avg_level.is_some(), "{}: no recommended level", d.id);
        assert!(d.mobs.len() >= 5, "{}: mobs {}", d.id, d.mobs.len());
        assert!(d.watch_out.len() >= 3, "{}: watch_out {}", d.id, d.watch_out.len());
        assert!(d.prep.is_some(), "{}: no prep", d.id);
        assert!(d.loot.len() >= 3, "{}: loot {}", d.id, d.loot.len());
    }
}
