use super::faction::Faction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RaceInfo {
    pub id: String,
    pub name: String,
    pub faction: Option<Faction>,
    pub is_dual_faction: bool,
    pub alliance_variant_name: Option<String>,
    pub horde_variant_name: Option<String>,
    pub lore_ru: String,
    pub lore_en: String,
    pub lore_fi: String,
    pub racial_ability_name: String,
    pub racial_ability_desc: String,
    pub available_classes: Vec<String>,
    #[serde(default)]
    pub alliance_available_classes: Option<Vec<String>>,
    #[serde(default)]
    pub horde_available_classes: Option<Vec<String>>,
    pub starter_zone_id: String,
    pub model_id: String,
}

impl RaceInfo {
    pub fn is_available_for(&self, faction: Faction) -> bool {
        if self.is_dual_faction {
            true
        } else {
            self.faction == Some(faction)
        }
    }

    /// Playable classes for the given faction. Dual-faction races (e.g. Skyborne)
    /// carry faction-specific lists — High Order (Alliance) and Windshaper (Horde) —
    /// because some classes are exclusive to one variant; all other races use the base list.
    pub fn classes_for_faction(&self, faction: Faction) -> &[String] {
        match faction {
            Faction::Alliance => self.alliance_available_classes.as_deref(),
            Faction::Horde => self.horde_available_classes.as_deref(),
        }
        .unwrap_or(&self.available_classes)
    }
}
