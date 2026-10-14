use crate::components::reference_popover::{npc_popover, PopoverData};
use crate::components::{LanguagePicker, LegacyPresetWidget, ReferencePopover};
use crate::i18n::{I18nService, Language};
use crate::models::{reference::NpcBook, Faction, GameData, RaceInfo, StarterZoneInfo};
use crate::views::race_selector::format_race_display_name;
use dioxus::prelude::*;

/// Trainer + bag reminders. Step 1 — the first quest — is rendered by the
/// view itself because the NPC name is interactive.
pub fn get_starter_zone_reminders(zone: &StarterZoneInfo) -> Vec<String> {
    vec![zone.trainer_reminder.clone(), zone.bag_reminder.clone()]
}

/// Split the localized first-quest template around its `{npc}` placeholder:
/// `Some((before, after))`, or `None` when the template has no placeholder.
pub fn split_first_quest(template: &str) -> Option<(String, String)> {
    template
        .split_once("{npc}")
        .map(|(before, after)| (before.to_string(), after.to_string()))
}

#[component]
pub fn ProgressionGuideView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    game_data: Signal<GameData>,
    race: RaceInfo,
    faction: Faction,
    on_back: EventHandler<()>,
    on_continue_to_dungeons: EventHandler<()>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = game_data.read();

    let zone = data
        .get_starter_zone(&race.starter_zone_id)
        .cloned()
        .unwrap_or_else(|| StarterZoneInfo {
            id: "unknown".to_string(),
            name_ru: "Стартовая зона".to_string(),
            name_en: "Starter Zone".to_string(),
            name_fi: "Aloitusalue".to_string(),
            level_range: "1-6".to_string(),
            starting_npc: "Quartermaster".to_string(),
            starting_npc_id: None,
            trainer_reminder: "Train skills at levels 2 and 4.".to_string(),
            bag_reminder: "Buy first bag with 5 silver.".to_string(),
            next_zone_direction: "Proceed to main territory.".to_string(),
        });

    let mut popover = use_signal(|| None::<PopoverData>);
    let npc_book = NpcBook::load_embedded();
    let first_quest_template = i18n_svc.t("ui_first_quest", lang).to_string();
    let (first_quest_before, first_quest_after) =
        match split_first_quest(&first_quest_template) {
            Some(parts) => parts,
            None => (
                first_quest_template.replace("{npc}", &zone.starting_npc),
                String::new(),
            ),
        };
    let clickable_npc = zone
        .starting_npc_id
        .clone()
        .filter(|id| first_quest_template.contains("{npc}") && npc_book.get(id).is_some());
    let reminders = get_starter_zone_reminders(&zone);

    let zone_name = match lang {
        Language::Ru => &zone.name_ru,
        Language::En => &zone.name_en,
        Language::Fi => &zone.name_fi,
    };

    let display_race = format_race_display_name(&race, faction);
    let guide_title = i18n_svc.t_with("ui_progression_guide_for", lang, &[("race", display_race)]);
    let zone_header = i18n_svc.t_with(
        "ui_zone_prefix",
        lang,
        &[("zone", zone_name.clone()), ("levels", zone.level_range.clone())],
    );

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
                        "{guide_title}"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "{i18n_svc.t(\"ui_step_by_step\", lang)}"
                    }
                }

                // 1-6 lvl Checklist
                div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-4",
                    div { class: "flex justify-between items-center border-b border-slate-800 pb-3",
                        div {
                            h3 { class: "text-base sm:text-lg font-bold text-slate-100",
                                "{i18n_svc.t(\"ui_starter_glade_checklist\", lang)}"
                            }
                            p { class: "text-xs text-amber-400 font-medium",
                                "{zone_header}"
                            }
                        }
                        span { class: "text-xs px-2.5 py-1 rounded-full bg-blue-950/80 border border-blue-500/40 text-blue-300 font-semibold",
                            "{i18n_svc.t(\"ui_phase_1_entry\", lang)}"
                        }
                    }

                    div { class: "space-y-3",
                        div {
                            key: "first-quest",
                            class: "flex items-start space-x-3 p-3 rounded-xl bg-slate-950/60 border border-slate-800/80",
                            span { class: "flex-shrink-0 w-6 h-6 rounded-full bg-amber-500/20 text-amber-300 border border-amber-500/40 flex items-center justify-center text-xs font-bold",
                                "1"
                            }
                            p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                                span { "{first_quest_before}" }
                                if let Some(npc_id) = clickable_npc.clone() {
                                    button {
                                        class: "font-semibold text-amber-300 hover:text-amber-200 cursor-pointer underline decoration-dotted underline-offset-2",
                                        onclick: move |_| {
                                            let book = NpcBook::load_embedded();
                                            let i18n_ref = i18n.read();
                                            if let Some(data) = npc_popover(&book, &i18n_ref, lang, &npc_id) {
                                                popover.set(Some(data));
                                            }
                                        },
                                        "{zone.starting_npc}"
                                    }
                                } else {
                                    span { "{zone.starting_npc}" }
                                }
                                span { "{first_quest_after}" }
                            }
                        }
                        for (idx, item) in reminders.iter().enumerate() {
                            div {
                                key: "{idx}",
                                class: "flex items-start space-x-3 p-3 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                span { class: "flex-shrink-0 w-6 h-6 rounded-full bg-amber-500/20 text-amber-300 border border-amber-500/40 flex items-center justify-center text-xs font-bold",
                                    "{idx + 2}"
                                }
                                p { class: "text-xs sm:text-sm text-slate-300 leading-relaxed",
                                    "{item}"
                                }
                            }
                        }
                    }
                }

                // Next Zone Direction (6–12 lvl)
                div { class: "w-full bg-gradient-to-r from-blue-950/60 via-slate-900 to-indigo-950/60 border border-blue-500/30 rounded-2xl p-5 shadow-xl flex flex-col sm:flex-row items-center justify-between gap-4",
                    div { class: "flex items-center space-x-4",
                        span { class: "text-3xl sm:text-4xl", "🗺️" }
                        div {
                            h4 { class: "text-sm sm:text-base font-bold text-blue-300",
                                "{i18n_svc.t(\"ui_next_horizon\", lang)}"
                            }
                            p { class: "text-xs sm:text-sm text-slate-300 mt-0.5",
                                "{zone.next_zone_direction}"
                            }
                        }
                    }
                    div { class: "flex-shrink-0",
                        span { class: "text-xs uppercase tracking-wider font-bold px-3 py-1.5 rounded-full bg-blue-500/20 border border-blue-500/40 text-blue-300",
                            "{i18n_svc.t(\"ui_auto_compass\", lang)}"
                        }
                    }
                }

                // 16 Legacy Points Preset Widget
                LegacyPresetWidget {
                    current_lang: current_lang,
                    i18n: i18n,
                }

                // Next Module Button
                div { class: "w-full max-w-md pt-2",
                    button {
                        class: "w-full py-3 rounded-xl font-bold text-sm uppercase tracking-wider bg-amber-500 hover:bg-amber-400 text-black shadow-lg transition-transform active:scale-95 cursor-pointer text-center",
                        onclick: move |_| on_continue_to_dungeons.call(()),
                        "{i18n_svc.t(\"ui_open_dungeon_codex\", lang)}"
                    }
                }

                if let Some(data) = popover.read().clone() {
                    ReferencePopover { data: data, on_close: move |_| popover.set(None) }
                }
            }

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6",
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}
