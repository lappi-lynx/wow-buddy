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
    // Client-verified 16-point leveling build (Forever beta build 1.60.1.70205):
    // Well Rested 5 + Thrill of Adventure 5 + Talented 5 (prereq: Well Rested 5/5)
    // + Field Guide 1. Source: docs/foreverdb-sources/legacy-system-explained.md.
    LegacyPreset {
        name: "Leveling Starter".to_string(),
        total_points: 16,
        perks: vec![
            LegacyPerk {
                id: "well_rested".to_string(),
                name: "Well Rested".to_string(),
                points: 5,
                description: "Rested experience accumulates 20% faster and your rested cap is 20% higher.".to_string(),
                icon: "💤".to_string(),
            },
            LegacyPerk {
                id: "thrill_of_adventure".to_string(),
                name: "Thrill of Adventure".to_string(),
                points: 5,
                description: "Killing blows restore 5% of your maximum Health and Mana over 10 sec (not in dungeons, raids or battlegrounds).".to_string(),
                icon: "🩹".to_string(),
            },
            LegacyPerk {
                id: "talented".to_string(),
                name: "Talented".to_string(),
                points: 5,
                description: "You gain talent points from level 5 instead of 10 (still at most 51 total). Requires Well Rested 5/5.".to_string(),
                icon: "🎓".to_string(),
            },
            LegacyPerk {
                id: "field_guide".to_string(),
                name: "Field Guide".to_string(),
                points: 1,
                description: "Reduces your cooldown on adding Camp features by 25%.".to_string(),
                icon: "🏕️".to_string(),
            },
        ],
    }
}
