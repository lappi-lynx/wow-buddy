use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearTier {
    Starter,
    Dungeon,
    ElitePlate,
    RaidTier2,
}

pub fn get_tier_for_level(level: u8) -> GearTier {
    match level {
        0..=10 => GearTier::Starter,
        11..=30 => GearTier::Dungeon,
        31..=50 => GearTier::ElitePlate,
        _ => GearTier::RaidTier2,
    }
}
