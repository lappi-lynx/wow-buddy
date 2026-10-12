use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DungeonRule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DungeonCodexInfo {
    pub id: String,
    pub name: String,
    pub level_range: String,
    pub estimated_time_minutes: u32,
    pub bag_slots_required: u8,
    pub rules: Vec<DungeonRule>,
}

pub fn get_dungeon_codex(dungeon_id: &str) -> DungeonCodexInfo {
    let deadmines_rules = vec![
        DungeonRule {
            id: "only_tank_pulls".to_string(),
            title: "Only Tank Pulls".to_string(),
            description: "Wait for tank to establish aggro on pack before attacking.".to_string(),
            icon: "🛡️".to_string(),
        },
        DungeonRule {
            id: "healer_mana".to_string(),
            title: "Watch Healer Mana".to_string(),
            description: "Wait for healer to drink if mana falls below 30%.".to_string(),
            icon: "💧".to_string(),
        },
        DungeonRule {
            id: "loot_need_vs_greed".to_string(),
            title: "Loot Etiquette: Need vs Greed".to_string(),
            description: "Greed by default; Need only if class appropriate and equipping immediately.".to_string(),
            icon: "🎲".to_string(),
        },
        DungeonRule {
            id: "clear_bags".to_string(),
            title: "Clear 10 Bag Slots".to_string(),
            description: "Free at least 10 bag slots before entering; no vendors inside.".to_string(),
            icon: "🎒".to_string(),
        },
    ];

    match dungeon_id {
        "deadmines" => DungeonCodexInfo {
            id: "deadmines".to_string(),
            name: "The Deadmines".to_string(),
            level_range: "17-21".to_string(),
            estimated_time_minutes: 45,
            bag_slots_required: 10,
            rules: deadmines_rules,
        },
        _ => DungeonCodexInfo {
            id: "ragefire_chasm".to_string(),
            name: "Ragefire Chasm".to_string(),
            level_range: "13-18".to_string(),
            estimated_time_minutes: 35,
            bag_slots_required: 10,
            rules: deadmines_rules,
        },
    }
}
