use crate::i18n::{I18nService, Language};
use dioxus::prelude::*;

/// The item pinned by the loot-etiquette scenario.
/// Verified against the WoW Forever 1.60.1 item database:
/// Item ID 7230, two-handed mace, level 20, drops from
/// Mr. Smite in The Deadmines.
pub const LOOT_SCENARIO_ITEM: &str = "Smite's Mighty Hammer";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LootChoice {
    Need,
    Greed,
}

/// The golden rule of dungeon loot: Greed is correct when the item
/// is not a direct, immediate upgrade you will equip right now;
/// rolling Need on such an item is ninja-looting.
pub fn evaluate_loot_choice(choice: LootChoice) -> bool {
    matches!(choice, LootChoice::Greed)
}

#[component]
pub fn LootQuiz(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
) -> Element {
    let mut choice = use_signal(|| None::<LootChoice>);
    let chosen = *choice.read();
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    let scenario = i18n_svc.t_with(
        "ui_loot_scenario",
        lang,
        &[("item", LOOT_SCENARIO_ITEM.to_string())],
    );
    let correct = chosen.map(evaluate_loot_choice).unwrap_or(false);
    let reset_label = i18n_svc.t("ui_loot_reset", lang);

    rsx! {
        div { class: "w-full bg-slate-900/90 border border-amber-500/30 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-3",
            div { class: "flex justify-between items-center",
                h4 { class: "text-sm sm:text-base font-bold text-amber-300",
                    "{i18n_svc.t(\"ui_loot_etiquette_test\", lang)}"
                }
                span { class: "text-xs px-2.5 py-1 rounded-full bg-amber-500/20 text-amber-300 font-bold",
                    "{i18n_svc.t(\"ui_golden_rule\", lang)}"
                }
            }

            p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                "{scenario}"
            }

            if chosen.is_none() {
                div { class: "flex space-x-4 pt-1",
                    button {
                        class: "flex-1 py-2.5 rounded-xl font-bold text-xs sm:text-sm bg-emerald-600 text-white border border-emerald-400 shadow-lg cursor-pointer",
                        onclick: move |_| choice.set(Some(LootChoice::Greed)),
                        "{i18n_svc.t(\"ui_greed_btn\", lang)}"
                    }
                    button {
                        class: "flex-1 py-2.5 rounded-xl font-bold text-xs sm:text-sm bg-rose-700 text-white border border-rose-500 shadow-lg cursor-pointer",
                        onclick: move |_| choice.set(Some(LootChoice::Need)),
                        "{i18n_svc.t(\"ui_need_btn\", lang)}"
                    }
                }
            } else if correct {
                div { class: "p-3 rounded-xl bg-emerald-950/60 border border-emerald-500/40 text-xs text-emerald-300 flex flex-col space-y-2",
                    p { "{i18n_svc.t(\"ui_greed_success\", lang)}" }
                    button {
                        class: "self-start text-xs font-semibold underline text-emerald-400 hover:text-emerald-200 cursor-pointer",
                        onclick: move |_| choice.set(None),
                        "{reset_label}"
                    }
                }
            } else {
                div { class: "p-3 rounded-xl bg-rose-950/60 border border-rose-500/40 text-xs text-rose-300 flex flex-col space-y-2",
                    p { "{i18n_svc.t(\"ui_need_warning\", lang)}" }
                    button {
                        class: "self-start text-xs font-semibold underline text-rose-400 hover:text-rose-200 cursor-pointer",
                        onclick: move |_| choice.set(None),
                        "{reset_label}"
                    }
                }
            }
        }
    }
}
