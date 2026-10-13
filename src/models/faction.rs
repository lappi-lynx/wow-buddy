use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Faction {
    Alliance,
    Horde,
}

impl Faction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Faction::Alliance => "alliance",
            Faction::Horde => "horde",
        }
    }

    /// Stable human-readable label used by hand-off strings and UI. Kept
    /// explicit so the export contract never depends on `Debug` output.
    pub fn display_name(&self) -> &'static str {
        match self {
            Faction::Alliance => "Alliance",
            Faction::Horde => "Horde",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactionMeta {
    pub faction: Faction,
    pub name_key: String,
    pub icon: String,
    pub color_hex: String,
    pub motto_key: String,
}

pub fn get_faction_meta(faction: Faction) -> FactionMeta {
    match faction {
        Faction::Alliance => FactionMeta {
            faction: Faction::Alliance,
            name_key: "alliance_name".to_string(),
            icon: "🦁".to_string(),
            color_hex: "#2563eb".to_string(),
            motto_key: "alliance_motto".to_string(),
        },
        Faction::Horde => FactionMeta {
            faction: Faction::Horde,
            name_key: "horde_name".to_string(),
            icon: "⚡".to_string(),
            color_hex: "#dc2626".to_string(),
            motto_key: "horde_motto".to_string(),
        },
    }
}
