use crate::components::FactionTheme;
use crate::models::{Faction, RaceInfo};
use dioxus::prelude::*;

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

    let icon = match race.id.as_str() {
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
    };

    rsx! {
        button {
            class: "flex flex-col items-center justify-center p-4 rounded-xl border-2 transition-all duration-200 cursor-pointer min-w-[120px] max-w-[160px] text-center {border_style}",
            onclick: move |_| on_select.call(race_clone.clone()),
            span { class: "text-3xl sm:text-4xl mb-2", "{icon}" }
            span { class: "text-xs sm:text-sm font-bold text-slate-100 leading-tight",
                "{display_name}"
            }
        }
    }
}
