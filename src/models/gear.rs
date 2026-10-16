use crate::models::milestones::MILESTONES;
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

/// i18n key of the full tier name for a tier — the single mapping used by the
/// viewer caption (and any other full-name rendering).
pub fn tier_label_key(tier: GearTier) -> &'static str {
    match tier {
        GearTier::Starter => "tier_starter",
        GearTier::Dungeon => "tier_dungeon",
        GearTier::ElitePlate => "tier_elite",
        GearTier::RaidTier2 => "tier_raid",
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

/// The kind of marking a slider dot carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DotKind {
    /// The recommended gear set changes at this level (amber dot).
    SetChange,
    /// A leveling milestone (blue dot).
    Milestone,
}

/// One progress dot on the 1–60 level slider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliderDot {
    pub level: u8,
    pub kind: DotKind,
}

/// Progress dots for the level slider: set-change dots from the given band
/// start levels plus the tier anchors at 40 and 60 (level 1 is the track start,
/// so it never gets a dot), and milestone dots for every milestone at level 10
/// or above. Sorted by level; ties keep `SetChange` first.
pub fn slider_dots(band_starts: &[u8]) -> Vec<SliderDot> {
    let mut dots: Vec<SliderDot> = Vec::new();
    for level in band_starts
        .iter()
        .copied()
        .chain(TIER_ANCHORS.iter().copied().filter(|a| *a >= 40))
    {
        if level <= 1 {
            continue;
        }
        if !dots
            .iter()
            .any(|d| d.kind == DotKind::SetChange && d.level == level)
        {
            dots.push(SliderDot {
                level,
                kind: DotKind::SetChange,
            });
        }
    }
    for milestone in MILESTONES.iter().filter(|m| m.level >= 10) {
        dots.push(SliderDot {
            level: milestone.level,
            kind: DotKind::Milestone,
        });
    }
    dots.sort_by_key(|d| {
        (
            d.level,
            if d.kind == DotKind::SetChange { 0 } else { 1 },
        )
    });
    dots
}
