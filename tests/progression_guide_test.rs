use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::models::legacy::{resolve_preset, LegacyCatalog};
use wow_buddy::models::GameData;
use wow_buddy::views::progression_guide::{get_starter_zone_reminders, split_first_quest};

#[test]
fn test_legacy_points_total_is_16() {
    let catalog = LegacyCatalog::load_embedded();
    for preset in &catalog.presets {
        assert_eq!(catalog.total_points(preset), 16, "{}", preset.id);
    }
}

#[test]
fn test_legacy_presets_use_verified_client_perks() {
    let catalog = LegacyCatalog::load_embedded();
    // Every perk referenced by any preset must exist in the client catalog
    // (docs/foreverdb-sources/legacy-system-explained.md, build 1.60.1.70205)
    // — an unknown id would mean invented content slipped back in.
    for preset in &catalog.presets {
        for a in &preset.allocation {
            assert!(
                catalog.perk(&a.id).is_some(),
                "unverified perk '{}' in {}",
                a.id,
                preset.id
            );
        }
        assert!(
            catalog.validate_allocation(&preset.allocation).is_ok(),
            "{}",
            preset.id
        );
    }
    let balanced: Vec<(String, u8)> = resolve_preset(&catalog, "balanced")
        .into_iter()
        .map(|(p, n)| (p.id, n))
        .collect();
    assert!(balanced.contains(&("well_rested".into(), 5)));
    assert!(balanced.contains(&("talented".into(), 5)));
}

#[test]
fn test_starter_zone_reminders() {
    let data = GameData::load_embedded();
    let zone = data.get_starter_zone("northshire").expect("Northshire exists");
    let reminders = get_starter_zone_reminders(zone);
    assert_eq!(reminders.len(), 2);
    assert!(reminders[0].contains("2 and 4"));
    assert!(reminders[1].contains("5 silver"));
}

#[test]
fn test_split_first_quest_places_the_npc() {
    assert_eq!(
        split_first_quest("Speak with {npc} now"),
        Some(("Speak with ".to_string(), " now".to_string()))
    );
    assert_eq!(
        split_first_quest("{npc}"),
        Some((String::new(), String::new()))
    );
    assert_eq!(split_first_quest("no placeholder"), None);
}

#[test]
fn test_all_locales_keep_the_npc_placeholder() {
    let i18n = I18nService::load_embedded();
    for lang in [Language::En, Language::Ru, Language::Fi] {
        assert!(
            i18n.t("ui_first_quest", lang).contains("{npc}"),
            "ui_first_quest lost its {{npc}} placeholder for {lang:?}"
        );
    }
}
