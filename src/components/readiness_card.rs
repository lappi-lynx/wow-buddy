use std::collections::HashSet;

use crate::i18n::{I18nService, Language};
use crate::models::dungeon::dungeons_for_level;
use crate::models::readiness::{readiness_level, ReadinessData, ReadinessLevel};
use dioxus::prelude::*;

/// Pre-dungeon checklist with a Not-ready/Almost/Ready meter, a level-aware
/// dungeon list, and the (KB-verified) how-to-find-a-group card — compact:
/// dense pills, the whole pill toggles, no checkbox glyphs.
#[component]
pub fn ReadinessCard(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = ReadinessData::load_embedded();
    let mut checked = use_signal(HashSet::<String>::new);
    let mut level = use_signal(|| 15u8);
    let mut group_open = use_signal(|| false);

    let total = data.items.len() as u8;
    let checked_count = checked.read().len() as u8;
    let state = readiness_level(checked_count, total, data.threshold_almost);
    let (state_key, state_class) = match state {
        ReadinessLevel::NotReady => (
            "ui_readiness_not_ready",
            "bg-red-500/15 border-red-500/40 text-red-300",
        ),
        ReadinessLevel::Almost => (
            "ui_readiness_almost",
            "bg-amber-500/15 border-amber-500/40 text-amber-300",
        ),
        ReadinessLevel::Ready => (
            "ui_readiness_ready",
            "bg-emerald-500/15 border-emerald-500/40 text-emerald-300",
        ),
    };
    let meter_pct = if total == 0 {
        0.0
    } else {
        checked_count as f32 / total as f32 * 100.0
    };
    let level_now = *level.read();
    let in_range = dungeons_for_level(level_now);

    rsx! {
        div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-4 sm:p-5 shadow-xl flex flex-col space-y-3",
            div { class: "flex justify-between items-center",
                h3 { class: "text-base sm:text-lg font-bold text-amber-300",
                    "✦ {i18n_svc.t(\"ui_readiness_title\", lang)}"
                }
                span { class: "text-xs px-2.5 py-1 rounded-full border font-bold {state_class}",
                    "{checked_count} / {total} · {i18n_svc.t(state_key, lang)}"
                }
            }

            div {
                style: "height: 4px; background: #1e293b; border-radius: 9999px; overflow: hidden;",
                div {
                    style: "height: 4px; width: {meter_pct:.0}%; background: #f59e0b; border-radius: 9999px;",
                }
            }

            div { class: "flex flex-wrap gap-1.5",
                for item in data.items.iter() {
                    {
                        let is_checked = checked.read().contains(&item.id);
                        let pill_class = if is_checked {
                            "px-3 py-1.5 rounded-full border text-xs font-semibold bg-emerald-950/40 border-emerald-800/60 text-slate-200 cursor-pointer"
                        } else {
                            "px-3 py-1.5 rounded-full border text-xs font-semibold bg-slate-950/60 border-slate-800 text-slate-400 hover:text-slate-200 cursor-pointer"
                        };
                        rsx! {
                            button {
                                key: "{item.id}",
                                class: "{pill_class}",
                                onclick: {
                                    let id = item.id.clone();
                                    move |_| {
                                        let mut set = checked.write();
                                        if set.contains(&id) {
                                            set.remove(&id);
                                        } else {
                                            set.insert(id.clone());
                                        }
                                    }
                                },
                                "{item.text.get(lang)}"
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-col sm:flex-row sm:items-center gap-2 pt-1",
                label { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                    "{i18n_svc.t(\"ui_level_range\", lang)}"
                }
                input {
                    r#type: "number",
                    min: "1",
                    max: "60",
                    value: "{level_now}",
                    class: "w-20 px-3 py-1.5 rounded-xl bg-slate-950/60 border border-slate-800 text-slate-200 text-sm",
                    oninput: move |evt: Event<FormData>| {
                        if let Ok(v) = evt.value().parse::<u8>() {
                            level.set(v.clamp(1, 60));
                        }
                    },
                }
                div { class: "flex flex-wrap items-center gap-2",
                    for dungeon in in_range.iter() {
                        span { key: "{dungeon.id}", class: "text-xs px-2 py-1 rounded-lg bg-slate-800 border border-slate-700 text-slate-200",
                            "{dungeon.icon} {dungeon.name} ({dungeon.level_range})"
                        }
                    }
                    button {
                        class: "text-xs font-semibold text-blue-300 hover:text-blue-200 cursor-pointer",
                        onclick: move |_| {
                            let open = *group_open.read();
                            group_open.set(!open);
                        },
                        "❔ {i18n_svc.t(\"ui_find_group\", lang)}"
                    }
                }
            }

            if *group_open.read() {
                div { class: "p-3.5 rounded-xl bg-slate-950/70 border border-slate-800/80 flex flex-col space-y-1",
                    span { class: "text-xs font-bold text-amber-300",
                        "{i18n_svc.t(\"ui_find_group\", lang)}"
                    }
                    p { class: "text-xs text-slate-300", "{data.group_finding.text.get(lang)}" }
                    if !data.group_finding.verified {
                        span { class: "text-[10px] font-bold uppercase tracking-wider text-amber-400/80",
                            "{i18n_svc.t(\"ui_beta_may_change\", lang)}"
                        }
                    }
                }
            }
        }
    }
}
