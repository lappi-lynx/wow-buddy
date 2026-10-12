pub mod class;
pub mod dungeon;
pub mod faction;
pub mod gear;
pub mod legacy;
pub mod race;
pub mod zone;

pub use class::{ClassInfo, CoreSpell};
pub use dungeon::{get_dungeon_codex, DungeonCodexInfo, DungeonRule};
pub use faction::{get_faction_meta, Faction, FactionMeta};
pub use gear::{get_tier_for_level, GearTier};
pub use legacy::{get_starter_legacy_preset, LegacyPerk, LegacyPreset};
pub use race::RaceInfo;
pub use zone::StarterZoneInfo;

use serde::{Deserialize, Serialize};

const CONTENT_JSON: &str = include_str!("../../assets/data/content.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub races: Vec<RaceInfo>,
    pub starter_zones: Vec<StarterZoneInfo>,
    pub classes: Vec<ClassInfo>,
    pub dungeons: Vec<DungeonCodexInfo>,
}

impl GameData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(CONTENT_JSON).expect("Embedded content.json must be valid")
    }

    pub fn get_races_for_faction(&self, faction: Faction) -> Vec<&RaceInfo> {
        self.races
            .iter()
            .filter(|r| r.is_available_for(faction))
            .collect()
    }

    pub fn get_race_by_id(&self, id: &str) -> Option<&RaceInfo> {
        self.races.iter().find(|r| r.id == id)
    }

    pub fn get_starter_zone(&self, id: &str) -> Option<&StarterZoneInfo> {
        self.starter_zones.iter().find(|z| z.id == id)
    }

    pub fn get_class_by_id(&self, id: &str) -> Option<&ClassInfo> {
        self.classes.iter().find(|c| c.id == id)
    }
}
