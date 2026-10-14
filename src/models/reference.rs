use std::collections::HashMap;

use serde::Deserialize;

const NPCS_JSON: &str = include_str!("../../assets/data/npcs.json");

/// One curated NPC reference: where the NPC stands (zone/subzone).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NpcRef {
    pub name: String,
    pub zone: String,
    pub subzone: String,
    /// Internal: KB source + verification date (see `docs/kb-sources/`).
    #[serde(default)]
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct NpcBookFile {
    npcs: HashMap<String, NpcRef>,
}

/// Embedded, curated NPC locations: id → zone/subzone.
#[derive(Debug, Clone)]
pub struct NpcBook {
    npcs: HashMap<String, NpcRef>,
}

impl NpcBook {
    pub fn load_embedded() -> Self {
        let file: NpcBookFile =
            serde_json::from_str(NPCS_JSON).expect("Embedded npcs.json must be valid");
        Self { npcs: file.npcs }
    }

    /// The curated reference for `id`, or `None` when the NPC is unknown —
    /// callers keep rendering the plain name.
    pub fn get(&self, id: &str) -> Option<&NpcRef> {
        self.npcs.get(id)
    }
}
