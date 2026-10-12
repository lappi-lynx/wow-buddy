use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterZoneInfo {
    pub id: String,
    pub name_ru: String,
    pub name_en: String,
    pub name_fi: String,
    pub level_range: String,
    pub starting_npc: String,
    pub trainer_reminder: String,
    pub bag_reminder: String,
    pub next_zone_direction: String,
}
