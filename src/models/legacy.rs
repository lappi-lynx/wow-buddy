use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyPerk {
    pub id: String,
    pub name: String,
    pub points: u8,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyPreset {
    pub name: String,
    pub total_points: u8,
    pub perks: Vec<LegacyPerk>,
}

pub fn get_starter_legacy_preset() -> LegacyPreset {
    LegacyPreset {
        name: "Starter Adventurer".to_string(),
        total_points: 16,
        perks: vec![
            LegacyPerk {
                id: "swift_stride".to_string(),
                name: "Swift Stride".to_string(),
                points: 6,
                description: "+10% out of combat run speed".to_string(),
                icon: "👟".to_string(),
            },
            LegacyPerk {
                id: "cozy_camp".to_string(),
                name: "Cozy Camp".to_string(),
                points: 5,
                description: "Extended campfire well-fed and health regeneration".to_string(),
                icon: "🏕️".to_string(),
            },
            LegacyPerk {
                id: "trophy_collector".to_string(),
                name: "Trophy Collector".to_string(),
                points: 5,
                description: "Increased cloth and bonus coin find on beasts and humanoids".to_string(),
                icon: "💰".to_string(),
            },
        ],
    }
}
