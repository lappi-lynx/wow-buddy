use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::i18n::Language;

const CLASS_TIPS_JSON: &str = include_str!("../../assets/data/class_tips.json");

/// The canonical class → specialization matrix we expect to author tips for.
pub const EXPECTED_SPECS: &[(&str, &[&str])] = &[
    ("warrior", &["arms", "fury", "protection"]),
    ("paladin", &["holy", "protection", "retribution"]),
    ("hunter", &["beast_mastery", "marksmanship", "survival"]),
    ("rogue", &["assassination", "combat", "subtlety"]),
    ("priest", &["discipline", "holy", "shadow"]),
    ("shaman", &["elemental", "enhancement", "restoration"]),
    ("mage", &["arcane", "fire", "frost"]),
    ("warlock", &["affliction", "demonology", "destruction"]),
    ("druid", &["balance", "feral", "restoration"]),
];

/// A single tip (or the shared spec info) in all three shipped languages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedText {
    pub text_en: String,
    pub text_ru: String,
    pub text_fi: String,
    /// Internal: KB source + verification date. Optional; missing is reported.
    #[serde(default)]
    pub provenance: Option<String>,
}

impl LocalizedText {
    pub fn get(&self, lang: Language) -> &str {
        match lang {
            Language::En => &self.text_en,
            Language::Ru => &self.text_ru,
            Language::Fi => &self.text_fi,
        }
    }
}

/// Beginner tips for one specialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecTips {
    pub id: String,
    pub name_en: String,
    #[serde(default)]
    pub tips: Vec<LocalizedText>,
    #[serde(default)]
    pub provenance: Option<String>,
}

/// A class's authored specializations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassTips {
    #[serde(default)]
    pub specs: Vec<SpecTips>,
}

/// Embedded New Player Tips dataset: class → specs, plus the shared spec info.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassTipsData {
    #[serde(default)]
    pub spec_info: Option<LocalizedText>,
    #[serde(default)]
    pub classes: HashMap<String, ClassTips>,
}

impl ClassTipsData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(CLASS_TIPS_JSON).expect("Embedded class_tips.json must be valid")
    }

    /// The authored specs for a class; empty for an unknown/unfilled class.
    pub fn specs_for(&self, class_id: &str) -> &[SpecTips] {
        self.classes
            .get(class_id)
            .map(|c| c.specs.as_slice())
            .unwrap_or(&[])
    }

    /// Sorted `"<class>/<spec>"` ids from `EXPECTED_SPECS` with no authored tips.
    pub fn missing_coverage(&self) -> Vec<String> {
        let mut missing = Vec::new();
        for (class_id, specs) in EXPECTED_SPECS {
            let authored = self.specs_for(class_id);
            for spec in specs.iter() {
                if !authored.iter().any(|s| s.id == *spec) {
                    missing.push(format!("{class_id}/{spec}"));
                }
            }
        }
        missing.sort();
        missing
    }
}
