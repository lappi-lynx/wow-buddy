use crate::assets::AssetRegistry;
use crate::i18n::{I18nService, Language};
use crate::models::gear_planner::{GearBand, GearPlannerData};
use crate::models::Faction;
use dioxus::prelude::*;

/// The band id to highlight for `level`: the containing band if one exists,
/// otherwise the next band above it (used when the level is in a gap).
pub fn band_highlight(bands: &[GearBand], level: u8) -> Option<String> {
    if let Some(band) = bands
        .iter()
        .find(|b| level >= b.min_level && level <= b.max_level)
    {
        return Some(band.id.clone());
    }
    bands
        .iter()
        .filter(|b| b.min_level > level)
        .min_by_key(|b| b.min_level)
        .map(|b| b.id.clone())
}

#[component]
pub fn GearPlannerView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    faction: Faction,
    class_id: String,
    on_back: EventHandler<()>,
) -> Element {
    let mut level = use_signal(|| 20u8);
    let mut view_faction = use_signal(|| faction);

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let registry = AssetRegistry::load_embedded();
    let data = GearPlannerData::load_embedded();

    let active_faction = *view_faction.read();
    let bands = data.bands_for(&class_id, active_faction).to_vec();
    let highlight = band_highlight(&bands, *level.read());
    let has_data = !bands.is_empty();

    let alliance_name = i18n_svc.t("alliance_name", lang);
    let horde_name = i18n_svc.t("horde_name", lang);
    let active_chip = "text-xs px-2.5 py-1 rounded-full bg-amber-500/20 text-amber-300 font-bold cursor-pointer";
    let idle_chip = "text-xs px-2.5 py-1 rounded-full bg-slate-900/60 border border-slate-800 text-slate-400 font-semibold cursor-pointer";
    let alliance_class = if active_faction == Faction::Alliance { active_chip } else { idle_chip };
    let horde_class = if active_faction == Faction::Horde { active_chip } else { idle_chip };

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
                    button {
                        class: "{alliance_class}",
                        onclick: move |_| view_faction.set(Faction::Alliance),
                        "{alliance_name}"
                    }
                    button {
                        class: "{horde_class}",
                        onclick: move |_| view_faction.set(Faction::Horde),
                        "{horde_name}"
                    }
                }
            }

            main { class: "w-full max-w-3xl mx-auto flex flex-col flex-grow space-y-5",
                div { class: "text-center",
                    h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide",
                        "{i18n_svc.t(\"ui_gear_planner_title\", lang)}"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "{i18n_svc.t(\"ui_gear_planner_subtitle\", lang)}"
                    }
                }

                div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-4 flex flex-col space-y-2",
                    span { class: "text-xs uppercase tracking-wider font-semibold text-slate-400",
                        "{i18n_svc.t_with(\"ui_gear_evolution\", lang, &[(\"level\", level.read().to_string())])}"
                    }
                    input {
                        r#type: "range",
                        min: "1",
                        max: "60",
                        step: "1",
                        value: "{level.read()}",
                        class: "w-full accent-amber-500 cursor-pointer h-2 bg-slate-800 rounded-lg",
                        oninput: move |evt: Event<FormData>| {
                            if let Ok(lvl) = evt.value().parse::<u8>() {
                                level.set(lvl);
                            }
                        },
                    }
                }

                if !has_data {
                    div { class: "p-6 rounded-2xl bg-slate-900/80 border border-slate-800 text-center text-slate-400 text-sm",
                        "{i18n_svc.t(\"ui_gear_no_data\", lang)}"
                    }
                } else {
                    div { class: "border-l-2 border-slate-800 pl-4 flex flex-col space-y-6",
                        for band in bands.iter() {
                            {
                                let is_current = highlight.as_deref() == Some(band.id.as_str());
                                let current_label = i18n_svc.t("ui_gear_current", lang);
                                let band_class = if is_current {
                                    "border-l-2 border-amber-500 pl-3 -ml-[18px]"
                                } else {
                                    "border-l-2 border-transparent pl-3 -ml-[18px]"
                                };
                                rsx! {
                                    div {
                                        key: "{band.id}",
                                        class: "{band_class}",
                                        div { class: "flex items-center space-x-2",
                                            span { class: "text-sm font-bold text-amber-400", "{band.label}" }
                                            if is_current {
                                                span { class: "text-xs font-semibold text-emerald-400", "★ {current_label}" }
                                            }
                                        }
                                        div { class: "mt-1 flex flex-col space-y-1",
                                            for weapon in band.weapons.iter() {
                                                div {
                                                    key: "{weapon.name}",
                                                    class: "flex items-center space-x-2 text-xs sm:text-sm text-slate-200",
                                                    span { class: "text-base", "{registry.item_icon(weapon.icon_id.as_deref().unwrap_or(\"\"))}" }
                                                    span { class: "text-amber-500 font-semibold", "Lvl {weapon.level}" }
                                                    span { class: "font-medium", "{weapon.name}" }
                                                    span { class: "text-slate-500", "— {weapon.source}" }
                                                }
                                            }
                                            div { class: "text-xs text-slate-400", "▤ {band.armor.text}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6",
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}
