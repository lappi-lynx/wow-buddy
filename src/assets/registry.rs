use std::collections::HashMap;

use serde::Deserialize;

use super::generated::{ARTS, ICONS};

const MANIFEST_JSON: &str = include_str!("../../assets/manifest.json");

/// Glyph shown when an icon id has no embedded asset (never a broken image).
pub const ICON_FALLBACK: &str = "⚔️";

/// One declared asset: its source file, kind, and attribution.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AssetEntry {
    pub path: String,
    pub kind: String,
    pub source: String,
    #[serde(default)]
    pub license: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct Manifest {
    #[serde(default)]
    assets: HashMap<String, AssetEntry>,
}

/// Embedded registry of declared assets plus their generated `data:` URIs.
#[derive(Debug, Clone)]
pub struct AssetRegistry {
    entries: HashMap<String, AssetEntry>,
}

impl AssetRegistry {
    pub fn load_embedded() -> Self {
        let manifest: Manifest =
            serde_json::from_str(MANIFEST_JSON).expect("Embedded manifest.json must be valid");
        Self {
            entries: manifest.assets,
        }
    }

    pub fn has(&self, icon_id: &str) -> bool {
        self.entries.contains_key(icon_id)
    }

    /// The declared manifest entry for `id`, if any.
    pub fn entry(&self, id: &str) -> Option<&AssetEntry> {
        self.entries.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &AssetEntry)> {
        self.entries.iter()
    }

    /// An embedded `data:` URI for the icon, or the glyph fallback when the id
    /// is unknown. Gated on the manifest (the single source of truth) so a
    /// declared-but-ungenerated asset can never silently render the fallback.
    /// Falls back to the arts pool, because some Forever items reuse a spell
    /// icon file (e.g. Bone Knuckles → `ability_whirlwind`). Never panics.
    pub fn item_icon(&self, icon_id: &str) -> &'static str {
        if !self.has(icon_id) {
            return ICON_FALLBACK;
        }
        ICONS
            .iter()
            .find(|(id, _)| *id == icon_id)
            .or_else(|| ARTS.iter().find(|(id, _)| *id == icon_id))
            .map(|(_, uri)| *uri)
            .unwrap_or(ICON_FALLBACK)
    }

    /// An embedded `data:` URI for an art asset, or `None` when it is neither
    /// declared nor generated. `None` is the normal state before the curator
    /// imports art — callers fall back to their emoji or label.
    pub fn art_uri(&self, id: &str) -> Option<&'static str> {
        if !self.has(id) {
            return None;
        }
        ARTS
            .iter()
            .find(|(art_id, _)| *art_id == id)
            .map(|(_, uri)| *uri)
    }
}
