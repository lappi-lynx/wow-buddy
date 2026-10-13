use wow_buddy::components::FactionTheme;
use wow_buddy::models::Faction;

#[test]
fn test_alliance_theme_bundles() {
    let theme = FactionTheme::for_faction(Faction::Alliance);
    assert_eq!(theme.accent_text, "text-blue-400");
    assert_eq!(theme.accent_border, "border-blue-500/40");
    assert!(theme.tile_selected.contains("bg-blue-950/80"));
    assert!(theme.tile_idle.contains("hover:border-slate-600"));
}

#[test]
fn test_horde_theme_bundles() {
    let theme = FactionTheme::for_faction(Faction::Horde);
    assert_eq!(theme.accent_text, "text-red-400");
    assert_eq!(theme.accent_border, "border-red-500/40");
    assert!(theme.tile_selected.contains("bg-red-950/80"));
    assert!(theme.tile_idle.contains("hover:border-slate-600"));
}

#[test]
fn test_theme_bundles_are_fully_static() {
    for faction in [Faction::Alliance, Faction::Horde] {
        let t = FactionTheme::for_faction(faction);
        for bundle in [t.accent_text, t.accent_border, t.tile_selected, t.tile_idle] {
            assert!(!bundle.contains('{'), "dynamic placeholder in static theme bundle: {bundle}");
        }
    }
}
