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
    pub icon: String,
    pub level_range: String,
    /// KB-verified clear-time estimate, kept as a range string (e.g. "60–90")
    /// so no single-minute precision is invented.
    pub estimated_time: String,
    pub bag_slots_required: u8,
    pub rules: Vec<DungeonRule>,
}

fn rule(id: &str, title: &str, description: &str, icon: &str) -> DungeonRule {
    DungeonRule {
        id: id.to_string(),
        title: title.to_string(),
        description: description.to_string(),
        icon: icon.to_string(),
    }
}

pub fn get_dungeon_codex(dungeon_id: &str) -> DungeonCodexInfo {
    let deadmines_rules = vec![
        rule(
            "only_tank_pulls",
            "Only Tank Pulls",
            "Wait for tank to establish aggro on pack before attacking.",
            "🛡️",
        ),
        rule(
            "healer_mana",
            "Watch Healer Mana",
            "Wait for healer to drink if mana falls below 30%.",
            "💧",
        ),
        rule(
            "loot_need_vs_greed",
            "Loot Etiquette: Need vs Greed",
            "Greed by default; Need only if class appropriate and equipping immediately.",
            "🎲",
        ),
        rule(
            "clear_bags",
            "Clear 10 Bag Slots",
            "Free at least 10 bag slots before entering; no vendors inside.",
            "🎒",
        ),
    ];

    match dungeon_id {
        "deadmines" => DungeonCodexInfo {
            id: "deadmines".to_string(),
            name: "The Deadmines".to_string(),
            icon: "☠️".to_string(),
            level_range: "17–21".to_string(),
            estimated_time: "45".to_string(),
            bag_slots_required: 10,
            rules: deadmines_rules,
        },
        "wailing_caverns" => DungeonCodexInfo {
            id: "wailing_caverns".to_string(),
            name: "Wailing Caverns".to_string(),
            icon: "🐍".to_string(),
            level_range: "15–22".to_string(),
            estimated_time: "60–90".to_string(),
            bag_slots_required: 10,
            rules: vec![
                rule(
                    "pull_back_to_cleared",
                    "Fight Runners in Cleared Ground",
                    "Pull raptors and humanoids back: they flee at low health and can drag sleeping or patrolling packs onto the group.",
                    "↩️",
                ),
                rule(
                    "interrupt_druids",
                    "Interrupt the Druids of the Fang",
                    "Priority-kick Sleep and Healing Touch, and bring poison cleansing for heavy nature DoTs.",
                    "🛑",
                ),
                rule(
                    "stack_on_escort",
                    "Stack for the Disciple Escort",
                    "During the final ritual escort, stay close to the Disciple and let the tank grab the ambush waves before AoE.",
                    "🛡️",
                ),
                rule(
                    "share_hybrid_drops",
                    "Discuss Hybrid Drops First",
                    "Agility leather, caster staves and shield drops are usable by many classes here — talk before rolling Need.",
                    "🎲",
                ),
            ],
        },
        "shadowfang_keep" => DungeonCodexInfo {
            id: "shadowfang_keep".to_string(),
            name: "Shadowfang Keep".to_string(),
            icon: "🐺".to_string(),
            level_range: "18–25".to_string(),
            estimated_time: "45–60".to_string(),
            bag_slots_required: 10,
            rules: vec![
                rule(
                    "los_pulls_stairs",
                    "Line-of-Sight Pulls on Stairs",
                    "Break line of sight around doorways and stairwells so ranged guards and fleeing worgen don't pull packs from above.",
                    "🧱",
                ),
                rule(
                    "interrupts_for_arugal",
                    "Save Interrupts for Arugal",
                    "Keep kicks and silences for Archmage Arugal's Voidbolt, and be ready when he mind-controls a party member.",
                    "🛑",
                ),
                rule(
                    "shadow_curse_cleanses",
                    "Bring Shadow and Curse Cleanses",
                    "Shadow protection potions plus curse and disease removal for worgen debuffs and boss curses.",
                    "🧪",
                ),
                rule(
                    "talk_drops_before_need",
                    "Talk Drops Before Rolling",
                    "Quest items (the Tome chain) and caster pieces like Meteor Shard or Robes of Arugal are in high demand — agree before Need.",
                    "🎲",
                ),
            ],
        },
        "hall_of_thanes" => DungeonCodexInfo {
            id: "hall_of_thanes".to_string(),
            name: "The Hall of Thanes".to_string(),
            icon: "⚒️".to_string(),
            level_range: "13–20".to_string(),
            estimated_time: "30–45".to_string(),
            bag_slots_required: 10,
            rules: vec![
                rule(
                    "watch_patrols",
                    "Watch the Dark Iron Patrols",
                    "Corridors are tight and patrolled — pull groups back into cleared rooms so adds don't chain.",
                    "👁️",
                ),
                rule(
                    "tank_threat_first",
                    "Let the Tank Build Threat",
                    "Fire elementals and golems hit hard with AoE; wait for threat from Demoralizing Shout or Thunder Clap before bursting.",
                    "🛡️",
                ),
                rule(
                    "fire_mitigation",
                    "Pack Fire Mitigation",
                    "Health potions and fire protection or dispels for elemental burn debuffs.",
                    "🔥",
                ),
                rule(
                    "share_early_upgrades",
                    "Share Early Upgrades",
                    "One of the earliest dungeons — check before needing loot that is a primary upgrade for someone else.",
                    "🎒",
                ),
            ],
        },
        _ => DungeonCodexInfo {
            id: "ragefire_chasm".to_string(),
            name: "Ragefire Chasm".to_string(),
            icon: "🔥".to_string(),
            level_range: "13–18".to_string(),
            estimated_time: "35".to_string(),
            bag_slots_required: 10,
            rules: deadmines_rules,
        },
    }
}
