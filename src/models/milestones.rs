/// A leveling milestone the slider can teach ("what unlocks around here").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone {
    pub level: u8,
    pub key: &'static str,
    pub verified: bool,
}

pub const MILESTONES: &[Milestone] = &[
    Milestone { level: 2, key: "ms.trainer", verified: true },
    Milestone { level: 10, key: "ms.talents", verified: true },
    Milestone { level: 13, key: "ms.first_dungeon", verified: true },
    Milestone { level: 15, key: "ms.group_leveling", verified: true },
    Milestone { level: 25, key: "ms.legacy", verified: true },
    Milestone { level: 40, key: "ms.mount_dual_spec", verified: false },
    Milestone { level: 60, key: "ms.planning", verified: false },
];

/// The next milestone strictly above `level`, if any.
pub fn next_milestone(level: u8) -> Option<&'static Milestone> {
    MILESTONES.iter().find(|m| m.level > level)
}

/// Milestones within `window` levels of `level`.
pub fn milestones_around(level: u8, window: u8) -> Vec<&'static Milestone> {
    MILESTONES
        .iter()
        .filter(|m| (m.level as i16 - level as i16).abs() <= window as i16)
        .collect()
}
