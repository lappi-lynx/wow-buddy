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

/// Male/female race portraits as embedded `data:` URIs, or `None` when the
/// race has no art (e.g. the custom Skyborne) — callers fall back to the
/// race emoji.
pub fn race_portraits(registry: &AssetRegistry, race_id: &str) -> Option<(String, String)> {
    let male = registry.art_uri(&format!("race_{race_id}_male_render"))?;
    let female = registry.art_uri(&format!("race_{race_id}_female_render"))?;
    Some((male.to_string(), female.to_string()))
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

    let portraits = race_portraits(&AssetRegistry::load_embedded(), &race.id);
    let emoji = race_emoji(&race.id);

    rsx! {
        button {
            class: "flex flex-col items-center justify-center p-4 rounded-xl border-2 transition-all duration-200 cursor-pointer min-w-[120px] max-w-[160px] text-center {border_style}",
            onclick: move |_| on_select.call(race_clone.clone()),
            if let Some((male, female)) = portraits {
                div { class: "flex items-center justify-center -space-x-2 mb-2",
                    img {
                        class: "w-10 h-10 rounded-full border-2 border-slate-700 bg-slate-900 object-cover",
                        src: "{male}",
                        alt: "male portrait",
                    }
                    img {
                        class: "w-10 h-10 rounded-full border-2 border-slate-700 bg-slate-900 object-cover",
                        src: "{female}",
                        alt: "female portrait",
                    }
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
