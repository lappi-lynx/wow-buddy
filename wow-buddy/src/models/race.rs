use super::faction::Faction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
