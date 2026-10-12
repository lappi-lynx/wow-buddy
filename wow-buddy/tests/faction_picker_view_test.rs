use wow_buddy::models::Faction;
use wow_buddy::views::faction_picker::get_faction_meta;

#[test]
fn test_faction_meta_shields() {
    let alliance = get_faction_meta(Faction::Alliance);
    assert_eq!(alliance.icon, "🦁");
    let horde = get_faction_meta(Faction::Horde);
    assert_eq!(horde.icon, "⚡");
}
