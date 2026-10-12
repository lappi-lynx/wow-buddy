use crate::components::{DungeonRuleCard, LanguagePicker};
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
    let mut simulated_loot_choice = use_signal(|| "greed".to_string());

    let lang = *current_lang.read();
    let current_id = selected_dungeon_id.read().clone();
    let codex = get_dungeon_codex(&current_id);

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
                div { class: "text-center",
                    h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide",
                        "🏰 Dungeon Survival Codex"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "The unspoken 5-player rules so you never get kicked from your first group"
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
                            span { class: "text-xs text-slate-400 block", "Estimated Duration" }
                            span { class: "text-sm sm:text-base font-bold text-amber-300", "~{codex.estimated_time_minutes} minutes" }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "🎒" }
                        div {
                            span { class: "text-xs text-slate-400 block", "Bags Preparation" }
                            span { class: "text-sm sm:text-base font-bold text-emerald-400", "Clear {codex.bag_slots_required}+ free slots" }
                        }
                    }
                    div { class: "p-4 rounded-2xl bg-slate-900/80 border border-slate-800 flex items-center space-x-3",
                        span { class: "text-2xl", "💬" }
                        div {
                            span { class: "text-xs text-slate-400 block", "Communication" }
                            span { class: "text-sm sm:text-base font-bold text-blue-300", "/p party chat only" }
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
                div { class: "w-full bg-slate-900/90 border border-amber-500/30 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-3",
                    div { class: "flex justify-between items-center",
                        h4 { class: "text-sm sm:text-base font-bold text-amber-300",
                            "Interactive Simulator: Loot Etiquette Test"
                        }
                        span { class: "text-xs px-2.5 py-1 rounded-full bg-amber-500/20 text-amber-300 font-bold",
                            "Golden Rule"
                        }
                    }
                    p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                        "A blue quality item just dropped: [Smite's Mighty Hammer]. You already have an equal or better weapon, or you cannot equip two-handed hammers. Which button do you press?"
                    }
                    div { class: "flex space-x-4 pt-1",
                        button {
                            class: if *simulated_loot_choice.read() == "greed" {
                                "flex-1 py-2.5 rounded-xl font-bold text-xs sm:text-sm bg-emerald-600 text-white border border-emerald-400 shadow-lg cursor-pointer"
                            } else {
                                "flex-1 py-2.5 rounded-xl font-semibold text-xs sm:text-sm bg-slate-800 text-slate-300 border border-slate-700 hover:bg-slate-700 cursor-pointer"
                            },
                            onclick: move |_| simulated_loot_choice.set("greed".to_string()),
                            "🎲 Greed (Не откажусь) — Correct!"
                        }
                        button {
                            class: if *simulated_loot_choice.read() == "need" {
                                "flex-1 py-2.5 rounded-xl font-bold text-xs sm:text-sm bg-rose-700 text-white border border-rose-500 shadow-lg cursor-pointer"
                            } else {
                                "flex-1 py-2.5 rounded-xl font-semibold text-xs sm:text-sm bg-slate-800 text-slate-300 border border-slate-700 hover:bg-slate-700 cursor-pointer"
                            },
                            onclick: move |_| simulated_loot_choice.set("need".to_string()),
                            "⚔️ Need (Нужно) — Ninja Looting!"
                        }
                    }
                    if *simulated_loot_choice.read() == "greed" {
                        div { class: "p-3 rounded-xl bg-emerald-950/60 border border-emerald-500/40 text-xs text-emerald-300",
                            "✓ Perfect! Never roll Need unless it is a direct upgrade you will equip immediately."
                        }
                    } else {
                        div { class: "p-3 rounded-xl bg-rose-950/60 border border-rose-500/40 text-xs text-rose-300",
                            "⚠️ Warning: Rolling Need for vendor gold will get you kicked and blacklisted on your realm!"
                        }
                    }
                }
            }

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6",
                "World of Warcraft Forever 1.60.1 Guide • 100% Offline"
            }
        }
    }
}
