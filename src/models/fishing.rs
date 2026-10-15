use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;

const FISHING_JSON: &str = include_str!("../../assets/data/fishing.json");

/// One step of the fishing & cooking card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FishingStep {
    pub id: String,
    pub text: Tr,
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
}

impl FishingData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(FISHING_JSON).expect("Embedded fishing.json must be valid")
    }
}
