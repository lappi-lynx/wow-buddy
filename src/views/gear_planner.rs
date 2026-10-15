use crate::assets::AssetRegistry;
use crate::components::reference_popover::{item_popover, PopoverData};
use crate::components::race_tile::class_render_source;
use crate::components::ReferencePopover;
use crate::components::LanguagePicker;
use crate::i18n::{I18nService, Language};
use crate::models::gear_planner::{GearBand, GearPlannerData, GearWeapon, SLOT_ORDER};
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
    let mut popover = use_signal(|| None::<PopoverData>);

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let registry = AssetRegistry::load_embedded();
    let data = GearPlannerData::load_embedded();

    let active_faction = *view_faction.read();
    let bands = data.bands_for(&class_id, active_faction).to_vec();
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
                    class: "flex items-center justify-center w-9 h-9 text-base text-slate-400 hover:text-amber-300 transition-colors rounded-lg border border-slate-800 hover:border-slate-700 bg-slate-900/60 cursor-pointer",
                    onclick: move |_| on_back.call(()),
                    "←"
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
                    {
                        let current_band = data
                            .band_for_level(&class_id, active_faction, *level.read())
                            .or_else(|| data.next_band_for_level(&class_id, active_faction, *level.read()));
                        let class_art = class_render_source(&registry, &class_id);
                        rsx! {
                            div { class: "w-full flex flex-col lg:flex-row gap-4",
                                div { class: "flex flex-col items-center justify-center p-4 rounded-2xl bg-slate-900/80 border border-slate-800 lg:w-52 shrink-0",
                                    if class_art.starts_with("data:") {
                                        img { class: "w-24 h-24 object-contain", src: "{class_art}", alt: "{class_id}" }
                                    } else {
                                        span { class: "text-5xl", "🛡️" }
                                    }
                                    span { class: "text-sm font-bold text-amber-300 mt-2 capitalize", "{class_id}" }
                                    if let Some(band) = current_band.as_ref() {
                                        span { class: "text-xs text-slate-500 mt-1 text-center", "▤ {band.armor.text}" }
                                    }
                                }
                                if let Some(band) = current_band.as_ref() {
                                    div { class: "flex-1 flex flex-col space-y-2",
                                        div { class: "grid grid-cols-3 sm:grid-cols-4 gap-2",
                                            if let Some(weapon) = band.weapons.first() {
                                                { slot_tile("weapon", "weapon", Some(weapon), &registry, i18n, lang, popover) }
                                            }
                                            for slot in SLOT_ORDER.iter() {
                                                { slot_tile(slot, slot, band.slots.get(*slot), &registry, i18n, lang, popover) }
                                            }
                                        }
                                        if band.weapons.len() > 1 {
                                            div { class: "flex flex-wrap gap-1.5",
                                                for weapon in band.weapons.iter().skip(1) {
                                                    span { key: "{weapon.name}", class: "text-[10px] px-2 py-0.5 rounded-full bg-slate-900/70 border border-slate-800 text-slate-400",
                                                        "{weapon.name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(data) = popover.read().clone() {
                ReferencePopover { data: data, on_close: move |_| popover.set(None) }
            }

            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-600 border-t border-slate-900 mt-6 flex flex-col items-center space-y-2",
                LanguagePicker { current_lang: current_lang }
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}

/// One equipment tile: icon + slot label + item name/level; tapping an item
/// opens the shared reference popover.
fn slot_tile(
    key: &str,
    label: &str,
    item: Option<&GearWeapon>,
    registry: &AssetRegistry,
    i18n: Signal<I18nService>,
    lang: Language,
    popover: Signal<Option<PopoverData>>,
) -> Element {
    let (name, uri, source, lvl, icon_opt) = match item {
        Some(it) => {
            let uri = registry
                .item_icon(it.icon_id.as_deref().unwrap_or(""))
                .to_string();
            (
                it.name.clone(),
                uri,
                it.source.clone(),
                it.level,
                it.icon_id.clone(),
            )
        }
        None => (String::new(), String::new(), String::new(), 0u8, None),
    };
    let has_item = item.is_some();
    let tile_class = if has_item {
        "flex flex-col items-center gap-1 p-2 rounded-xl bg-slate-900/80 border border-slate-800 hover:border-amber-500/40 cursor-pointer min-h-[92px]"
    } else {
        "flex flex-col items-center gap-1 p-2 rounded-xl bg-slate-950/40 border border-slate-900 min-h-[92px] cursor-default"
    };
    let reg = registry.clone();
    let name_for_pop = name.clone();
    let mut popover = popover;
    rsx! {
        button {
            key: "{key}",
            class: "{tile_class}",
            onclick: move |_| {
                if !name_for_pop.is_empty() {
                    let i18n_ref = i18n.read();
                    popover.set(Some(item_popover(&reg, &i18n_ref, lang, &name_for_pop, icon_opt.as_deref(), &source)));
                }
            },
            if has_item {
                if uri.starts_with("data:") {
                    img { class: "w-8 h-8 object-contain", src: "{uri}", alt: "" }
                } else {
                    span { class: "text-xl", "{uri}" }
                }
            } else {
                span { class: "text-xl text-slate-700", "—" }
            }
            span { class: "text-[9px] uppercase tracking-wider text-slate-500", "{label}" }
            if has_item {
                span { class: "text-[10px] font-semibold text-slate-200 text-center leading-tight", "{name}" }
                span { class: "text-[9px] text-amber-500", "Lvl {lvl}" }
            }
        }
    }
}
