use crate::assets::AssetRegistry;
use crate::components::FactionTheme;
use crate::models::{Faction, RaceInfo};
use dioxus::prelude::*;

/// The emoji glyph for a race — the fallback when no race render art exists.
pub fn race_emoji(race_id: &str) -> &'static str {
    match race_id {
        "human" => "🛡️",
        "dwarf" => "⛏️",
        "night_elf" => "🌙",
        "gnome" => "⚙️",
        "skyborne" => "🪶",
        "orc" => "🪓",
        "undead" => "💀",
        "tauren" => "🐂",
        "troll" => "🏹",
        _ => "👤",
    }
}

/// Race render art as a `data:` URI, or the race emoji when no art exists.
/// Skyborne honours the faction variant id (High Order / Windshaper); the Fan
/// Site Kit has no Skyborne art, so it falls back to the emoji today.
pub fn race_render_source(registry: &AssetRegistry, race_id: &str, faction: Faction) -> String {
    let id = if race_id == "skyborne" {
        match faction {
            Faction::Alliance => "race_skyborne_alliance_render".to_string(),
            Faction::Horde => "race_skyborne_horde_render".to_string(),
        }
    } else {
        format!("race_{race_id}_render")
    };
    registry
        .art_uri(&id)
        .map(str::to_string)
        .unwrap_or_else(|| race_emoji(race_id).to_string())
}

/// Class render art as a `data:` URI, or an empty string when absent (callers
/// keep their text label).
pub fn class_render_source(registry: &AssetRegistry, class_id: &str) -> String {
    registry
        .art_uri(&format!("class_{class_id}_render"))
        .unwrap_or("")
        .to_string()
}

#[component]
pub fn RaceTile(
    race: RaceInfo,
    faction: Faction,
    is_selected: bool,
    display_name: String,
    on_select: EventHandler<RaceInfo>,
) -> Element {
    let race_clone = race.clone();

    let theme = FactionTheme::for_faction(faction);
    let border_style = if is_selected {
        theme.tile_selected
    } else {
        theme.tile_idle
    };

    let art = race_render_source(&AssetRegistry::load_embedded(), &race.id, faction);
    let has_art = art.starts_with("data:");
    let emoji = race_emoji(&race.id);

    rsx! {
        button {
            class: "flex flex-col items-center justify-center p-4 rounded-xl border-2 transition-all duration-200 cursor-pointer min-w-[120px] max-w-[160px] text-center {border_style}",
            onclick: move |_| on_select.call(race_clone.clone()),
            if has_art {
                img {
                    class: "w-12 h-12 object-contain mb-2",
                    src: "{art}",
                    alt: "race render",
                }
            } else {
                span { class: "text-3xl sm:text-4xl mb-2", "{emoji}" }
            }
            span { class: "text-xs sm:text-sm font-bold text-slate-100 leading-tight",
                "{display_name}"
            }
        }
    }
}
