use crate::components::{AppFooter, FactionShield};
use crate::i18n::{I18nService, Language};
pub use crate::models::faction::get_faction_meta;
use crate::models::Faction;
use dioxus::prelude::*;

#[component]
pub fn FactionPickerView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    on_open_about: EventHandler<()>,
    on_faction_selected: EventHandler<Faction>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    let alliance_meta = get_faction_meta(Faction::Alliance);
    let horde_meta = get_faction_meta(Faction::Horde);

    let alliance_name = i18n_svc.t(&alliance_meta.name_key, lang);
    let alliance_motto = i18n_svc.t(&alliance_meta.motto_key, lang);

    let horde_name = i18n_svc.t(&horde_meta.name_key, lang);
    let horde_motto = i18n_svc.t(&horde_meta.motto_key, lang);

    let skyborne_notice = i18n_svc.t("skyborne_notice", lang);

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col items-center justify-between p-4 sm:p-8 select-none",
            header { class: "w-full max-w-5xl flex justify-between items-center py-4 border-b border-slate-800/80 mb-6",
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "⚔️" }
                    h1 { class: "text-lg sm:text-xl font-bold tracking-wider text-amber-400 uppercase",
                        "{i18n_svc.t(\"app_name\", lang)}"
                    }
                }
                button {
                    class: "text-xs font-semibold uppercase tracking-wider text-slate-500 hover:text-amber-300 transition-colors cursor-pointer",
                    onclick: move |_| on_open_about.call(()),
                    "{i18n_svc.t(\"btn_about\", lang)}"
                }
            }

            main { class: "w-full max-w-4xl flex flex-col items-center flex-grow justify-center my-4",
                h2 { class: "text-2xl sm:text-4xl font-extrabold text-center text-amber-200 tracking-wide uppercase drop-shadow-md mb-2",
                    "{i18n_svc.t(\"ui_choose_allegiance\", lang)}"
                }
                p { class: "text-sm sm:text-base text-slate-400 text-center max-w-lg mb-8",
                    "{i18n_svc.t(\"ui_faction_subtitle\", lang)}"
                }

                div { class: "grid grid-cols-2 gap-4 w-full justify-items-center mb-8",
                    FactionShield {
                        meta: alliance_meta,
                        title: alliance_name,
                        motto: alliance_motto,
                        btn_text: i18n_svc.t("select", lang),
                        on_select: move |f| on_faction_selected.call(f),
                    }
                    FactionShield {
                        meta: horde_meta,
                        title: horde_name,
                        motto: horde_motto,
                        btn_text: i18n_svc.t("select", lang),
                        on_select: move |f| on_faction_selected.call(f),
                    }
                }

                div { class: "w-full max-w-2xl bg-amber-950/40 border border-amber-600/30 rounded-xl p-4 text-center shadow-lg backdrop-blur-sm",
                    p { class: "text-xs sm:text-sm text-amber-200 font-medium leading-relaxed",
                        "✨ {skyborne_notice}"
                    }
                }
            }

            AppFooter { current_lang: current_lang, i18n: i18n }        }
    }
}
