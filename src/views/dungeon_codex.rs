use crate::assets::AssetRegistry;
use crate::components::reference_popover::{item_popover, PopoverData};
use crate::components::{AppFooter, DungeonRuleCard, LootQuiz, ReadinessCard, ReferencePopover};
use crate::i18n::{I18nService, Language};
use crate::models::dungeon::get_dungeon_codex;
use dioxus::prelude::*;

pub fn get_active_dungeons() -> Vec<&'static str> {
    vec![
        "ragefire_chasm",
        "hall_of_thanes",
        "wailing_caverns",
        "deadmines",
        "shadowfang_keep",
    ]
}

#[component]
pub fn DungeonCodexView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
) -> Element {
    let mut selected_dungeon_id = use_signal(|| "deadmines".to_string());
    let mut popover = use_signal(|| None::<PopoverData>);

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let current_id = selected_dungeon_id.read().clone();
    let codex = get_dungeon_codex(&current_id);
    let registry = AssetRegistry::load_embedded();

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",

            main { class: "w-full max-w-4xl mx-auto flex flex-col items-center flex-grow space-y-6",
                div { class: "text-center",
                    h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide",
                        "🏰 {i18n_svc.t(\"ui_dungeon_survival_codex\", lang)}"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "{i18n_svc.t(\"ui_dungeon_subtitle\", lang)}"
                    }
                }

                // Dungeon Selection Tabs
                div { class: "flex flex-wrap justify-center gap-2 bg-slate-900 p-1.5 rounded-2xl border border-slate-800",
                    for dungeon_id in get_active_dungeons().iter() {
                        {
                            let id = dungeon_id.to_string();
                            let tab = get_dungeon_codex(&id);
                            let is_active = current_id == id;
                            let tab_class = if is_active {
                                "px-4 py-2 rounded-xl text-xs sm:text-sm font-bold bg-amber-500 text-black shadow-md cursor-pointer"
                            } else {
                                "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold text-slate-400 hover:text-slate-200 cursor-pointer"
                            };
                            rsx! {
                                button {
                                    key: "{id}",
                                    class: "{tab_class}",
                                    onclick: move |_| selected_dungeon_id.set(id.clone()),
                                    "{tab.icon} {tab.name} ({tab.level_range})"
                                }
                            }
                        }
                    }
                }

                // Readiness checklist + meter + find-a-group card
                ReadinessCard { current_lang: current_lang, i18n: i18n }

                // Summary Stats Bar
                div { class: "w-full grid grid-cols-1 sm:grid-cols-3 gap-3",
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "⏱️" }
                        div {
                            span { class: "text-xs text-slate-400 block", "{i18n_svc.t(\"ui_estimated_duration\", lang)}" }
                            span { class: "text-sm sm:text-base font-bold text-amber-300", "~{codex.estimated_time} min" }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "🎒" }
                        div {
                            span { class: "text-xs text-slate-400 block", "{i18n_svc.t(\"ui_bags_preparation\", lang)}" }
                            span { class: "text-sm sm:text-base font-bold text-emerald-400", "{codex.bag_slots_required}+ slots" }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "💬" }
                        div {
                            span { class: "text-xs text-slate-400 block", "{i18n_svc.t(\"ui_communication\", lang)}" }
                            span { class: "text-sm sm:text-base font-bold text-blue-300", "{i18n_svc.t(\"ui_party_chat_only\", lang)}" }
                        }
                    }
                }

                // Recommended group level (per-dungeon, Beta-marked when advice)
                if let Some(rec) = codex.recommended_avg_level.as_ref() {
                    div { class: "w-full p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "👥" }
                        div { class: "flex flex-col",
                            span { class: "text-xs text-slate-400", "{i18n_svc.t(\"ui_codex_recommended\", lang)}" }
                            span { class: "text-sm sm:text-base font-bold text-amber-300",
                                "{rec.text.get(lang)}"
                                if !rec.verified {
                                    span { class: "text-xs font-normal text-amber-400/80", " · {i18n_svc.t(\"ui_beta_may_change\", lang)}" }
                                }
                            }
                        }
                    }
                }

                // What's inside — mob types and the notable names
                if !codex.mobs.is_empty() {
                    div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-3",
                        h3 { class: "text-xs font-bold uppercase tracking-wider text-amber-300",
                            "{i18n_svc.t(\"ui_codex_inside\", lang)}"
                        }
                        div { class: "flex flex-wrap gap-2",
                            for mob in codex.mobs.iter() {
                                div { key: "{mob.name}", class: "px-3 py-2 rounded-xl bg-slate-950/60 border border-slate-800/80 flex flex-col",
                                    span { class: "text-xs sm:text-sm font-bold text-slate-200", "{mob.name}" }
                                    span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-400", "{mob.kind.get(lang)}" }
                                    if let Some(note) = mob.note.as_ref() {
                                        span { class: "text-xs text-slate-400", "{note.get(lang)}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Watch out — per-dungeon warnings
                if !codex.watch_out.is_empty() {
                    div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                        h3 { class: "text-xs font-bold uppercase tracking-wider text-amber-300",
                            "{i18n_svc.t(\"ui_codex_watch_out\", lang)}"
                        }
                        for (i, w) in codex.watch_out.iter().enumerate() {
                            p { key: "{i}", class: "text-xs sm:text-sm text-slate-300",
                                "⚠️ {w.text.get(lang)}"
                                if !w.verified {
                                    span { class: "text-xs text-amber-400/80", " · {i18n_svc.t(\"ui_beta_may_change\", lang)}" }
                                }
                            }
                        }
                    }
                }

                // Prep — potions, bag slots (stats row above) and buffs
                if let Some(prep) = codex.prep.as_ref() {
                    div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                        h3 { class: "text-xs font-bold uppercase tracking-wider text-amber-300",
                            "{i18n_svc.t(\"ui_codex_prep\", lang)}"
                        }
                        if let Some(potions) = prep.potions.as_ref() {
                            p { class: "text-xs sm:text-sm text-slate-300", "🧪 {potions.get(lang)}" }
                        }
                        for buff in prep.buffs.iter() {
                            p { key: "{buff.text_en}", class: "text-xs sm:text-sm text-slate-300", "✨ {buff.get(lang)}" }
                        }
                    }
                }

                // Loot — top blue drops with icons; tap for the source popover
                if !codex.loot.is_empty() {
                    div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                        h3 { class: "text-xs font-bold uppercase tracking-wider text-amber-300",
                            "{i18n_svc.t(\"ui_codex_loot\", lang)}"
                        }
                        for entry in codex.loot.iter() {
                            {
                                let reg = registry.clone();
                                let name = entry.name.clone();
                                let icon = entry.icon_id.clone();
                                let source = entry.source.clone();
                                rsx! {
                                    button {
                                        key: "{entry.item_id}",
                                        class: "flex items-center space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80 hover:border-slate-700 transition-colors text-left cursor-pointer",
                                        onclick: move |_| {
                                            let i18n_ref = i18n.read();
                                            popover.set(Some(item_popover(&reg, &i18n_ref, lang, &name, Some(icon.as_str()), &source)));
                                        },
                                        img {
                                            class: "w-9 h-9 rounded-md shrink-0",
                                            src: "{registry.item_icon(&entry.icon_id)}",
                                            alt: "{entry.name}",
                                        }
                                        div { class: "flex flex-col flex-1",
                                            span { class: "text-xs sm:text-sm font-bold text-slate-200", "{entry.name}" }
                                            span { class: "text-xs text-slate-400", "{entry.slot}" }
                                        }
                                        span { class: "text-[10px] font-bold uppercase tracking-wider text-amber-400/90 text-right", "{entry.source}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Core Survival Rules Grid
                div { class: "w-full grid grid-cols-1 sm:grid-cols-2 gap-4",
                    for rule in codex.rules.iter() {
                        DungeonRuleCard {
                            key: "{rule.id}",
                            rule: rule.clone(),
                        }
                    }
                }

                // Interactive Loot Etiquette Simulator (Need vs Greed)
                LootQuiz {
                    current_lang: current_lang,
                    i18n: i18n,
                }
            }

            if let Some(data) = popover.read().clone() {
                ReferencePopover { data: data, on_close: move |_| popover.set(None) }
            }

            AppFooter { current_lang: current_lang, i18n: i18n }
        }
    }
}
