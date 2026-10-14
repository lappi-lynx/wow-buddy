use wow_buddy::models::gear::{get_tier_for_level, nearest_anchor, tier_label, GearTier};
use wow_buddy::views::dressing_room::{get_tier_label_key, CharacterAnimation};

#[test]
fn test_gear_tier_milestones() {
    assert_eq!(get_tier_for_level(1), GearTier::Starter);
    assert_eq!(get_tier_for_level(20), GearTier::Dungeon);
    assert_eq!(get_tier_for_level(40), GearTier::ElitePlate);
    assert_eq!(get_tier_for_level(60), GearTier::RaidTier2);
}

#[test]
fn snaps_to_nearest_anchor_ties_go_up() {
    for (level, want) in [
        (1, 1), (10, 1), (11, 20), (29, 20), (30, 40), (38, 40), (50, 60), (60, 60), (99, 60),
    ] {
        assert_eq!(nearest_anchor(level), want, "level {level}");
    }
}

#[test]
fn label_explains_snapping() {
    assert_eq!(tier_label(38, 40), "Level 38 (Level 40 set)");
    assert_eq!(tier_label(40, 40), "Level 40");
}

#[test]
fn snapping_drives_the_aura_too() {
    // aura and label share one boundary set now
    assert_eq!(get_tier_for_level(15), GearTier::Dungeon); // snaps to 20
    assert_eq!(get_tier_for_level(30), GearTier::ElitePlate); // snaps to 40
    assert_eq!(get_tier_for_level(45), GearTier::ElitePlate);
}

#[test]
fn test_gear_tier_label_keys() {
    assert_eq!(get_tier_label_key(GearTier::Starter), "tier_starter");
    assert_eq!(get_tier_label_key(GearTier::Dungeon), "tier_dungeon");
    assert_eq!(get_tier_label_key(GearTier::ElitePlate), "tier_elite");
    assert_eq!(get_tier_label_key(GearTier::RaidTier2), "tier_raid");
}

#[test]
fn test_character_animations() {
    let anims = CharacterAnimation::all();
    assert_eq!(anims.len(), 4);
    assert_eq!(CharacterAnimation::Stand.as_str(), "stand");
    assert_eq!(CharacterAnimation::Attack.as_str(), "attack");
    assert_eq!(CharacterAnimation::Cast.as_str(), "cast");
    assert_eq!(CharacterAnimation::Dance.as_str(), "dance");
}
