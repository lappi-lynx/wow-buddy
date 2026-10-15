use crate::i18n::{I18nService, Language};
use crate::models::comfort_tips::ComfortTipsData;
use dioxus::prelude::*;

/// The comfort tips deck: one idea per card, prev/next through the deck,
/// with a current/total counter.
#[component]
pub fn ComfortDeck(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = ComfortTipsData::load_embedded();
    let total = data.tips.len();
    let mut index = use_signal(|| 0usize);

    let current = (*index.read()).min(total.saturating_sub(1));
    let tip = &data.tips[current];

    rsx! {
        div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-3",
            div { class: "flex justify-between items-center border-b border-slate-800 pb-3",
                h3 { class: "text-base sm:text-lg font-bold text-amber-300",
                    "💡 {i18n_svc.t(\"ui_comfort_title\", lang)}"
                }
                span { class: "text-xs font-bold px-2.5 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-300",
                    "{i18n_svc.t_with(\"ui_comfort_counter\", lang, &[(\"current\", (current + 1).to_string()), (\"total\", total.to_string())])}"
                }
            }
            p { key: "{tip.id}", class: "text-sm text-slate-200 min-h-[3rem]", "{tip.text.get(lang)}" }
            div { class: "flex justify-between gap-3 pt-1",
                button {
                    class: "flex-1 py-2 rounded-xl font-bold text-xs uppercase tracking-wider bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 cursor-pointer disabled:opacity-40",
                    disabled: current == 0,
                    onclick: move |_| {
                        let next = index.read().saturating_sub(1);
                        index.set(next);
                    },
                    "←"
                }
                button {
                    class: "flex-1 py-2 rounded-xl font-bold text-xs uppercase tracking-wider bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 cursor-pointer disabled:opacity-40",
                    disabled: current + 1 >= total,
                    onclick: move |_| {
                        let next = (*index.read() + 1).min(total.saturating_sub(1));
                        index.set(next);
                    },
                    "→"
                }
            }
        }
    }
}
