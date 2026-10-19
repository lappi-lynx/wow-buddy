use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;
use crate::models::pets::SourceRef;

const FISHING_JSON: &str = include_str!("../../assets/data/fishing.json");

/// One step of the fishing & cooking card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FishingStep {
    pub id: String,
    pub text: Tr,
}

/// One purchasable the guide references (its icon is embedded via the manifest).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemRef {
    pub name: String,
    pub item_id: u32,
    pub icon_id: String,
}

/// One numbered "how to fish" step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuideStep {
    pub id: String,
    pub text: Tr,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source_id: String,
}

/// A capital-city fishing trainer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapitalTrainer {
    pub faction: String,
    pub city: String,
    pub trainer: String,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source_id: String,
}

/// A starting-zone fishing trainer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartingZoneTrainer {
    pub races: String,
    pub zone: String,
    pub trainer: String,
    pub where_hint: String,
    #[serde(default)]
    pub faction: Option<String>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trainers {
    #[serde(default)]
    pub capitals: Vec<CapitalTrainer>,
    #[serde(default)]
    pub starting_zones: Vec<StartingZoneTrainer>,
}

/// One skill range of the leveling route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillTier {
    pub id: String,
    pub min: u16,
    pub max: u16,
    pub learn: Tr,
    #[serde(default)]
    pub zones: Vec<String>,
    #[serde(default)]
    pub tips: Option<Tr>,
    #[serde(default)]
    pub lures: Vec<ItemRef>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source_id: String,
}

/// The Nat Pagle artisan-fishing quest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtisanQuest {
    pub npc: String,
    pub quest_id: u32,
    pub min_level: u8,
    pub min_skill: u16,
    pub text: Tr,
    #[serde(default)]
    pub fish: Vec<ItemRef>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FishingData {
    pub title: Tr,
    pub steps: Vec<FishingStep>,
    pub tie_in: Tr,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub provenance: Option<String>,
    /// Provenance artifacts for the guide sections (ids used by `source_id`).
    #[serde(default)]
    pub sources: BTreeMap<String, SourceRef>,
    /// What to buy before the first cast (pole, lures, the Expert book).
    #[serde(default)]
    pub shop: Vec<ItemRef>,
    #[serde(default)]
    pub basics: Vec<GuideStep>,
    #[serde(default)]
    pub trainers: Option<Trainers>,
    #[serde(default)]
    pub tiers: Vec<SkillTier>,
    #[serde(default)]
    pub artisan_quest: Option<ArtisanQuest>,
}

impl FishingData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(FISHING_JSON).expect("Embedded fishing.json must be valid")
    }
}
