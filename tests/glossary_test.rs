use std::collections::HashMap;

use wow_buddy::models::glossary::{get_term, TERMS};

fn dict(raw: &str) -> HashMap<String, String> {
    serde_json::from_str(raw).expect("locale json must parse")
}

#[test]
fn every_term_has_trilingual_text_and_short() {
    let en = dict(include_str!("../assets/i18n/en.json"));
    let ru = dict(include_str!("../assets/i18n/ru.json"));
    let fi = dict(include_str!("../assets/i18n/fi.json"));
    for term in TERMS {
        for (name, d) in [("en", &en), ("ru", &ru), ("fi", &fi)] {
            let text = d
                .get(term.text_key)
                .unwrap_or_else(|| panic!("{name}: missing {}", term.text_key));
            assert!(!text.trim().is_empty(), "{name}: empty {}", term.text_key);
            let short = d
                .get(term.short_key)
                .unwrap_or_else(|| panic!("{name}: missing {}", term.short_key));
            assert!(!short.trim().is_empty(), "{name}: empty {}", term.short_key);
        }
    }
}

#[test]
fn minimum_term_set_is_covered() {
    let want = [
        "tank",
        "melee_dps",
        "rage",
        "aggro",
        "taunt",
        "respec",
        "specialization",
        "legacy_points",
        "rested_xp",
        "hearthstone",
        "flight_path",
        "need_greed",
        "tier2",
    ];
    for id in want {
        assert!(get_term(id).is_some(), "missing glossary term: {id}");
    }
    assert_eq!(TERMS.len(), want.len(), "unexpected extra terms");
}
