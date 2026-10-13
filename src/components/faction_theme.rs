use crate::models::Faction;

/// Static Tailwind class bundles per faction.
///
/// Every bundle is a fully static string literal: `assets/app.css` is
/// precompiled, so classes assembled at runtime through `format!` would
/// never appear in production builds.
pub struct FactionTheme {
    pub accent_text: &'static str,
    pub accent_border: &'static str,
    pub accent_heading: &'static str,
    pub tile_selected: &'static str,
    pub tile_idle: &'static str,
}

impl FactionTheme {
    pub fn for_faction(faction: Faction) -> Self {
        match faction {
            Faction::Alliance => Self {
                accent_text: "text-blue-400",
                accent_border: "border-blue-500/40",
                accent_heading: "text-blue-400 border-blue-500/40",
                tile_selected: "border-amber-400 bg-blue-950/80 shadow-[0_0_20px_rgba(251,191,36,0.5)] scale-105",
                tile_idle: "border-slate-800 bg-slate-900/60 hover:border-slate-600 hover:bg-slate-800/80",
            },
            Faction::Horde => Self {
                accent_text: "text-red-400",
                accent_border: "border-red-500/40",
                accent_heading: "text-red-400 border-red-500/40",
                tile_selected: "border-red-500 bg-red-950/80 shadow-[0_0_20px_rgba(239,68,68,0.5)] scale-105",
                tile_idle: "border-slate-800 bg-slate-900/60 hover:border-slate-600 hover:bg-slate-800/80",
            },
        }
    }
}
