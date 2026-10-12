use crate::i18n::{I18nService, Language};
use crate::models::legacy::{get_starter_legacy_preset, LegacyPreset};
use dioxus::prelude::*;

#[component]
pub fn LegacyPresetWidget(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    on_applied: Option<EventHandler<LegacyPreset>>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let preset = get_starter_legacy_preset();
    let mut is_applied = use_signal(|| false);

    let preset_clone = preset.clone();
    let apply_handler = move |_| {
        is_applied.set(true);
        if let Some(ref handler) = on_applied {
            handler.call(preset_clone.clone());
        }
    };

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
                    "{i18n_svc.t(\"ui_legacy_points\", lang)}"
                }
            }

            div { class: "grid grid-cols-1 sm:grid-cols-3 gap-3",
                for perk in preset.perks.iter() {
                    div {
                        key: "{perk.id}",
                        class: "flex flex-col justify-between p-3.5 rounded-xl bg-slate-950/70 border border-slate-800/80 shadow-inner",
                        div { class: "flex items-center justify-between mb-2",
                            span { class: "text-2xl", "{perk.icon}" }
                            span { class: "text-xs font-extrabold px-2 py-0.5 rounded-md bg-amber-500/20 text-amber-300 border border-amber-500/30",
                                "{perk.points} pts"
                            }
                        }
                        div {
                            h4 { class: "text-xs sm:text-sm font-bold text-slate-200", "{perk.name}" }
                            p { class: "text-[11px] text-slate-400 leading-tight mt-1", "{perk.description}" }
                        }
                    }
                }
            }

            div { class: "pt-2 flex flex-col sm:flex-row items-center justify-between gap-3",
                button {
                    class: "w-full sm:w-auto flex-1 py-2.5 px-4 rounded-xl font-bold text-xs sm:text-sm uppercase tracking-wider bg-amber-500 hover:bg-amber-400 text-black shadow-lg transition-transform active:scale-95 cursor-pointer text-center",
                    onclick: apply_handler,
                    if *is_applied.read() {
                        "{i18n_svc.t(\"ui_apply_legacy_applied\", lang)}"
                    } else {
                        "{i18n_svc.t(\"ui_apply_legacy\", lang)}"
                    }
                }
                button {
                    class: "w-full sm:w-auto py-2.5 px-4 rounded-xl font-semibold text-xs text-slate-400 hover:text-slate-200 border border-slate-800 hover:border-slate-700 bg-slate-950/40 transition-colors cursor-pointer",
                    "{i18n_svc.t(\"ui_manual_tweak\", lang)}"
                }
            }
        }
    }
}
