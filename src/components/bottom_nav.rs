use crate::i18n::{I18nService, Language};
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavTab {
    DressingRoom,
    Progression,
    Dungeons,
}

#[component]
pub fn BottomNav(
    active_tab: NavTab,
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    on_select_tab: EventHandler<NavTab>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    let tabs = [
        (
            NavTab::DressingRoom,
            "🛡️",
            i18n_svc.t("nav_gear", lang),
        ),
        (NavTab::Progression, "🧭", i18n_svc.t("nav_progression", lang)),
        (NavTab::Dungeons, "🏰", i18n_svc.t("nav_dungeons", lang)),
    ];

    rsx! {
        nav { class: "fixed bottom-0 left-0 right-0 z-40 bg-slate-950/95 border-t border-slate-800/90 backdrop-blur-md px-3 py-2 flex justify-around items-center max-w-lg mx-auto shadow-2xl rounded-t-2xl sm:rounded-none",
            style: "padding-bottom: calc(0.5rem + env(safe-area-inset-bottom))",
            for (tab, icon, label) in tabs {
                {
                    let is_active = active_tab == tab;
                    let active_style = if is_active {
                        "text-amber-400 bg-amber-500/15 border-amber-500/30 font-bold scale-105"
                    } else {
                        "text-slate-400 hover:text-slate-200 border-transparent hover:bg-slate-900"
                    };

                    rsx! {
                        button {
                            key: "{icon}",
                            class: "flex flex-col items-center justify-center py-1.5 px-3 rounded-xl border transition-all cursor-pointer min-w-[72px] sm:min-w-[90px] {active_style}",
                            onclick: move |_| on_select_tab.call(tab),
                            span { class: "text-lg sm:text-xl", "{icon}" }
                            span { class: "text-[10px] sm:text-xs truncate max-w-[80px] mt-0.5", "{label}" }
                        }
                    }
                }
            }
        }
    }
}
