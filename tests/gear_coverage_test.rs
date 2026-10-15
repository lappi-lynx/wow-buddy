use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
use wow_buddy::models::gear_planner::GearPlannerData;

pub const EXPECTED_CLASSES: [&str; 9] = [
    "warrior", "paladin", "hunter", "rogue", "priest", "shaman", "mage", "warlock", "druid",
];
pub const BANDS: [(&str, u8, u8); 4] = [
    ("1-6", 1, 6),
    ("12-16", 12, 16),
    ("20-24", 20, 24),
    ("24-30", 24, 30),
];

/// Every class ships both factions' four bands with real, sourced content.
/// The `1-6` starter band is exempt from the provenance/item-id rules (it
/// carries the generic starter weapon deliberately, mirroring warrior).
///
/// Weapon `level` is deliberately NOT bounded by the band's min/max: the KB
/// guide files some pickups with the bracket they route with (Cruel Barb 19,
/// Cookie's/Emberstone 17, Meteor Shard 23, Odo's 21), and each per-class
/// artifact records why. Band placement is a reviewed decision, not a check.
pub fn assert_class_complete(data: &GearPlannerData, class_id: &str) {
    let gear = data
        .classes
        .get(class_id)
        .unwrap_or_else(|| panic!("{class_id} missing from gear.json"));
    for (faction, bands) in [("alliance", &gear.alliance), ("horde", &gear.horde)] {
        assert_eq!(bands.len(), BANDS.len(), "{class_id}/{faction} band count");
        for (i, (id, min, max)) in BANDS.iter().enumerate() {
            let band = &bands[i];
            assert_eq!(band.id, *id, "{class_id}/{faction} band {i} id");
            assert_eq!(band.min_level, *min, "{class_id}/{faction}/{id} min");
            assert_eq!(band.max_level, *max, "{class_id}/{faction}/{id} max");
            assert!(
                !band.armor.text.trim().is_empty(),
                "{class_id}/{faction}/{id} armor text"
            );
            if *id != "1-6" {
                assert!(
                    band.armor.provenance.is_some(),
                    "{class_id}/{faction}/{id} armor provenance"
                );
                assert!(
                    band.weapons.len() >= 2,
                    "{class_id}/{faction}/{id} needs >=2 weapons"
                );
                for w in &band.weapons {
                    assert!(
                        w.item_id.is_some(),
                        "{class_id}/{faction}/{id} weapon {} item_id",
                        w.name
                    );
                    assert!(
                        w.icon_id.is_some(),
                        "{class_id}/{faction}/{id} weapon {} icon_id",
                        w.name
                    );
                    assert!(
                        w.provenance.is_some(),
                        "{class_id}/{faction}/{id} weapon {} provenance",
                        w.name
                    );
                    assert!(
                        !w.source.trim().is_empty(),
                        "{class_id}/{faction}/{id} weapon {} source",
                        w.name
                    );
                }
            }
        }
    }
}

#[test]
fn warrior_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "warrior");
}

#[test]
fn paladin_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "paladin");
}

#[test]
fn hunter_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "hunter");
}

#[test]
fn rogue_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "rogue");
}

#[test]
fn priest_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "priest");
}

#[test]
fn shaman_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "shaman");
}

#[test]
fn mage_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "mage");
}

#[test]
fn warlock_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "warlock");
}

#[test]
fn druid_is_complete() {
    assert_class_complete(&GearPlannerData::load_embedded(), "druid");
}

#[test]
fn all_classes_complete() {
    let data = GearPlannerData::load_embedded();
    assert_eq!(data.classes.len(), EXPECTED_CLASSES.len());
    for c in EXPECTED_CLASSES {
        assert_class_complete(&data, c);
    }
}

#[test]
fn every_gear_icon_resolves_to_embedded_bytes() {
    let reg = AssetRegistry::load_embedded();
    let data = GearPlannerData::load_embedded();
    for class in EXPECTED_CLASSES {
        let gear = data.classes.get(class).unwrap_or_else(|| panic!("{class} missing"));
        for bands in [&gear.alliance, &gear.horde] {
            for band in bands {
                for w in &band.weapons {
                    if let Some(id) = &w.icon_id {
                        assert_ne!(
                            reg.item_icon(id),
                            ICON_FALLBACK,
                            "{class} {id} has no embedded bytes"
                        );
                    }
                }
            }
        }
    }
}
