use wow_buddy::models::dungeon::get_dungeon_codex;
use wow_buddy::views::dungeon_codex::get_active_dungeons;

#[test]
fn test_dungeon_codex_rules_present() {
    let codex = get_dungeon_codex("deadmines");
    assert_eq!(codex.estimated_time_minutes, 45);
    assert!(codex.rules.iter().any(|r| r.id == "only_tank_pulls"));
    assert!(codex.rules.iter().any(|r| r.id == "loot_need_vs_greed"));
}

#[test]
fn test_active_dungeon_selection_list() {
    let dungeons = get_active_dungeons();
    assert_eq!(dungeons.len(), 2);
    assert_eq!(dungeons[0], "deadmines");
    assert_eq!(dungeons[1], "ragefire_chasm");
}
