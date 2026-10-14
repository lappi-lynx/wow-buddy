use crate::assets::AssetRegistry;
use crate::components::{BlueprintExport, ClassTipsCard, GlossaryTerm, LanguagePicker, ModelViewer};
use crate::i18n::{I18nService, Language};
use crate::models::{
    class::{availability, SpellAvailability},
    gear::{get_tier_for_level, nearest_anchor, tier_short_key, GearTier},
    glossary::TERMS,
    milestones::{milestones_around, next_milestone},
    ClassInfo, Faction, RaceInfo,
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterAnimation {
    Stand,
    Attack,
    Cast,
    Dance,
}

impl CharacterAnimation {
    pub fn as_str(&self) -> &'static str {
        match self {
            CharacterAnimation::Stand => "stand",
            CharacterAnimation::Attack => "attack",
            CharacterAnimation::Cast => "cast",
            CharacterAnimation::Dance => "dance",
        }
    }

    pub fn label_key(&self) -> &'static str {
        match self {
            CharacterAnimation::Stand => "anim_stand",
            CharacterAnimation::Attack => "anim_attack",
            CharacterAnimation::Cast => "anim_cast",
            CharacterAnimation::Dance => "anim_dance",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            CharacterAnimation::Stand => "🛡️",
            CharacterAnimation::Attack => "⚔️",
            CharacterAnimation::Cast => "✨",
            CharacterAnimation::Dance => "🕺",
        }
    }

    pub fn all() -> &'static [CharacterAnimation] {
        &[
            CharacterAnimation::Stand,
            CharacterAnimation::Attack,
            CharacterAnimation::Cast,
            CharacterAnimation::Dance,
        ]
    }
}

pub fn get_tier_label_key(tier: GearTier) -> &'static str {
    match tier {
        GearTier::Starter => "tier_starter",
        GearTier::Dungeon => "tier_dungeon",
        GearTier::ElitePlate => "tier_elite",
        GearTier::RaidTier2 => "tier_raid",
    }
}

