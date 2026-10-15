use wow_buddy::i18n::Language;
use wow_buddy::models::legacy::{Allocation, LegacyCatalog};

fn alloc(pairs: &[(&str, u8)]) -> Vec<Allocation> {
    pairs
        .iter()
        .map(|(id, p)| Allocation {
            id: id.to_string(),
            points: *p,
        })
        .collect()
}

#[test]
fn catalog_has_21_perks_across_three_trees() {
    let c = LegacyCatalog::load_embedded();
    assert_eq!(c.perks.len(), 21);
    for tree in ["professions", "adventure", "resourcefulness"] {
        assert_eq!(
            c.perks.iter().filter(|p| p.tree == tree).count(),
            7,
            "{tree}"
        );
    }
    for p in &c.perks {
        for l in [Language::En, Language::Ru, Language::Fi] {
            assert!(!p.name.get(l).trim().is_empty(), "{} name {l:?}", p.id);
            assert!(!p.desc.get(l).trim().is_empty(), "{} desc {l:?}", p.id);
        }
    }
}

#[test]
fn every_preset_totals_16_and_validates() {
    let c = LegacyCatalog::load_embedded();
    assert_eq!(c.presets.len(), 3);
    for preset in &c.presets {
        assert_eq!(c.total_points(preset), 16, "{} points", preset.id);
        assert!(
            c.validate_allocation(&preset.allocation).is_ok(),
            "{} invalid",
            preset.id
        );
    }
}

#[test]
fn validator_rejects_broken_allocations() {
    let c = LegacyCatalog::load_embedded();
    // Bartering needs 5 Professions — alone it is invalid
    assert!(c.validate_allocation(&alloc(&[("bartering", 2)])).is_err());
    // Talented needs Well Rested 5/5 + 5 Adventure
    assert!(c.validate_allocation(&alloc(&[("talented", 5)])).is_err());
    // 17 points exceeds the seasonal cap
    assert!(c
        .validate_allocation(&alloc(&[
            ("well_rested", 5),
            ("thrill_of_adventure", 5),
            ("talented", 5),
            ("field_guide", 2)
        ]))
        .is_err());
}
