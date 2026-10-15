use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;

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

/// Resolve a preset into (perk, points) pairs, in allocation order.
pub fn resolve_preset(catalog: &LegacyCatalog, preset_id: &str) -> Vec<(LegacyPerkDef, u8)> {
    let Some(preset) = catalog.preset(preset_id) else {
        return Vec::new();
    };
    preset
        .allocation
        .iter()
        .filter_map(|a| catalog.perk(&a.id).map(|p| (p.clone(), a.points)))
        .collect()
}

/// The copyable allocation summary, e.g.
/// "Well Rested 5 · Thrill of Adventure 5 · Talented 5 · Field Guide 1 — 16 points".
/// Perk names are client terms and stay English in every locale.
pub fn preset_summary(catalog: &LegacyCatalog, preset_id: &str) -> String {
    let resolved = resolve_preset(catalog, preset_id);
    let parts: Vec<String> = resolved
        .iter()
        .map(|(p, n)| format!("{} {}", p.name.en, n))
        .collect();
    let total: u8 = resolved.iter().map(|(_, n)| n).sum();
    format!("{} — {total} points", parts.join(" · "))
}
