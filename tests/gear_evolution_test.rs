use wow_buddy::models::gear::{get_tier_for_level, GearTier};
use wow_buddy::views::dressing_room::{get_tier_label_key, CharacterAnimation};

#[test]
fn test_gear_tier_milestones() {
    assert_eq!(get_tier_for_level(1), GearTier::Starter);
    assert_eq!(get_tier_for_level(20), GearTier::Dungeon);
    assert_eq!(get_tier_for_level(40), GearTier::ElitePlate);
    assert_eq!(get_tier_for_level(60), GearTier::RaidTier2);
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
