use crate::models::{gear::GearTier, ClassInfo, RaceInfo};
use dioxus::prelude::*;

#[component]
pub fn ModelViewer(
    race: RaceInfo,
    class: ClassInfo,
    tier: GearTier,
    animation: String,
) -> Element {
    let is_skyborne = race.id == "skyborne";

    let avatar_glyph = match class.id.as_str() {
        "warrior" => "⚔️",
        "paladin" => "🔨",
        "hunter" => "🏹",
        "rogue" => "🗡️",
        "priest" => "✨",
        "shaman" => "⚡",
        "mage" => "🔮",
        "warlock" => "💀",
        "druid" => "🐾",
        _ => "🛡️",
    };

    let tier_aura = match tier {
        GearTier::Starter => "from-slate-800 to-slate-950 border-slate-700",
        GearTier::Dungeon => "from-blue-950/80 to-slate-950 border-blue-500/50 shadow-[0_0_30px_rgba(59,130,246,0.3)]",
        GearTier::ElitePlate => "from-purple-950/80 to-slate-950 border-purple-500/50 shadow-[0_0_30px_rgba(168,85,247,0.3)]",
        GearTier::RaidTier2 => "from-amber-950/80 to-slate-950 border-amber-500/60 shadow-[0_0_40px_rgba(245,158,11,0.4)]",
    };

    let tier_title = match tier {
        GearTier::Starter => "Level 1 Starter",
        GearTier::Dungeon => "Level 20 Dungeon Set",
        GearTier::ElitePlate => "Level 40 Elite Armor",
        GearTier::RaidTier2 => "Level 60 Legendary T2",
    };

    rsx! {
        div { class: "relative w-full aspect-square max-w-sm sm:max-w-md mx-auto rounded-3xl border-2 bg-gradient-to-b {tier_aura} flex flex-col items-center justify-between p-6 overflow-hidden transition-all duration-300",
            div { class: "w-full flex justify-between items-center z-10",
                span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full bg-slate-900/80 border border-slate-700 text-slate-300",
                    "Animation: {animation}"
                }
                if is_skyborne {
                    span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full bg-amber-500/20 border border-amber-500/40 text-amber-300",
                        "2D High Art"
                    }
                } else {
                    span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full bg-blue-500/20 border border-blue-500/40 text-blue-300",
                        "WebGL 3D Active"
                    }
                }
            }

            // Central 3D / 2D Model Representation
            div { class: "relative my-auto flex flex-col items-center justify-center group cursor-grab",
                div { class: "w-48 h-48 sm:w-56 sm:h-56 rounded-full bg-black/40 border border-amber-500/20 flex items-center justify-center relative shadow-inner",
                    div { class: "text-7xl sm:text-8xl drop-shadow-[0_15px_25px_rgba(0,0,0,0.9)] transform group-hover:scale-110 transition-transform duration-300",
                        "{avatar_glyph}"
                    }
                    div { class: "absolute -bottom-2 text-2xl",
                        match race.id.as_str() {
                            "skyborne" => "🪶",
                            "night_elf" => "🌙",
                            "tauren" => "🐂",
                            "orc" => "🪓",
                            "human" => "🛡️",
                            "dwarf" => "⛏️",
                            "gnome" => "⚙️",
                            "undead" => "💀",
                            "troll" => "🏹",
                            _ => "👤",
                        }
                    }
                }
                div { class: "mt-4 text-center",
                    h4 { class: "text-lg sm:text-xl font-extrabold text-amber-200 tracking-wide",
                        "{race.name} {class.name}"
                    }
                    p { class: "text-xs font-medium text-slate-400 mt-0.5",
                        "{tier_title}"
                    }
                }
            }

            div { class: "w-full text-center text-slate-500 text-[11px] uppercase tracking-widest z-10",
                "Drag to rotate • Pinch to zoom"
            }
        }
    }
}
