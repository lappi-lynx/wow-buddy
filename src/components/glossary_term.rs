use crate::components::reference_popover::{PopoverData, ReferencePopover};
use crate::i18n::{I18nService, Language};
use crate::models::glossary::get_term;
use dioxus::prelude::*;

/// A tap-to-explain chip: renders the localized term and opens the shared
/// reference popover with its one-line explanation.
#[component]
pub fn GlossaryTerm(
    term_id: String,
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
) -> Element {
    let Some(term) = get_term(term_id.as_str()) else {
        return rsx! {};
    };
    let mut open = use_signal(|| false);
    let lang = *current_lang.read();
    let name = i18n.read().t(term.text_key, lang);
    let short = i18n.read().t(term.short_key, lang);
    let title = i18n.read().t("ui_glossary_title", lang);
    let btn_label = name.clone();
    let heading = name;

    rsx! {
        button {
            class: "inline-flex items-center gap-1 px-3 min-h-[44px] rounded-full bg-slate-800/80 border border-slate-700 text-slate-300 hover:text-amber-300 hover:border-amber-500/40 text-xs cursor-pointer",
            onclick: move |_| open.set(true),
            "{btn_label}"
            span { class: "text-[10px] text-slate-500", "?" }
        }
        if open() {
            ReferencePopover {
                data: PopoverData {
                    heading: heading.clone(),
                    subtitle: Some(title.clone()),
                    lines: vec![short.clone()],
                    icon: None,
                },
                on_close: move |_| open.set(false),
            }
        }
    }
}
