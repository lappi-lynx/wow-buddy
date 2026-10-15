use crate::components::{DungeonRuleCard, LanguagePicker, LootQuiz, ReadinessCard};
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
    on_back: EventHandler<()>,
) -> Element {
    let mut selected_dungeon_id = use_signal(|| "deadmines".to_string());

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let current_id = selected_dungeon_id.read().clone();
    let codex = get_dungeon_codex(&current_id);

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",
            header { class: "w-full max-w-5xl mx-auto flex justify-between items-center py-3 border-b border-slate-800/80 mb-4",
                button {
                    class: "flex items-center justify-center w-9 h-9 text-base text-slate-400 hover:text-amber-300 transition-colors rounded-lg border border-slate-800 hover:border-slate-700 bg-slate-900/60 cursor-pointer",
                    onclick: move |_| on_back.call(()),
                    "←"
                }
            }

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

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6 flex flex-col items-center space-y-2",
                LanguagePicker { current_lang: current_lang }
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}
