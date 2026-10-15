use crate::i18n::{Language, LanguageStorage};
use dioxus::prelude::*;

#[component]
pub fn LanguagePicker(
    current_lang: Signal<Language>,
    mut on_change: Option<EventHandler<Language>>,
) -> Element {
    let on_select = move |evt: Event<FormData>| {
        let val = evt.value();
        if let Some(selected) = Language::from_code(&val) {
            current_lang.set(selected);
            let mut storage = LanguageStorage::default();
            storage.set_language(selected);
            if let Some(ref mut handler) = on_change {
                handler.call(selected);
            }
        }
    };

    rsx! {
        div { class: "relative inline-block",
            select {
                class: "bg-slate-900 text-amber-300 border border-amber-600/30 rounded-md px-2 py-1 text-xs font-semibold focus:outline-none focus:ring-2 focus:ring-amber-500 cursor-pointer transition",
                value: "{current_lang.read().code()}",
                onchange: on_select,
                for lang in Language::all() {
                    option {
                        key: "{lang.code()}",
                        value: "{lang.code()}",
                        selected: *lang == *current_lang.read(),
                        "{lang.code().to_uppercase()}"
                    }
                }
            }
        }
    }
}
