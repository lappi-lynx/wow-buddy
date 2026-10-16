use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::models::class_tips::LocalizedText;
use crate::models::pets::SourceRef;

const DUNGEONS_JSON: &str = include_str!("../../assets/data/dungeons.json");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DungeonRule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}

/// The recommended average group level, with its verification state. When no
/// source states a group level, the fallback rule is recorded in the artifact
/// and `verified` stays false (shipped with the Beta marker).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedAdvice {
    pub text: LocalizedText,
    #[serde(default)]
    pub verified: bool,
    pub source_id: String,
}

/// One "who lives here" entry: a mob type or a notable named elite/boss.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobEntry {
    pub name: String,
    pub kind: LocalizedText,
    #[serde(default)]
    pub note: Option<LocalizedText>,
}

/// One per-dungeon warning ("what to avoid").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchOut {
    pub text: LocalizedText,
    #[serde(default)]
    pub verified: bool,
}

/// Prep guidance: consumables and the buffs to have covered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepInfo {
    #[serde(default)]
    pub potions: Option<LocalizedText>,
    #[serde(default)]
    pub buffs: Vec<LocalizedText>,
}

/// One blue drop: item id + icon (both verified) and where it comes from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LootEntry {
    pub item_id: u32,
    pub icon_id: String,
    pub slot: String,
    pub source: String,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DungeonCodexInfo {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub level_range: String,
    /// Numeric bounds for level-aware filtering (mirror `level_range`).
    #[serde(default)]
    pub min_level: u8,
    #[serde(default)]
    pub max_level: u8,
    /// KB-verified clear-time estimate, kept as a range string (e.g. "60–90")
    /// so no single-minute precision is invented.
    pub estimated_time: String,
    pub bag_slots_required: u8,
    #[serde(default)]
    pub recommended_avg_level: Option<LocalizedAdvice>,
    #[serde(default)]
    pub mobs: Vec<MobEntry>,
    #[serde(default)]
    pub watch_out: Vec<WatchOut>,
    #[serde(default)]
    pub prep: Option<PrepInfo>,
    #[serde(default)]
    pub loot: Vec<LootEntry>,
    #[serde(default)]
    pub sources: BTreeMap<String, SourceRef>,
    pub rules: Vec<DungeonRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DungeonsData {
    pub dungeons: Vec<DungeonCodexInfo>,
}

impl DungeonsData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(DUNGEONS_JSON).expect("Embedded dungeons.json must be valid")
    }
}

/// The codex dungeon ids in display order (single source for level queries).
pub const DUNGEON_IDS: [&str; 5] = [
    "ragefire_chasm",
    "hall_of_thanes",
    "wailing_caverns",
    "deadmines",
    "shadowfang_keep",
];

/// One dungeon's codex entry. Unknown ids fall back to the first dungeon in
/// display order (the view only ever passes ids from `DUNGEON_IDS`).
pub fn get_dungeon_codex(dungeon_id: &str) -> DungeonCodexInfo {
    let data = DungeonsData::load_embedded();
    data.dungeons
        .iter()
        .find(|d| d.id == dungeon_id)
        .or_else(|| data.dungeons.first())
        .cloned()
        .expect("dungeons.json must contain at least one dungeon")
}

pub fn get_all_dungeons() -> Vec<DungeonCodexInfo> {
    let data = DungeonsData::load_embedded();
    DUNGEON_IDS
        .iter()
        .filter_map(|id| data.dungeons.iter().find(|d| d.id == *id).cloned())
        .collect()
}

/// The dungeons whose level range contains `level`.
pub fn dungeons_for_level(level: u8) -> Vec<DungeonCodexInfo> {
    get_all_dungeons()
        .into_iter()
        .filter(|d| level >= d.min_level && level <= d.max_level)
        .collect()
}
