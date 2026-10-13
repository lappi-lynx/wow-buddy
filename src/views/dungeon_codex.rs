use crate::components::{DungeonRuleCard, LanguagePicker, LootQuiz};
use crate::i18n::{I18nService, Language};
use crate::models::dungeon::get_dungeon_codex;
use dioxus::prelude::*;

pub fn get_active_dungeons() -> Vec<&'static str> {
    vec!["deadmines", "ragefire_chasm"]
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
                    class: "flex items-center space-x-2 text-sm text-slate-400 hover:text-amber-300 transition-colors px-3 py-1.5 rounded-lg border border-slate-800 hover:border-slate-700 bg-slate-900/60",
                    onclick: move |_| on_back.call(()),
                    span { "←" }
                    span { "{i18n_svc.t(\"btn_back\", lang)}" }
                }
                div { class: "flex items-center space-x-2",
                    LanguagePicker { current_lang: current_lang }
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
                div { class: "flex space-x-3 bg-slate-900 p-1.5 rounded-2xl border border-slate-800",
                    button {
                        class: if current_id == "deadmines" {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-bold bg-amber-500 text-black shadow-md"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold text-slate-400 hover:text-slate-200"
                        },
                        onclick: move |_| selected_dungeon_id.set("deadmines".to_string()),
                        "☠️ The Deadmines (17–21)"
                    }
                    button {
                        class: if current_id == "ragefire_chasm" {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-bold bg-amber-500 text-black shadow-md"
                        } else {
                            "px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold text-slate-400 hover:text-slate-200"
                        },
                        onclick: move |_| selected_dungeon_id.set("ragefire_chasm".to_string()),
                        "🔥 Ragefire Chasm (13–18)"
                    }
                }

                // Summary Stats Bar
                div { class: "w-full grid grid-cols-1 sm:grid-cols-3 gap-3",
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "⏱️" }
                        div {
                            span { class: "text-xs text-slate-400 block", "{i18n_svc.t(\"ui_estimated_duration\", lang)}" }
                            span { class: "text-sm sm:text-base font-bold text-amber-300", "~{codex.estimated_time_minutes} min" }
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

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6",
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}
