use wow_buddy::models::dungeon::get_dungeon_codex;
use wow_buddy::views::dungeon_codex::get_active_dungeons;

#[test]
fn test_dungeon_codex_rules_present() {
    let codex = get_dungeon_codex("deadmines");
    assert_eq!(codex.estimated_time, "45");
    assert!(codex.rules.iter().any(|r| r.id == "only_tank_pulls"));
    assert!(codex.rules.iter().any(|r| r.id == "loot_need_vs_greed"));
}

#[test]
fn test_active_dungeon_selection_list() {
    let dungeons = get_active_dungeons();
    assert_eq!(dungeons.len(), 5);
    for id in &dungeons {
        assert_eq!(get_dungeon_codex(id).id, *id, "id {id} must resolve to itself");
    }
}

#[test]
fn test_every_dungeon_has_complete_codex_data() {
    for id in get_active_dungeons() {
        let codex = get_dungeon_codex(id);
        assert!(!codex.name.is_empty(), "{id} missing name");
        assert!(!codex.icon.is_empty(), "{id} missing icon");
        assert!(!codex.level_range.is_empty(), "{id} missing level range");
        assert!(!codex.estimated_time.is_empty(), "{id} missing time");
        assert!(codex.bag_slots_required >= 1, "{id} invalid bag slots");
        assert!(codex.rules.len() >= 4, "{id} should have at least 4 rules");
        let mut rule_ids: Vec<String> = codex.rules.iter().map(|r| r.id.clone()).collect();
        let total = rule_ids.len();
        rule_ids.sort();
        rule_ids.dedup();
        assert_eq!(rule_ids.len(), total, "{id} has duplicate rule ids");
        for rule in &codex.rules {
            assert!(!rule.title.is_empty(), "{id}/{} missing title", rule.id);
            assert!(!rule.description.is_empty(), "{id}/{} missing description", rule.id);
            assert!(!rule.icon.is_empty(), "{id}/{} missing icon", rule.id);
        }
    }
}
