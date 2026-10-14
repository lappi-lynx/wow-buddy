use wow_buddy::models::reference::NpcBook;
use wow_buddy::models::GameData;

#[test]
fn test_every_starter_zone_npc_resolves() {
    let book = NpcBook::load_embedded();
    let data = GameData::load_embedded();
    let mut seen = 0;
    for zone in &data.starter_zones {
        let id = zone
            .starting_npc_id
            .as_deref()
            .unwrap_or_else(|| panic!("{} must declare its NPC id", zone.id));
        let npc = book
            .get(id)
            .unwrap_or_else(|| panic!("{id} missing from npcs.json"));
        assert!(
            !npc.zone.is_empty() && !npc.subzone.is_empty(),
            "{id} needs zone + subzone"
        );
        assert!(
            npc.provenance.as_deref().is_some_and(|p| p.contains("kb:")),
            "{id} needs KB provenance"
        );
        seen += 1;
    }
    assert!(seen >= 7, "expected at least 7 starter zones");
}

#[test]
fn test_unknown_npc_returns_none() {
    let book = NpcBook::load_embedded();
    assert!(book.get("does_not_exist").is_none());
}

#[test]
fn test_gornek_is_the_valley_of_trials_starter_npc() {
    let data = GameData::load_embedded();
    let zone = data
        .get_starter_zone("valley_of_trials")
        .expect("valley_of_trials exists");
    assert_eq!(zone.starting_npc, "Gornek");
    assert_eq!(zone.starting_npc_id.as_deref(), Some("gornek"));
}
