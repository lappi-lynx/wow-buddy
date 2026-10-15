use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;

const READINESS_JSON: &str = include_str!("../../assets/data/readiness.json");

/// One checklist item a player ticks before entering a dungeon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessItem {
    pub id: String,
    pub text: Tr,
}

/// The "how do I find a group?" card content. `verified` stays false until
/// the KB confirms how grouping works in 1.60.1; the UI then shows the
/// "Beta, may change" tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupFinding {
    pub text: Tr,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessData {
    pub threshold_almost: u8,
    pub items: Vec<ReadinessItem>,
    pub group_finding: GroupFinding,
}

impl ReadinessData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(READINESS_JSON).expect("Embedded readiness.json must be valid")
    }
}

/// Not ready / almost / ready from the checked-item count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadinessLevel {
    NotReady,
    Almost,
    Ready,
}

pub fn readiness_level(checked: u8, total: u8, threshold_almost: u8) -> ReadinessLevel {
    if checked >= total {
        ReadinessLevel::Ready
    } else if checked >= threshold_almost {
        ReadinessLevel::Almost
    } else {
        ReadinessLevel::NotReady
    }
}
