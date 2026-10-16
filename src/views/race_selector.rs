use crate::assets::AssetRegistry;
use crate::components::race_tile::class_render_source;
use crate::components::{AppFooter, FactionTheme, RaceTile};
use crate::i18n::{I18nService, Language};
use crate::models::{ClassInfo, Faction, GameData, RaceInfo};
use crate::selection::effective_class_id;
use dioxus::prelude::*;

pub fn format_race_display_name(race: &RaceInfo, faction: Faction) -> String {
    if race.is_dual_faction || race.id == "skyborne" {
        match faction {
            Faction::Alliance => race
                .alliance_variant_name
                .clone()
                .unwrap_or_else(|| "High Order Skyborne".to_string()),
            Faction::Horde => race
                .horde_variant_name
                .clone()
                .unwrap_or_else(|| "Windshaper Skyborne".to_string()),
        }
    } else {
        race.name.clone()
    }
}

/// The race id to show as active: the current selection when it exists in
/// this faction's list, otherwise the first race. Guards against a stale
/// selection carried over from the other faction list.
pub fn effective_race_id(races: &[RaceInfo], current: Option<&str>) -> Option<String> {
    current
        .filter(|id| races.iter().any(|r| r.id == *id))
        .map(str::to_string)
        .or_else(|| races.first().map(|r| r.id.clone()))
}

#[component]
pub fn RaceSelectorView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    game_data: Signal<GameData>,
    faction: Faction,
    selected_race_id: Signal<Option<String>>,
    selected_class: Signal<Option<ClassInfo>>,
    on_race_selected: EventHandler<RaceInfo>,
) -> Element {
    let lang = *current_lang.read();
    let data = game_data.read();
    let races: Vec<RaceInfo> = data
        .get_races_for_faction(faction)
        .into_iter()
        .cloned()
        .collect();

    // The active race: the current selection when it belongs to this
    // faction's list, otherwise the first race (a stale id from the other
    // faction's list must not blank out the philosophy card).
    let current_id =
        effective_race_id(&races, selected_race_id.read().as_deref()).unwrap_or_default();

    let active_race = races.iter().find(|r| r.id == current_id).cloned();

    let faction_theme = FactionTheme::for_faction(faction);
    let heading_classes = faction_theme.accent_heading;

    let faction_title = if faction == Faction::Alliance {
        format!("{} Races", i18n.read().t("alliance_name", lang))
    } else {
        format!("{} Races", i18n.read().t("horde_name", lang))
    };

    let class_registry = AssetRegistry::load_embedded();
    let (active_name, active_lore, active_ability_name, active_ability_desc, available_classes) =
        if let Some(ref r) = active_race {
            let name = format_race_display_name(r, faction);
            let lore = match lang {
                Language::Ru => &r.lore_ru,
                Language::En => &r.lore_en,
                Language::Fi => &r.lore_fi,
            };
            (
                name,
                lore.clone(),
                r.racial_ability_name.clone(),
                r.racial_ability_desc.clone(),
                r.classes_for_faction(faction).to_vec(),
            )
        } else {
            (
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                vec![],
            )
        };

    let selected_class_id = selected_class.read().as_ref().map(|c| c.id.clone());
    let effective_class = active_race
        .as_ref()
        .and_then(|r| effective_class_id(faction, r, selected_class_id.as_deref()));

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",

            main { class: "w-full max-w-4xl mx-auto flex flex-col items-center flex-grow",
                div { class: "text-center mb-6",
                    h2 { class: "text-2xl sm:text-3xl font-extrabold tracking-wide uppercase {heading_classes}",
                        "{faction_title}"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "{i18n.read().t(\"ui_race_selector_subtitle\", lang)}"
                    }
                }

                // Grid of 5 races
                div { class: "grid grid-cols-2 sm:grid-cols-5 gap-3 sm:gap-4 w-full mb-8 justify-items-center",
                    for race in races.iter() {
                        {
                            let r = race.clone();
                            let is_sel = r.id == current_id;
                            let disp_name = format_race_display_name(&r, faction);
                            rsx! {
                                RaceTile {
                                    key: "{r.id}",
                                    race: r.clone(),
                                    faction: faction,
                                    is_selected: is_sel,
                                    display_name: disp_name,
                                    on_select: move |selected: RaceInfo| {
                                        selected_race_id.set(Some(selected.id.clone()));
                                    },
                                }
                            }
                        }
                    }
                }

                // Selected Race Philosophy Card
                if let Some(race_obj) = active_race {
                    div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-6 sm:p-8 shadow-2xl backdrop-blur-md flex flex-col space-y-4 max-w-3xl",
                        div { class: "flex justify-between items-start border-b border-slate-800 pb-3",
                            div {
                                h3 { class: "text-xl sm:text-2xl font-bold text-amber-300", "{active_name}" }
                                p { class: "text-xs text-slate-400 mt-0.5", "{i18n.read().t(\"ui_racial_culture_heritage\", lang)}" }
                            }
                            div { class: "flex flex-col items-end",
                                span { class: "text-xs uppercase tracking-wider text-amber-500/80 font-semibold", "Racial Ability" }
                                span { class: "text-sm font-bold text-slate-200", "⚡ {active_ability_name}" }
                            }
                        }

                        p { class: "text-sm sm:text-base text-slate-300 leading-relaxed italic",
                            "\"{active_lore}\""
                        }

                        div { class: "bg-slate-950/60 p-3 rounded-xl border border-slate-800/80 text-xs sm:text-sm text-slate-300",
                            span { class: "font-semibold text-amber-400 mr-2", "{i18n.read().t(\"ui_effect\", lang)}" }
                            "{active_ability_desc}"
                        }

                        div { class: "pt-2",
                            span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider block mb-2 text-center", "{i18n.read().t(\"ui_available_classes\", lang)}" }
                            div { class: "flex flex-wrap gap-2 justify-center",
                                for cls in available_classes.iter() {
                                    {
                                        let cls_id = cls.clone();
                                        let is_active = effective_class.as_deref() == Some(cls_id.as_str());
                                        let badge_class = if is_active {
                                            "flex flex-col items-center gap-1 p-2 w-[74px] rounded-xl bg-amber-500/20 border-2 border-amber-400 cursor-pointer"
                                        } else {
                                            "flex flex-col items-center gap-1 p-2 w-[74px] rounded-xl bg-slate-800/60 border-2 border-slate-700 cursor-pointer hover:border-slate-500"
                                        };
                                        let label_class = if is_active {
                                            "text-[9px] capitalize font-bold text-amber-300"
                                        } else {
                                            "text-[9px] capitalize font-medium text-slate-400"
                                        };
                                        let art = class_render_source(&class_registry, &cls_id);
                                        rsx! {
                                            button {
                                                key: "{cls_id}",
                                                class: "{badge_class}",
                                                onclick: {
                                                    let race_for_nav = race_obj.clone();
                                                    move |_| {
                                                        if let Some(cls_obj) = game_data.read().get_class_by_id(&cls_id) {
                                                            selected_class.set(Some(cls_obj.clone()));
                                                        }
                                                        // Clicking a class is the confirmation: race + class
                                                        // are enough to move on (no extra Choose button).
                                                        on_race_selected.call(race_for_nav.clone());
                                                    }
                                                },
                                                if art.starts_with("data:") {
                                                    img { class: "w-9 h-9 object-contain rounded-md", src: "{art}", alt: "{cls_id}" }
                                                } else {
                                                    span { class: "text-xl", "🛡️" }
                                                }
                                                span { class: "{label_class}", "{cls_id}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            AppFooter { current_lang: current_lang, i18n: i18n }        }
    }
}