#[component]
pub fn DressingRoomView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    race: RaceInfo,
    class: ClassInfo,
    faction: Faction,
    on_back: EventHandler<()>,
    on_continue_to_guide: EventHandler<()>,
    on_open_gear_planner: EventHandler<()>,
) -> Element {
    let mut selected_level = use_signal(|| 1u8);
    let mut current_animation = use_signal(|| CharacterAnimation::Stand);

    let lang = *current_lang.read();
    let current_tier = get_tier_for_level(*selected_level.read());
    let level_now = *selected_level.read();
    let anchor = nearest_anchor(level_now);
    let set_name = i18n.read().t(tier_short_key(anchor), lang).to_string();
    let snap_label = if level_now == anchor {
        set_name.clone()
    } else {
        i18n.read().t_with(
            "ui_gear_snap",
            lang,
            &[("level", level_now.to_string()), ("set", set_name)],
        )
    };
    let registry = AssetRegistry::load_embedded();

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",
            header { class: "w-full max-w-5xl mx-auto flex justify-between items-center py-3 border-b border-slate-800/80 mb-4",
                button {
                    class: "flex items-center space-x-2 text-sm text-slate-400 hover:text-amber-300 transition-colors px-3 py-1.5 rounded-lg border border-slate-800 hover:border-slate-700 bg-slate-900/60",
                    onclick: move |_| on_back.call(()),
                    span { "←" }
                    span { "{i18n.read().t(\"btn_back\", lang)}" }
                }
                div { class: "flex items-center space-x-2",
                    LanguagePicker { current_lang: current_lang }
                }
            }

            main { class: "w-full max-w-4xl mx-auto flex flex-col items-center flex-grow space-y-6",
                // 3D Dressing Room Canvas
                ModelViewer {
                    current_lang: current_lang,
                    i18n: i18n,
                    race: race.clone(),
                    class: class.clone(),
                    faction: faction,
                    tier: current_tier,
                    animation: current_animation.read().as_str().to_string(),
                }

                // Action Buttons for Animations
                div { class: "flex flex-wrap justify-center gap-2 sm:gap-3 w-full max-w-md",
                    for anim in CharacterAnimation::all() {
                        {
                            let a = *anim;
                            let is_active = *current_animation.read() == a;
                            let btn_style = if is_active {
                                "bg-amber-500 text-black border-amber-400 font-bold shadow-lg"
                            } else {
                                "bg-slate-900 text-slate-300 border-slate-800 hover:bg-slate-800 font-medium"
                            };
                            let anim_title = i18n.read().t(a.label_key(), lang);
                            rsx! {
                                button {
                                    key: "{a.as_str()}",
                                    class: "px-3.5 py-1.5 rounded-xl border text-xs sm:text-sm flex items-center space-x-1.5 transition-all cursor-pointer {btn_style}",
                                    onclick: move |_| current_animation.set(a),
                                    span { "{a.icon()}" }
                                    span { "{anim_title}" }
                                }
                            }
                        }
                    }
                }

                // Level Evolution Slider (1 -> 20 -> 40 -> 60)
                div { class: "w-full max-w-lg bg-slate-900/80 border border-slate-800 rounded-2xl p-4 sm:p-6 shadow-xl flex flex-col space-y-3",
                    div { class: "flex justify-between items-center",
                        span { class: "text-xs uppercase tracking-wider font-semibold text-slate-400",
                            "{i18n.read().t_with(\"ui_gear_evolution\", lang, &[(\"level\", selected_level.read().to_string())])}"
                        }
                        span { class: "text-xs font-bold text-amber-400",
                            "{snap_label}"
                        }
                    }

                    input {
                        r#type: "range",
                        min: "1",
                        max: "60",
                        step: "1",
                        value: "{selected_level.read()}",
                        class: "w-full accent-amber-500 cursor-pointer h-11 bg-slate-800 rounded-lg",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(lvl) = evt.value().parse::<u8>() {
                                selected_level.set(lvl);
                            }
                        },
                    }

                    div { class: "flex justify-between text-[11px] font-bold text-slate-400 px-1",
                        button {
                            class: "hover:text-amber-400 cursor-pointer",
                            onclick: move |_| selected_level.set(1),
                            "Lvl 1"
                        }
                        button {
                            class: "hover:text-amber-400 cursor-pointer",
                            onclick: move |_| selected_level.set(20),
                            "Lvl 20"
                        }
                        button {
                            class: "hover:text-amber-400 cursor-pointer",
                            onclick: move |_| selected_level.set(40),
                            "Lvl 40"
                        }
                        button {
                            class: "hover:text-amber-400 cursor-pointer",
                            onclick: move |_| selected_level.set(60),
                            "Lvl 60"
                        }
                    }
                }

                // "What unlocks around here" + next unlock
                div { class: "w-full max-w-lg bg-slate-900/70 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                    div { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                        "{i18n.read().t(\"ui_unlocks_around\", lang)}"
                    }
                    div { class: "flex flex-wrap gap-2",
                        for m in milestones_around(level_now, 5) {
                            span {
                                key: "{m.level}",
                                class: "text-xs px-2 py-1 rounded-lg bg-slate-800 border border-slate-700 text-slate-200",
                                "{i18n.read().t(m.key, lang)}"
                                if !m.verified {
                                    span { class: "text-amber-400/80", " · {i18n.read().t(\"ui_beta_may_change\", lang)}" }
                                }
                            }
                        }
                    }
                    if let Some(next) = next_milestone(level_now) {
                        p { class: "text-xs text-amber-300",
                            "{i18n.read().t_with(\"ui_next_unlock\", lang, &[(\"level\", next.level.to_string())])} — {i18n.read().t(next.key, lang)}"
                        }
                    }
                }

                // Novice Card (Difficulty & 3 Core Spells)
                div { class: "w-full max-w-lg bg-slate-900/90 border border-slate-800 rounded-2xl p-5 shadow-xl flex flex-col space-y-4",
                    div { class: "flex justify-between items-center border-b border-slate-800 pb-3",
                        div {
                            h3 { class: "text-base sm:text-lg font-bold text-amber-300", "{i18n.read().t_with(\"ui_novice_guide\", lang, &[(\"class\", class.name.clone())])}" }
                            p { class: "text-xs text-slate-400", "{i18n.read().t_with(\"ui_role_label\", lang, &[(\"role\", class.role.clone())])}" }
                        }
                        span { class: "text-xs px-2.5 py-1 rounded-full bg-emerald-950/80 border border-emerald-500/40 text-emerald-400 font-semibold",
                            "🟢 {class.difficulty}"
                        }
                    }

                    div { class: "space-y-2.5",
                        span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider block", "{i18n.read().t(\"ui_core_spells\", lang)}" }
                        for spell in class.core_spells.iter() {
                            {
                                let icon_uri = spell
                                    .icon_id
                                    .as_deref()
                                    .and_then(|id| registry.art_uri(id));
                                let unlock = availability(spell, level_now);
                                rsx! {
                                    div {
                                        key: "{spell.name}",
                                        class: "flex items-start space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                        if let Some(uri) = icon_uri {
                                            img {
                                                class: "w-9 h-9 rounded-md shrink-0",
                                                src: "{uri}",
                                                alt: "{spell.name}",
                                            }
                                        } else {
                                            span { class: "text-xl", "{spell.icon}" }
                                        }
                                        div { class: "flex flex-col",
                                            span { class: "text-xs sm:text-sm font-bold text-slate-200", "{spell.name}" }
                                            span { class: "text-xs text-slate-400 leading-tight mt-0.5", "{spell.description}" }
                                        }
                                        if let SpellAvailability::AtLevel(l) = unlock {
                                            span { class: "ml-auto shrink-0 text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400",
                                                "{i18n.read().t_with(\"ui_unlocks_at\", lang, &[(\"level\", l.to_string())])}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "w-full py-3 rounded-xl font-bold text-sm uppercase tracking-wider bg-amber-500 hover:bg-amber-400 text-black shadow-lg transition-transform active:scale-95 cursor-pointer mt-2",
                        onclick: move |_| on_continue_to_guide.call(()),
                        "{i18n.read().t(\"ui_proceed_to_guide\", lang)}"
                    }
                }

                // Terminology (tap-to-explain)
                div { class: "w-full max-w-lg bg-slate-900/70 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                    div { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                        "{i18n.read().t(\"ui_glossary_title\", lang)}"
                    }
                    div { class: "flex flex-wrap gap-2",
                        for term in TERMS.iter() {
                            GlossaryTerm {
                                key: "{term.id}",
                                term_id: term.id.to_string(),
                                current_lang: current_lang,
                                i18n: i18n,
                            }
                        }
                    }
                }

                // Character Blueprint Export Card
                BlueprintExport {
                    faction: faction,
                    race: race.clone(),
                    class: class.clone(),
                    current_lang: current_lang,
                    i18n: i18n,
                }

                // Entry point into the Gear Planner
                button {
                    class: "w-full max-w-lg py-3 rounded-xl font-bold text-xs sm:text-sm uppercase tracking-wider bg-slate-800 hover:bg-slate-700 text-amber-200 border border-amber-500/30 shadow-lg transition-transform active:scale-95 cursor-pointer",
                    onclick: move |_| on_open_gear_planner.call(()),
                    "{i18n.read().t(\"ui_open_gear_planner\", lang)}"
                }

                // New Player Tips for the selected class (all specs at once)
                ClassTipsCard {
                    current_lang: current_lang,
                    i18n: i18n,
                    class_id: class.id.clone(),
                }
            }

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-400 border-t border-slate-900 mt-6",
                "{i18n.read().t(\"footer_offline\", lang)}"
            }
        }
    }
}
