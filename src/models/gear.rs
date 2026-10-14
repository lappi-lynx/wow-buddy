use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearTier {
    Starter,
    Dungeon,
    ElitePlate,
    RaidTier2,
}

/// Slider anchors — one boundary set drives both the aura and the snap label.
pub const TIER_ANCHORS: [u8; 4] = [1, 20, 40, 60];

/// Snap a level to the nearest anchor; ties go to the higher anchor.
pub fn nearest_anchor(level: u8) -> u8 {
    let l = level.clamp(1, 60) as i16;
    let mut best = TIER_ANCHORS[0];
    for a in TIER_ANCHORS {
        if (a as i16 - l).abs() <= (best as i16 - l).abs() {
            best = a;
        }
    }
    best
}

/// Pure English snap label (tests/aria): "Level 40" or "Level 38 (Level 40 set)".
pub fn tier_label(level: u8, anchor: u8) -> String {
    if level == anchor {
        format!("Level {level}")
    } else {
        format!("Level {level} (Level {anchor} set)")
    }
}

/// Short localized set-name i18n key for an anchor (used by the snap label).
pub fn tier_short_key(anchor: u8) -> &'static str {
    match anchor {
        20 => "tier_dungeon_short",
        40 => "tier_elite_short",
        60 => "tier_raid_short",
        _ => "tier_starter_short",
    }
}

/// Tier for a level — snapped to the anchors so the aura and the label agree.
pub fn get_tier_for_level(level: u8) -> GearTier {
    match nearest_anchor(level) {
        20 => GearTier::Dungeon,
        40 => GearTier::ElitePlate,
        60 => GearTier::RaidTier2,
        _ => GearTier::Starter,
    }
}
