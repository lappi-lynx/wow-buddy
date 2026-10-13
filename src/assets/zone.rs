use super::registry::AssetRegistry;

/// Optional zone artwork as a `data:` URI. `None` is the normal state before
/// the curator imports kit art — callers show the zone name as text instead.
pub fn zone_art_source(registry: &AssetRegistry, zone_id: &str) -> Option<String> {
    registry
        .art_uri(&format!("zone_{zone_id}_art"))
        .map(str::to_string)
}
