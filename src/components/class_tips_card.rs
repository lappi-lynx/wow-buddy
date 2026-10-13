use std::collections::HashSet;

use crate::i18n::{I18nService, Language};
use crate::models::class_tips::{ClassTipsData, SpecTips};
use dioxus::prelude::*;

/// The spec cards to render for a class: all authored specs, or none.
pub fn spec_cards(data: &ClassTipsData, class_id: &str) -> Vec<SpecTips> {
    data.specs_for(class_id).to_vec()
}

/// "New Player Tips" — all of a class's specializations at once (variant b),
/// each collapsible, plus the shared spec info tip.
#[component]
pub fn ClassTipsCard(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    class_id: String,
) -> Element {
    let mut expanded = use_signal(HashSet::<String>::new);

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = ClassTipsData::load_embedded();

    let specs = spec_cards(&data, &class_id);
    let spec_info = data.spec_info.as_ref().map(|t| t.get(lang).to_string());

    rsx! {
        div { class: "w-full max-w-lg bg-slate-900/90 border border-slate-800 rounded-2xl p-5 shadow-xl flex flex-col space-y-4",
            div { class: "flex items-center justify-between border-b border-slate-800 pb-3",
                h3 { class: "text-base sm:text-lg font-bold text-amber-300",
                    "{i18n_svc.t(\"ui_tips_title\", lang)}"
                }
            }

            if let Some(info) = spec_info {
                div { class: "bg-slate-950/60 p-3 rounded-xl border border-slate-800/80",
                    div { class: "text-[11px] uppercase tracking-wider font-semibold text-amber-400 mb-1",
                        "{i18n_svc.t(\"ui_specs_info_title\", lang)}"
                    }
                    div { class: "text-xs text-slate-300 leading-relaxed",
                        span { class: "font-semibold text-amber-400 mr-1",
                            "{i18n_svc.t(\"ui_specs_info_label\", lang)}"
                        }
                        span { "{info}" }
                    }
                }
            }

            if specs.is_empty() {
                div { class: "text-xs text-slate-400",
                    "{i18n_svc.t(\"ui_tips_empty\", lang)}"
                }
            } else {
                div { class: "flex flex-col space-y-2",
                    for spec in specs.iter() {
                        {
                            let spec_id = spec.id.clone();
                            let toggle_id = spec_id.clone();
                            let title = spec.name_en.clone();
                            let is_open = expanded.read().contains(&spec_id);
                            let tips: Vec<String> =
                                spec.tips.iter().map(|t| t.get(lang).to_string()).collect();
                            rsx! {
                                div {
                                    key: "{spec_id}",
                                    class: "rounded-xl border border-slate-800 bg-slate-950/50",
                                    button {
                                        class: "w-full flex items-center justify-between px-3 py-2 text-left text-sm font-semibold text-slate-200 cursor-pointer",
                                        onclick: move |_| {
                                            let mut set = expanded.write();
                                            if set.contains(&toggle_id) {
                                                set.remove(&toggle_id);
                                            } else {
                                                set.insert(toggle_id.clone());
                                            }
                                        },
                                        span { "{title}" }
                                        span { "▾" }
                                    }
                                    if is_open {
                                        ul { class: "px-3 pb-3 flex flex-col space-y-1",
                                            for tip in tips.iter() {
                                                li {
                                                    key: "{tip}",
                                                    class: "text-xs text-slate-400 leading-relaxed",
                                                    "• {tip}"
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
    }
}
