use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::models::legacy::get_starter_legacy_preset;
use wow_buddy::models::GameData;
use wow_buddy::views::progression_guide::{get_starter_zone_reminders, split_first_quest};

#[test]
fn test_legacy_points_total_is_16() {
    let preset = get_starter_legacy_preset();
    let total_points: u8 = preset.perks.iter().map(|p| p.points).sum();
    assert_eq!(total_points, 16);
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
