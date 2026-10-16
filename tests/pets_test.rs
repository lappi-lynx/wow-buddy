use std::collections::BTreeSet;

use wow_buddy::i18n::Language;
use wow_buddy::models::class_tips::LocalizedText;
use wow_buddy::models::pets::PetsData;

const ARTIFACT: &str = include_str!("../docs/wowhead-sources/pets-families.json");

fn assert_localized(t: &LocalizedText, what: &str) {
    for lang in [Language::En, Language::Ru, Language::Fi] {
        assert!(!t.get(lang).trim().is_empty(), "{what}: empty {lang:?}");
    }
}

/// (name, type_id, diet_id_set) derived from the raw artifact rows.
fn artifact_families() -> Vec<(String, String, BTreeSet<String>)> {
    let v: serde_json::Value = serde_json::from_str(ARTIFACT).expect("artifact parses");
    v["rows"]
        .as_array()
        .expect("rows array")
        .iter()
        .map(|row| {
            let cells: Vec<String> = row
                .as_array()
                .unwrap()
                .iter()
                .map(|c| c.as_str().unwrap_or_default().to_string())
                .collect();
            let diet: BTreeSet<String> = cells[5]
                .split(',')
                .map(|d| d.trim().to_lowercase().replace(' ', "_"))
                .filter(|d| !d.is_empty())
                .collect();
            (cells[2].clone(), cells[6].to_lowercase(), diet)
        })
        .collect()
}

#[test]
fn families_match_the_raw_artifact() {
    let artifact = artifact_families();
    assert_eq!(artifact.len(), 17, "the raw artifact must hold 17 rows");
    let data = PetsData::load_embedded();
    assert_eq!(data.families.len(), artifact.len(), "shipped family count");
    for (name, pet_type, diet) in artifact {
        let f = data
            .families
            .iter()
            .find(|f| f.name == name)
            .unwrap_or_else(|| panic!("missing family {name}"));
        assert_eq!(f.pet_type, pet_type, "{name} type");
        let shipped: BTreeSet<String> = f.diet.iter().cloned().collect();
        assert_eq!(shipped, diet, "{name} diet (order-insensitive)");
        assert!(!f.note.get(Language::En).trim().is_empty(), "{name} note");
    }
    let ids: BTreeSet<&str> = data.families.iter().map(|f| f.id.as_str()).collect();
    assert_eq!(ids.len(), 17, "family ids unique");
}

#[test]
fn every_type_and_diet_id_has_a_legend() {
    let data = PetsData::load_embedded();
    let type_ids: BTreeSet<&str> = data.legends.types.keys().map(|s| s.as_str()).collect();
    let want: BTreeSet<&str> = ["cunning", "ferocity", "tenacity"].into_iter().collect();
    assert_eq!(type_ids, want, "type legend ids");
    for f in &data.families {
        assert!(data.legends.types.contains_key(&f.pet_type), "{} type legend", f.name);
        for d in &f.diet {
            let l = data
                .legends
                .diets
                .get(d)
                .unwrap_or_else(|| panic!("{} diet legend {d}", f.name));
            assert_localized(l, &format!("diet legend {d}"));
        }
        assert_localized(&f.note, &format!("family {} note", f.id));
    }
    for (id, l) in &data.legends.types {
        assert_localized(l, &format!("type legend {id}"));
    }
}

#[test]
fn every_source_id_points_at_a_docs_file() {
    let data = PetsData::load_embedded();
    assert!(!data.sources.is_empty());
    for (id, sref) in &data.sources {
        assert!(!sref.kind.trim().is_empty(), "{id} kind");
        assert!(sref.path.starts_with("docs/"), "{id} path under docs/");
        assert!(std::path::Path::new(&sref.path).exists(), "{id} missing file {}", sref.path);
    }
    let mut used: BTreeSet<&str> = BTreeSet::new();
    for f in &data.families {
        used.insert(f.source_id.as_str());
    }
    for b in &data.basics {
        used.insert(b.source_id.as_str());
    }
    for p in &data.notable {
        used.insert(p.source_id.as_str());
    }
    for m in &data.minions {
        used.insert(m.source_id.as_str());
    }
    if let Some(c) = &data.companion {
        used.insert(c.source_id.as_str());
    }
    for id in used {
        assert!(data.sources.contains_key(id), "unknown source_id {id}");
    }
}

