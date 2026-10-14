use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::models::class_tips::LocalizedText;

const PETS_JSON: &str = include_str!("../../assets/data/pets.json");

/// Where a dataset entry's facts come from; `path` must point at a real
/// provenance file under `docs/` (enforced by `tests/pets_test.rs`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub note: Option<String>,
}

/// Localized labels for the machine ids used in the dataset (diet ids like
/// `meat`, type ids like `ferocity`) — keeps English out of RU/FI views.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Legends {
    #[serde(default)]
    pub diets: BTreeMap<String, LocalizedText>,
    #[serde(default)]
    pub types: BTreeMap<String, LocalizedText>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PetFamily {
    pub id: String,
    pub name: String,
    pub pet_type: String,
    pub diet: Vec<String>,
    pub note: LocalizedText,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TameBasic {
    pub id: String,
    #[serde(default)]
    pub level_at: Option<u8>,
    pub title: LocalizedText,
    pub body: LocalizedText,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotablePet {
    pub id: String,
    pub name: String,
    pub family_id: String,
    pub min_level: u8,
    pub where_hint: LocalizedText,
    pub why: LocalizedText,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarlockMinion {
    pub id: String,
    pub name: String,
    pub learned_at: u8,
    pub role: LocalizedText,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionCard {
    pub title: LocalizedText,
    pub body: LocalizedText,
    pub source_id: String,
}

/// The pets dataset. All collections except `families` default so partial
/// pack documents parse (Track D replaces the whole document, v1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PetsData {
    #[serde(default)]
    pub sources: BTreeMap<String, SourceRef>,
    #[serde(default)]
    pub legends: Legends,
    pub families: Vec<PetFamily>,
    #[serde(default)]
    pub first_pick: Vec<String>,
    #[serde(default)]
    pub basics: Vec<TameBasic>,
    #[serde(default)]
    pub notable: Vec<NotablePet>,
    #[serde(default)]
    pub minions: Vec<WarlockMinion>,
    #[serde(default)]
    pub companion: Option<CompanionCard>,
}

impl PetsData {
    pub fn load_embedded() -> Self {
        serde_json::from_str(PETS_JSON).expect("Embedded pets.json must be valid")
    }
}

/// Which pets card the Dressing Room shows for a class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetCardKind {
    Hunter,
    Warlock,
    Companion,
}

/// `""` (no hero selected — deep link) falls back to the companion view.
pub fn pet_card_kind(class_id: &str) -> PetCardKind {
    match class_id {
        "hunter" => PetCardKind::Hunter,
        "warlock" => PetCardKind::Warlock,
        _ => PetCardKind::Companion,
    }
}

/// The sections the `/pets` view renders, in order, for a card kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetsSection {
    Basics,
    Families,
    Notable,
    Minions,
    Companion,
}

pub fn pets_sections(kind: PetCardKind) -> &'static [PetsSection] {
    match kind {
        PetCardKind::Hunter => &[
            PetsSection::Basics,
            PetsSection::Families,
            PetsSection::Notable,
            PetsSection::Companion,
        ],
        PetCardKind::Warlock => &[PetsSection::Minions, PetsSection::Companion],
        PetCardKind::Companion => &[PetsSection::Companion],
    }
}
