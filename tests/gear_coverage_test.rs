use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
use wow_buddy::models::gear_planner::{GearPlannerData, SLOT_ORDER};
use wow_buddy::models::Faction;

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

/// Honest per-band slot availability before 30: the 12–16 basics (no
/// verified head/neck/shoulders/ring/trinket/wrist at that level), everything
/// but the trinket by 20–24 (class trinkets start ~28), and all but the neck
/// by 24–30 (the early neck keeps serving; no verified upgrade exists ≤30).
pub const MIN_SLOTS: [(&str, usize); 3] = [("12-16", 6), ("20-24", 10), ("24-30", 10)];

/// Slot completeness for the given factions (the 1-6 starter band is exempt:
/// its slots are deliberately empty until starter gear is authored). Every
/// present slot must be well-formed and each band must reach its minimum.
///
/// Like weapon levels, a slot item's `level` keeps its VERIFIED required
/// level and is deliberately not bounded by the band (the 12-16 crafted
/// basics are req 7-11; the artifacts record the band assignment).
pub fn assert_class_slots_complete(data: &GearPlannerData, class_id: &str, factions: &[Faction]) {
    let gear = data
        .classes
        .get(class_id)
        .unwrap_or_else(|| panic!("{class_id} missing from gear.json"));
    for faction in factions {
        let (key, bands) = match faction {
            Faction::Alliance => ("alliance", &gear.alliance),
            Faction::Horde => ("horde", &gear.horde),
        };
        for band in bands {
            if band.id == "1-6" {
                continue;
            }
            let min = MIN_SLOTS
                .iter()
                .find(|(id, _)| *id == band.id)
                .map(|(_, n)| *n)
                .unwrap_or_else(|| panic!("unknown band {}", band.id));
            assert!(
                band.slots.len() >= min,
                "{class_id}/{key}/{} has {} slots, needs >= {min}",
                band.id,
                band.slots.len()
            );
            for (slot, item) in &band.slots {
                assert!(
                    SLOT_ORDER.contains(&slot.as_str()),
                    "{class_id}/{key}/{}/{slot} unknown slot",
                    band.id
                );
                assert!(item.item_id.is_some(), "{class_id}/{key}/{}/{slot} item_id", band.id);
                assert!(item.icon_id.is_some(), "{class_id}/{key}/{}/{slot} icon_id", band.id);
                assert!(item.provenance.is_some(), "{class_id}/{key}/{}/{slot} provenance", band.id);
                assert!(!item.source.trim().is_empty(), "{class_id}/{key}/{}/{slot} source", band.id);
            }
        }
    }
}

#[test]
fn shaman_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "shaman",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn mage_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "mage",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn warlock_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "warlock",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn druid_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "druid",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn priest_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "priest",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn rogue_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "rogue",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn paladin_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "paladin",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn hunter_slots_complete() {
    // Both factions (Alliance elf + Horde). The other classes land with the
    // remaining fills; the umbrella gate (Task 13) requires all nine.
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "hunter",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn warrior_slots_complete() {
    assert_class_slots_complete(
        &GearPlannerData::load_embedded(),
        "warrior",
        &[Faction::Alliance, Faction::Horde],
    );
}

#[test]
fn all_classes_slots_complete() {
    let data = GearPlannerData::load_embedded();
    for c in EXPECTED_CLASSES {
        assert_class_slots_complete(&data, c, &[Faction::Alliance, Faction::Horde]);
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
                let weapon_icons = band.weapons.iter().map(|w| w.icon_id.clone());
                let slot_icons = band.slots.values().map(|s| s.icon_id.clone());
                for icon in weapon_icons.chain(slot_icons).flatten() {
                    assert_ne!(
                        reg.item_icon(&icon),
                        ICON_FALLBACK,
                        "{class} {icon} has no embedded bytes"
                    );
                }
            }
        }
    }
}