#[test]
fn partial_documents_parse_with_defaults() {
    let data: PetsData = serde_json::from_str(r#"{"families":[]}"#).expect("partial pack parses");
    assert!(data.families.is_empty() && data.basics.is_empty() && data.notable.is_empty());
    assert!(data.minions.is_empty() && data.companion.is_none() && data.first_pick.is_empty());
    assert!(data.sources.is_empty() && data.legends.diets.is_empty() && data.legends.types.is_empty());
}

#[test]
fn first_picks_are_real_families() {
    let data = PetsData::load_embedded();
    assert_eq!(data.first_pick.len(), 3);
    assert!(data.first_pick.iter().any(|f| f == "cat"));
    let ids: BTreeSet<&str> = data.families.iter().map(|f| f.id.as_str()).collect();
    for pick in &data.first_pick {
        assert!(ids.contains(pick.as_str()), "unknown first pick {pick}");
    }
}

#[test]
fn tame_basics_lead_with_the_level_10_unlock() {
    let data = PetsData::load_embedded();
    assert_eq!(data.basics[0].id, "unlock_tame_beast");
    assert_eq!(data.basics[0].level_at, Some(10));
    let ids: Vec<&str> = data.basics.iter().map(|b| b.id.as_str()).collect();
    for want in ["feed_pet", "happiness", "loyalty", "mend_pet", "stables"] {
        assert!(ids.contains(&want), "missing basic {want}");
    }
    let mend = data.basics.iter().find(|b| b.id == "mend_pet").unwrap();
    assert_eq!(mend.level_at, Some(12));
    for b in &data.basics {
        assert_localized(&b.title, &format!("basic {} title", b.id));
        assert_localized(&b.body, &format!("basic {} body", b.id));
    }
}

#[test]
fn notable_pets_are_in_bracket_with_known_homes() {
    let data = PetsData::load_embedded();
    let family_ids: BTreeSet<&str> = data.families.iter().map(|f| f.id.as_str()).collect();
    let mut last = 0u8;
    for p in &data.notable {
        assert!((10..=30).contains(&p.min_level), "{} level {}", p.name, p.min_level);
        assert!(p.min_level >= last, "{} out of order", p.name);
        last = p.min_level;
        assert!(family_ids.contains(p.family_id.as_str()), "{} unknown family", p.name);
        assert_localized(&p.why, &format!("{} why", p.name));
        assert_localized(&p.where_hint, &format!("{} where", p.name));
    }
    for (name, zone) in [
        ("The Rake", "Mulgore"),
        ("Savannah Patriarch", "The Barrens"),
        ("Greater Flesh Ripper", "Westfall"),
        ("Giant Grizzled Bear", "Silverpine Forest"),
        ("Leech Widow", "Wetlands"),
        ("Cloud Serpent", "Thousand Needles"),
    ] {
        let p = data.notable.iter().find(|p| p.name == name).unwrap_or_else(|| panic!("missing {name}"));
        assert!(p.where_hint.get(Language::En).contains(zone), "{name} in {zone}");
    }
}

#[test]
fn warlock_minions_cover_1_to_30() {
    let data = PetsData::load_embedded();
    let got: Vec<(&str, u8)> = data.minions.iter().map(|m| (m.name.as_str(), m.learned_at)).collect();
    for want in [("Imp", 1), ("Voidwalker", 10), ("Succubus", 20), ("Felhunter", 30)] {
        assert!(got.contains(&want), "missing minion {want:?}");
    }
    for m in &data.minions {
        assert_localized(&m.role, &format!("minion {} role", m.name));
    }
}

#[test]
fn companion_card_teaches_vanity_pets() {
    let data = PetsData::load_embedded();
    let c = data.companion.as_ref().expect("companion card present");
    assert_localized(&c.title, "companion title");
    assert_localized(&c.body, "companion body");
    let en = c.body.get(Language::En);
    assert!(en.contains("Westfall") && en.contains("Mechanical Squirrel"), "needs concrete examples");
    assert!(!en.to_lowercase().contains("fish"), "fishing is not a companion source (KB-corrected)");
}

use wow_buddy::models::pets::{pet_card_kind, pets_sections, PetCardKind, PetsSection};
use wow_buddy::models::GameData;

#[test]
fn card_kind_and_sections_cover_the_real_classes() {
    let game = GameData::load_embedded();
    let (mut hunters, mut warlocks) = (0, 0);
    for class in &game.classes {
        match pet_card_kind(&class.id) {
            PetCardKind::Hunter => hunters += 1,
            PetCardKind::Warlock => warlocks += 1,
            PetCardKind::Companion => {}
        }
    }
    assert_eq!(hunters, 1, "exactly one hunter class");
    assert_eq!(warlocks, 1, "exactly one warlock class");
    assert!(matches!(pet_card_kind(""), PetCardKind::Companion), "no hero -> companion view");
    assert_eq!(
        pets_sections(PetCardKind::Hunter),
        &[PetsSection::Basics, PetsSection::Families, PetsSection::Notable, PetsSection::Companion]
    );
    assert_eq!(pets_sections(PetCardKind::Warlock), &[PetsSection::Minions, PetsSection::Companion]);
    assert_eq!(pets_sections(PetCardKind::Companion), &[PetsSection::Companion]);
}

#[test]
fn pets_card_only_shows_where_the_guide_has_content() {
    use wow_buddy::models::pets::shows_pets_card;
    assert!(shows_pets_card(pet_card_kind("hunter")));
    assert!(shows_pets_card(pet_card_kind("warlock")));
    for class in ["warrior", "priest", "shaman", "mage", "rogue", "druid", "paladin"] {
        assert!(!shows_pets_card(pet_card_kind(class)), "{class} should hide the entry");
    }
}

#[test]
fn every_family_and_minion_has_an_embedded_icon() {
    use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
    let reg = AssetRegistry::load_embedded();
    let d = PetsData::load_embedded();
    for f in d.families.iter() {
        assert!(!f.icon_id.is_empty(), "{}: no icon id", f.id);
        assert_ne!(
            reg.item_icon(&f.icon_id),
            ICON_FALLBACK,
            "{}: {} has no embedded bytes",
            f.id,
            f.icon_id
        );
    }
    for m in d.minions.iter() {
        assert!(!m.icon_id.is_empty(), "{}: no icon id", m.id);
        assert_ne!(
            reg.item_icon(&m.icon_id),
            ICON_FALLBACK,
            "{}: {} has no embedded bytes",
            m.id,
            m.icon_id
        );
    }
}
