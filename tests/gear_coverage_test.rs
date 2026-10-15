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
