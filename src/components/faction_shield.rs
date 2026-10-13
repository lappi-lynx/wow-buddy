use crate::assets::AssetRegistry;
use crate::models::{faction::FactionMeta, get_faction_meta, Faction};
use dioxus::prelude::*;

/// Faction shield art: an embedded `data:` URI when art exists, otherwise the
/// faction's emoji icon (the normal state before the curator imports the kit).
pub fn faction_art_source(registry: &AssetRegistry, faction: Faction) -> String {
    let id = match faction {
        Faction::Alliance => "faction_alliance_shield",
        Faction::Horde => "faction_horde_shield",
    };
    registry
        .art_uri(id)
        .map(str::to_string)
        .unwrap_or_else(|| get_faction_meta(faction).icon)
}

#[component]
pub fn FactionShield(
    meta: FactionMeta,
    title: String,
    motto: String,
    btn_text: Option<String>,
    on_select: EventHandler<Faction>,
) -> Element {
    let is_alliance = meta.faction == Faction::Alliance;
    let button_label = btn_text.unwrap_or_else(|| "Select".to_string());
    let registry = AssetRegistry::load_embedded();
    let art = faction_art_source(&registry, meta.faction);
    let has_art = art.starts_with("data:");

    let border_color = if is_alliance {
        "border-blue-500/60 hover:border-yellow-400 group-hover:shadow-[0_0_35px_rgba(59,130,246,0.6)]"
    } else {
        "border-red-600/60 hover:border-red-500 group-hover:shadow-[0_0_35px_rgba(220,38,38,0.6)]"
    };

    let bg_gradient = if is_alliance {
        "from-blue-950/80 via-slate-900 to-black hover:from-blue-900/90"
    } else {
        "from-red-950/80 via-neutral-900 to-black hover:from-red-900/90"
    };

    rsx! {
        button {
            class: "group relative flex flex-col items-center justify-between p-6 sm:p-8 rounded-2xl border-2 transition-all duration-300 transform hover:-translate-y-2 cursor-pointer bg-gradient-to-b {bg_gradient} {border_color} w-full max-w-sm min-h-[380px] shadow-2xl text-center select-none",
            onclick: move |_| on_select.call(meta.faction),
            div { class: "w-28 h-28 sm:w-32 sm:h-32 flex items-center justify-center rounded-full bg-black/40 border border-white/10 shadow-inner group-hover:scale-110 transition-transform duration-300",
                if has_art {
                    img {
                        class: "w-full h-full object-contain drop-shadow-[0_4px_12px_rgba(0,0,0,0.8)]",
                        src: "{art}",
                        alt: "{title}",
                    }
                } else {
                    span { class: "text-6xl sm:text-7xl drop-shadow-[0_4px_12px_rgba(0,0,0,0.8)]",
                        "{meta.icon}"
                    }
                }
            }
            div { class: "mt-6 flex flex-col items-center space-y-2",
                h3 { class: "text-2xl sm:text-3xl font-extrabold tracking-wider uppercase text-amber-200 group-hover:text-amber-300 transition-colors",
                    "{title}"
                }
                p { class: "text-xs sm:text-sm text-slate-300 italic px-2 max-w-xs leading-relaxed",
                    "\"{motto}\""
                }
            }
            div { class: "mt-6 px-6 py-2.5 rounded-full text-xs sm:text-sm font-bold uppercase tracking-wider bg-amber-500/20 text-amber-300 border border-amber-500/40 group-hover:bg-amber-500 group-hover:text-black transition-all duration-200 shadow-md",
                "{button_label}"
            }
        }
    }
}
