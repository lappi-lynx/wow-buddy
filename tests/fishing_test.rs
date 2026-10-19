use wow_buddy::i18n::Language;
use wow_buddy::models::fishing::{FishingData, ItemRef};
use wow_buddy::models::localized::Tr;

#[test]
fn fishing_card_has_three_trilingual_steps() {
    let d = FishingData::load_embedded();
    assert_eq!(d.steps.len(), 3);
    for step in &d.steps {
        for l in [Language::En, Language::Ru, Language::Fi] {
            assert!(!step.text.get(l).trim().is_empty(), "{} {l:?}", step.id);
        }
    }
    for l in [Language::En, Language::Ru, Language::Fi] {
        assert!(!d.title.get(l).trim().is_empty(), "title {l:?}");
        assert!(!d.tie_in.get(l).trim().is_empty(), "tie_in {l:?}");
    }
    assert!(d.tie_in.en.contains("Cooking"));
}

#[test]
fn card_only_json_still_parses() {
    // Backward compatibility: a fishing.json carrying only the original card
    // fields must keep deserializing (the guide collections default empty).
    let minimal = r#"{"title":{"en":"t","ru":"t","fi":"t"},"steps":[],"tie_in":{"en":"t","ru":"t","fi":"t"}}"#;
    let d: FishingData = serde_json::from_str(minimal).expect("card-only JSON must parse");
    assert!(d.sources.is_empty());
    assert!(d.shop.is_empty());
    assert!(d.basics.is_empty());
    assert!(d.trainers.is_none());
    assert!(d.tiers.is_empty());
    assert!(d.artisan_quest.is_none());
}

#[test]
fn fishing_guide_sections_are_well_formed() {
    let d = FishingData::load_embedded();
    assert!(d.basics.len() >= 7, "basics: {}", d.basics.len());
    let t = d.trainers.as_ref().expect("trainers");
    assert_eq!(t.capitals.len(), 6);
    assert_eq!(t.starting_zones.len(), 7);
    assert_eq!(d.tiers.len(), 4);
    let ids: Vec<&str> = d.tiers.iter().map(|tier| tier.id.as_str()).collect();
    assert_eq!(ids, ["1-75", "75-150", "150-225", "225-300"]);
    assert!(d.artisan_quest.is_some());
}

#[test]
fn fishing_tiers_chain_and_order() {
    let d = FishingData::load_embedded();
    let pairs: Vec<(u16, u16)> = d.tiers.iter().map(|tier| (tier.min, tier.max)).collect();
    assert_eq!(pairs, [(1, 75), (75, 150), (150, 225), (225, 300)]);
}

#[test]
fn every_guide_source_resolves_and_exists() {
    let d = FishingData::load_embedded();
    let mut ids: Vec<&str> = Vec::new();
    for step in &d.basics {
        ids.push(&step.source_id);
    }
    if let Some(t) = &d.trainers {
        ids.extend(t.capitals.iter().map(|c| c.source_id.as_str()));
        ids.extend(t.starting_zones.iter().map(|s| s.source_id.as_str()));
    }
    ids.extend(d.tiers.iter().map(|tier| tier.source_id.as_str()));
    if let Some(q) = &d.artisan_quest {
        ids.push(&q.source_id);
    }
    for id in ids {
        assert!(d.sources.contains_key(id), "unknown source {id}");
    }
    for src in d.sources.values() {
        assert!(
            std::path::Path::new(&src.path).exists(),
            "missing {}",
            src.path
        );
    }
}

#[test]
fn every_item_ref_has_an_embedded_icon() {
    use wow_buddy::assets::registry::{AssetRegistry, ICON_FALLBACK};
    let reg = AssetRegistry::load_embedded();
    let d = FishingData::load_embedded();
    let mut refs: Vec<&ItemRef> = d.shop.iter().collect();
    refs.extend(d.tiers.iter().flat_map(|tier| tier.lures.iter()));
    if let Some(q) = &d.artisan_quest {
        refs.extend(q.fish.iter());
    }
    assert!(refs.len() >= 13, "item refs: {}", refs.len());
    for item in refs {
        assert_ne!(item.item_id, 0);
        assert!(!item.name.is_empty());
        assert_ne!(
            reg.item_icon(&item.icon_id),
            ICON_FALLBACK,
            "{}",
            item.icon_id
        );
    }
}

#[test]
fn guide_texts_are_trilingual_and_clean() {
    let d = FishingData::load_embedded();
    let mut texts: Vec<&Tr> = d.basics.iter().map(|b| &b.text).collect();
    texts.extend(d.tiers.iter().map(|tier| &tier.learn));
    texts.extend(d.tiers.iter().filter_map(|tier| tier.tips.as_ref()));
    if let Some(q) = &d.artisan_quest {
        texts.push(&q.text);
    }
    for text in texts {
        for lang in [Language::En, Language::Ru, Language::Fi] {
            assert!(!text.get(lang).trim().is_empty());
        }
        assert!(
            !text
                .ru
                .chars()
                .any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c)),
            "CJK in RU"
        );
        assert!(
            !text
                .en
                .chars()
                .any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)),
            "Cyrillic in EN"
        );
        assert!(
            !text
                .fi
                .chars()
                .any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)),
            "Cyrillic in FI"
        );
    }
}

#[test]
fn the_guide_keeps_the_two_key_facts() {
    let d = FishingData::load_embedded();
    assert!(d
        .provenance
        .as_deref()
        .unwrap_or("")
        .contains("wow-professions"));
    assert!(
        d.basics
            .iter()
            .any(|b| b.text.en.to_lowercase().contains("hammer")),
        "the hammer-button gotcha must ship"
    );
}
