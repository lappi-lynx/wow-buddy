use crate::components::LanguagePicker;
use crate::i18n::{I18nService, Language};
use dioxus::prelude::*;

/// The single-line footer shared by every view: the guide name on the left and
/// the compact EN/FI/RU language selector on the right.
#[component]
pub fn AppFooter(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    rsx! {
        footer { class: "w-full max-w-5xl mx-auto flex items-center justify-between gap-3 py-3 text-xs text-slate-500 border-t border-slate-900 mt-6",
            span { class: "truncate", "{i18n_svc.t(\"footer_guide\", lang)}" }
            div { class: "shrink-0",
                LanguagePicker { current_lang: current_lang }
            }
        }
    }
}
