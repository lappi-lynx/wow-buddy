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

    /// Playable classes of the race `race_id` for the given faction
    /// (delegates to `RaceInfo::classes_for_faction`).
    pub fn classes_for_faction(&self, faction: Faction, race_id: &str) -> &[String] {
        self.races
            .iter()
            .find(|r| r.id == race_id)
            .map(|r| r.classes_for_faction(faction))
            .unwrap_or(&[])
    }

    /// Whether `class_id` is playable by the race `race_id` in the given
    /// faction (faction-aware for dual-faction races such as Skyborne).
    pub fn is_class_available_for_race(
        &self,
        faction: Faction,
        race_id: &str,
        class_id: &str,
    ) -> bool {
        self.races
            .iter()
            .find(|r| r.id == race_id)
            .map(|r| r.classes_for_faction(faction).iter().any(|c| c == class_id))
            .unwrap_or(false)
    }

    /// Full selection validation: the race must be available in the faction
    /// and the class must be playable by that race in that faction.
    pub fn validate_selection(&self, faction: Faction, race_id: &str, class_id: &str) -> bool {
        self.races
            .iter()
            .find(|r| r.id == race_id)
            .map(|r| {
                r.is_available_for(faction)
                    && r.classes_for_faction(faction).iter().any(|c| c == class_id)
            })
            .unwrap_or(false)
    }

    /// First race native to the given faction (fallback: first race in the
    /// dataset). Dual-faction races such as Skyborne are never the default —
    /// the canonical faction starter (Human / Orc) is.
    pub fn default_race_for_faction(&self, faction: Faction) -> RaceInfo {
        self.races
            .iter()
            .find(|r| r.faction == Some(faction))
            .cloned()
            .unwrap_or_else(|| self.races[0].clone())
    }

    /// First playable class of the race `race_id` in the given faction,
    /// resolved to `ClassInfo`.
    pub fn default_class_for_race(
        &self,
        faction: Faction,
        race_id: &str,
    ) -> Option<ClassInfo> {
        let class_id = self
            .races
            .iter()
            .find(|r| r.id == race_id)?
            .classes_for_faction(faction)
            .first()?;
        self.get_class_by_id(class_id).cloned()
    }

    pub fn get_starter_zone(&self, id: &str) -> Option<&StarterZoneInfo> {
        self.starter_zones.iter().find(|z| z.id == id)
    }

    pub fn get_class_by_id(&self, id: &str) -> Option<&ClassInfo> {
        self.classes.iter().find(|c| c.id == id)
    }
}
