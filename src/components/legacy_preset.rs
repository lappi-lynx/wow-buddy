use crate::i18n::{I18nService, Language};
use crate::models::legacy::{resolve_preset, LegacyCatalog};
use dioxus::prelude::*;

/// Legacy presets: Balanced / Explorer / Coin Saver. The picker shows each
/// preset's point distribution; nothing leaves the app.
#[component]
pub fn LegacyPresetWidget(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let catalog = LegacyCatalog::load_embedded();
    let mut selected = use_signal(|| "balanced".to_string());

    let selected_id = selected.read().clone();
    let resolved = resolve_preset(&catalog, &selected_id);
    let total: u8 = resolved.iter().map(|(_, n)| *n).sum();

    rsx! {
        div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-4",
            div { class: "flex justify-between items-center border-b border-slate-800 pb-3",
                div {
                    h3 { class: "text-base sm:text-lg font-bold text-amber-300",
                        "{i18n_svc.t(\"ui_legacy_preset_title\", lang)}"
                    }
                    p { class: "text-xs text-slate-400 mt-0.5",
                        "{i18n_svc.t(\"ui_legacy_preset_desc\", lang)}"
                    }
                }
                span { class: "text-xs px-2.5 py-1 rounded-full bg-amber-500/20 border border-amber-500/40 text-amber-300 font-bold",
                    "{total} / {LegacyCatalog::SEASONAL_CAP}"
                }
            }

            div { class: "flex flex-wrap items-center gap-2",
                span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                    "{i18n_svc.t(\"ui_legacy_preset_pick\", lang)}"
                }
                for preset in catalog.presets.iter() {
                    {
                        let is_active = preset.id == selected_id;
                        let tab_class = if is_active {
                            "px-3 py-1.5 rounded-xl border text-xs font-bold bg-amber-500/20 border-amber-500/40 text-amber-300 cursor-pointer"
                        } else {
                            "px-3 py-1.5 rounded-xl border text-xs font-semibold bg-slate-950/60 border-slate-800 text-slate-400 hover:text-slate-200 hover:border-slate-700 cursor-pointer"
                        };
                        rsx! {
                            button {
                                key: "{preset.id}",
                                class: "{tab_class}",
                                onclick: {
                                    let id = preset.id.clone();
                                    move |_| selected.set(id.clone())
                                },
                                "{preset.name.get(lang)}"
                            }
                        }
                    }
                }
            }

            div { class: "grid grid-cols-2 sm:grid-cols-3 gap-2",
                for (perk, points) in resolved.iter() {
                    div {
                        key: "{perk.id}",
                        class: "flex flex-col justify-between p-3 rounded-xl bg-slate-950/70 border border-slate-800/80 shadow-inner",
                        div { class: "flex items-center justify-between mb-1.5",
                            span { class: "text-lg", "✦" }
                            span { class: "text-[11px] font-extrabold px-2 py-0.5 rounded-md bg-amber-500/20 text-amber-300 border border-amber-500/30",
                                "{points} pts"
                            }
                        }
                        div {
                            h4 { class: "text-xs font-bold text-slate-200", "{perk.name.en}" }
                            p { class: "text-[10px] text-slate-400 leading-tight mt-0.5", "{perk.desc.get(lang)}" }
                        }
                    }
                }
            }
        }
    }
}
