use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::faction::Faction;

const GEAR_JSON: &str = include_str!("../../assets/data/gear.json");

/// A single recommended weapon within a level band.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearWeapon {
    pub name: String,
    pub level: u8,
    #[serde(default)]
    pub item_id: Option<u32>,
    #[serde(default)]
    pub icon_id: Option<String>,
    /// User-facing: where to get it ("Mr. Smite, The Deadmines").
    pub source: String,
    /// Internal: KB source + verification date. Optional; missing entries are reported.
    #[serde(default)]
    pub provenance: Option<String>,
}

/// The one-line armor hint for a band (lean: dungeon/zone to farm).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearArmorLine {
    pub text: String,
    pub source: String,
    #[serde(default)]
    pub provenance: Option<String>,
}

/// A level band (e.g. "12–16") grouping the weapon upgrades it contains.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearBand {
    pub id: String,
    pub label: String,
    pub min_level: u8,
    pub max_level: u8,
    #[serde(default)]
    pub weapons: Vec<GearWeapon>,
    pub armor: GearArmorLine,
}

/// A class's gear paths per faction.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGear {
    #[serde(default)]
    pub alliance: Vec<GearBand>,
    #[serde(default)]
    pub horde: Vec<GearBand>,
}

/// Embedded Gear Planner dataset: class → faction → bands.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearPlannerData {
    #[serde(default)]
    pub classes: HashMap<String, ClassGear>,
}

impl GearPlannerData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(GEAR_JSON).expect("Embedded gear.json must be valid")
    }

    /// The bands for a class + faction; empty for an unknown class (never panics).
    pub fn bands_for(&self, class_id: &str, faction: Faction) -> &[GearBand] {
        self.classes
            .get(class_id)
            .map(|c| match faction {
                Faction::Alliance => c.alliance.as_slice(),
                Faction::Horde => c.horde.as_slice(),
            })
            .unwrap_or(&[])
    }

    /// The band that contains `level`, if any (first match wins).
    pub fn band_for_level(&self, class_id: &str, faction: Faction, level: u8) -> Option<&GearBand> {
        self.bands_for(class_id, faction)
            .iter()
            .find(|b| level >= b.min_level && level <= b.max_level)
    }

    /// The next band above `level` (used when the level falls in a gap).
    pub fn next_band_for_level(
        &self,
        class_id: &str,
        faction: Faction,
        level: u8,
    ) -> Option<&GearBand> {
        self.bands_for(class_id, faction)
            .iter()
            .filter(|b| b.min_level > level)
            .min_by_key(|b| b.min_level)
    }

    /// Deterministic, sorted list of record identifiers that lack `provenance`.
    pub fn bands_missing_provenance(&self) -> Vec<String> {
        let mut missing = Vec::new();
        for (class_id, gear) in &self.classes {
            for (faction_key, bands) in
                [("alliance", &gear.alliance), ("horde", &gear.horde)]
            {
                for band in bands {
                    if band.armor.provenance.is_none() {
                        missing.push(format!("{class_id}/{faction_key}/{}/armor", band.id));
                    }
                    for w in &band.weapons {
                        if w.provenance.is_none() {
                            missing.push(format!(
                                "{class_id}/{faction_key}/{}/weapon:{}",
                                band.id, w.name
                            ));
                        }
                    }
                }
            }
        }
        missing.sort();
        missing
    }
}
