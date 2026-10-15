use serde::{Deserialize, Serialize};

use crate::models::localized::Tr;

const COMFORT_JSON: &str = include_str!("../../assets/data/comfort_tips.json");

/// One one-idea comfort card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComfortTip {
    pub id: String,
    pub text: Tr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComfortTipsData {
    pub tips: Vec<ComfortTip>,
}

impl ComfortTipsData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(COMFORT_JSON).expect("Embedded comfort_tips.json must be valid")
    }
}
