use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::components::reference_popover::{item_popover, npc_popover};
use wow_buddy::i18n::{I18nService, Language};
use wow_buddy::models::reference::NpcBook;

#[test]
fn test_npc_popover_resolves_known_npc() {
    let book = NpcBook::load_embedded();
    let i18n = I18nService::load_embedded();
    let popover = npc_popover(&book, &i18n, Language::En, "marshal_mcbride")
        .expect("marshal_mcbride must resolve");
    assert_eq!(popover.heading, "Marshal McBride");
    assert!(
        popover.lines.iter().any(|l| l.contains("Elwynn Forest")),
        "popover must show the zone: {:?}",
        popover.lines
    );
    assert!(
        popover.lines.iter().any(|l| l.contains("Northshire Abbey")),
        "popover must show the subzone: {:?}",
        popover.lines
    );
}

#[test]
fn test_npc_popover_unknown_npc_is_none() {
    let book = NpcBook::load_embedded();
    let i18n = I18nService::load_embedded();
    assert!(npc_popover(&book, &i18n, Language::En, "does_not_exist").is_none());
}

#[test]
fn test_item_popover_with_icon_resolves_data_uri() {
    let reg = AssetRegistry::load_embedded();
    let i18n = I18nService::load_embedded();
    let popover = item_popover(
        &reg,
        &i18n,
        Language::En,
        "Smite's Mighty Hammer",
        Some("inv_hammer_01"),
        "Mr. Smite, The Deadmines (Westfall)",
    );
    assert_eq!(popover.heading, "Smite's Mighty Hammer");
    let icon = popover.icon.expect("icon must resolve");
    assert!(icon.starts_with("data:"), "icon must be embedded: {icon}");
    assert!(popover.lines.iter().any(|l| l.contains("Mr. Smite")));
}

#[test]
fn test_item_popover_without_icon_still_opens() {
    let reg = AssetRegistry::load_embedded();
    let i18n = I18nService::load_embedded();
    let popover = item_popover(
        &reg,
        &i18n,
        Language::En,
        "starter club",
        None,
        "First quest reward",
    );
    assert_eq!(popover.heading, "starter club");
    assert!(popover.icon.is_none(), "no icon for a generic starter item");
    assert!(
        popover.lines.iter().any(|l| l.contains("First quest reward")),
        "source must be shown: {:?}",
        popover.lines
    );
}
