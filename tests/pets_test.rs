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
