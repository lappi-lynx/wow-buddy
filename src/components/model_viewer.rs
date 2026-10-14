use crate::assets::AssetRegistry;
use crate::components::race_tile::class_render_source;
use crate::i18n::{I18nService, Language};
use crate::models::{gear::GearTier, ClassInfo, Faction, RaceInfo};
use crate::views::race_selector::format_race_display_name;
use dioxus::prelude::*;

/// What the viewer actually shows. The badge text follows this — it never
/// reports WebGL capability on its own (we have no 3D models yet).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerMode {
    Model3d,
    PaperDoll,
    Placeholder,
}

/// Choose the honest viewer mode from capability + what content exists.
pub fn choose_mode(webgl_ok: bool, has_model: bool, has_paper_doll: bool) -> ViewerMode {
    if webgl_ok && has_model {
        ViewerMode::Model3d
    } else if has_paper_doll {
        ViewerMode::PaperDoll
    } else {
        ViewerMode::Placeholder
    }
}

#[component]
pub fn ModelViewer(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    race: RaceInfo,
    class: ClassInfo,
    faction: Faction,
    tier: GearTier,
    animation: String,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let display_race = format_race_display_name(&race, faction);

    // Honest mode: no model and no WebGL detection in this build; a paper-doll
    // (the class icon) exists for every class, so the badge reads "Preview".
    let mode = choose_mode(
        false,
        false,
        !class_render_source(&AssetRegistry::load_embedded(), &class.id).is_empty(),
    );
    let mode_label = match mode {
        ViewerMode::Model3d => i18n_svc.t("ui_viewer_3d", lang),
        ViewerMode::PaperDoll => i18n_svc.t("ui_viewer_preview", lang),
        ViewerMode::Placeholder => i18n_svc.t("ui_viewer_soon", lang),
    };
    let is_3d = mode == ViewerMode::Model3d;

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
    let class_art = class_render_source(&AssetRegistry::load_embedded(), &class.id);
    let has_class_art = class_art.starts_with("data:");

    let tier_aura = match tier {
        GearTier::Starter => "from-slate-800 to-slate-950 border-slate-700",
        GearTier::Dungeon => "from-blue-950/80 to-slate-950 border-blue-500/50 shadow-[0_0_30px_rgba(59,130,246,0.3)]",
        GearTier::ElitePlate => "from-purple-950/80 to-slate-950 border-purple-500/50 shadow-[0_0_30px_rgba(168,85,247,0.3)]",
        GearTier::RaidTier2 => "from-amber-950/80 to-slate-950 border-amber-500/60 shadow-[0_0_40px_rgba(245,158,11,0.4)]",
    };

    let tier_title = match tier {
        GearTier::Starter => i18n_svc.t("tier_starter", lang),
        GearTier::Dungeon => i18n_svc.t("tier_dungeon", lang),
        GearTier::ElitePlate => i18n_svc.t("tier_elite", lang),
        GearTier::RaidTier2 => i18n_svc.t("tier_raid", lang),
    };

    let anim_text = i18n_svc.t_with("ui_animation", lang, &[("anim", animation)]);

    rsx! {
        div { class: "relative w-full aspect-square max-w-sm sm:max-w-md mx-auto rounded-3xl border-2 bg-gradient-to-b {tier_aura} flex flex-col items-center justify-between p-6 overflow-hidden transition-all duration-300",
            div { class: "w-full flex justify-between items-center z-10",
                span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full bg-slate-900/80 border border-slate-700 text-slate-300",
                    "{anim_text}"
                }
                span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full bg-blue-500/20 border border-blue-500/40 text-blue-300",
                    "{mode_label}"
                }
            }

            // Central 3D / 2D Model Representation
            div { class: "relative my-auto flex flex-col items-center justify-center group cursor-grab",
                div { class: "w-48 h-48 sm:w-56 sm:h-56 rounded-full bg-black/40 border border-amber-500/20 flex items-center justify-center relative shadow-inner",
                    if has_class_art {
                        img {
                            class: "w-20 h-20 sm:w-24 sm:h-24 rounded-lg drop-shadow-[0_15px_25px_rgba(0,0,0,0.9)] transform group-hover:scale-110 transition-transform duration-300",
                            src: "{class_art}",
                            alt: "{class.name} class icon",
                        }
                    } else {
                        div { class: "text-7xl sm:text-8xl drop-shadow-[0_15px_25px_rgba(0,0,0,0.9)] transform group-hover:scale-110 transition-transform duration-300",
                            "{avatar_glyph}"
                        }
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
                        "{display_race} {class.name}"
                    }
                    p { class: "text-xs font-medium text-slate-400 mt-0.5",
                        "{tier_title}"
                    }
                }
            }

            if is_3d {
                div { class: "w-full text-center text-slate-400 text-[11px] uppercase tracking-widest z-10",
                    "{i18n_svc.t(\"ui_drag_rotate\", lang)}"
                }
            }
        }
    }
}
