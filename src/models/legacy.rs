use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;

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

const LEGACY_JSON: &str = include_str!("../../assets/data/legacy.json");

/// A specific perk prerequisite ("Perk X at N ranks") on top of the tree
/// requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerkPrereq {
    pub id: String,
    pub ranks: u8,
}

/// One perk from the client's 21-perk Legacy catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyPerkDef {
    pub id: String,
    pub tree: String,
    pub ranks: u8,
    #[serde(default)]
    pub prereq_tree_points: u8,
    #[serde(default)]
    pub prereq_perk: Option<PerkPrereq>,
    pub name: Tr,
    pub desc: Tr,
}

/// Points assigned to one perk inside a preset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation {
    pub id: String,
    pub points: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyPresetDef {
    pub id: String,
    pub name: Tr,
    pub allocation: Vec<Allocation>,
}

/// The full Legacy catalog: 21 perks in 3 trees + the shipped presets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyCatalog {
    pub perks: Vec<LegacyPerkDef>,
    pub presets: Vec<LegacyPresetDef>,
    #[serde(default)]
    pub source: Option<String>,
}

impl LegacyCatalog {
    pub const SEASONAL_CAP: u8 = 16;

    pub fn load_embedded() -> Self {
        serde_json::from_str(LEGACY_JSON).expect("Embedded legacy.json must be valid")
    }

    pub fn perk(&self, id: &str) -> Option<&LegacyPerkDef> {
        self.perks.iter().find(|p| p.id == id)
    }

    pub fn preset(&self, id: &str) -> Option<&LegacyPresetDef> {
        self.presets.iter().find(|p| p.id == id)
    }

    pub fn total_points(&self, preset: &LegacyPresetDef) -> u8 {
        preset.allocation.iter().map(|a| a.points).sum()
    }

    /// Validate an allocation against the client's prereq chains and the
    /// 16-point seasonal cap. The `prereq_perk` check looks at the whole
    /// allocation (in-game you may assign points in any order).
    pub fn validate_allocation(&self, alloc: &[Allocation]) -> Result<(), String> {
        let planned: std::collections::BTreeMap<&str, u8> =
            alloc.iter().map(|a| (a.id.as_str(), a.points)).collect();
        let mut spent: std::collections::BTreeMap<&str, u8> = std::collections::BTreeMap::new();
        let mut total: u8 = 0;
        for a in alloc {
            let perk = self
                .perk(&a.id)
                .ok_or_else(|| format!("unknown perk {}", a.id))?;
            if a.points > perk.ranks {
                return Err(format!(
                    "{}: {} points exceeds its {} ranks",
                    a.id, a.points, perk.ranks
                ));
            }
            if spent.get(perk.tree.as_str()).copied().unwrap_or(0) < perk.prereq_tree_points {
                return Err(format!(
                    "{} needs {} points in {} first",
                    a.id, perk.prereq_tree_points, perk.tree
                ));
            }
            if let Some(pp) = &perk.prereq_perk {
                let have = planned.get(pp.id.as_str()).copied().unwrap_or(0);
                if have < pp.ranks {
                    return Err(format!(
                        "{} needs {} {}/{}",
                        a.id, pp.id, pp.ranks, pp.ranks
                    ));
                }
            }
            total = total.saturating_add(a.points);
            if total > Self::SEASONAL_CAP {
                return Err(format!(
                    "{total} points exceeds the {} seasonal cap",
                    Self::SEASONAL_CAP
                ));
            }
            *spent.entry(perk.tree.as_str()).or_insert(0) += a.points;
        }
        Ok(())
    }
}
