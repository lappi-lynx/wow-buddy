use crate::assets::AssetRegistry;
use crate::i18n::{I18nService, Language};
use crate::models::reference::NpcBook;
use dioxus::prelude::*;

/// Content of a reference popover (NPC location or item source).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverData {
    pub heading: String,
    pub subtitle: Option<String>,
    pub lines: Vec<String>,
    pub icon: Option<String>,
}

/// Build the popover content for a curated NPC, or `None` when the id is
/// unknown — callers keep rendering the plain name.
pub fn npc_popover(
    book: &NpcBook,
    i18n: &I18nService,
    lang: Language,
    npc_id: &str,
) -> Option<PopoverData> {
    let npc = book.get(npc_id)?;
    Some(PopoverData {
        heading: npc.name.clone(),
        subtitle: None,
        lines: vec![
            format!("{}: {}", i18n.t("ui_npc_zone", lang), npc.zone),
            format!("{}: {}", i18n.t("ui_npc_subzone", lang), npc.subzone),
        ],
        icon: None,
    })
}

/// Build the popover content for an item reference. The icon is optional:
/// generic items (e.g. a starter club) have none and still open the panel.
pub fn item_popover(
    registry: &AssetRegistry,
    i18n: &I18nService,
    lang: Language,
    name: &str,
    icon_id: Option<&str>,
    source: &str,
) -> PopoverData {
    let icon = icon_id
        .map(|id| registry.item_icon(id))
        .filter(|uri| uri.starts_with("data:"))
        .map(str::to_string);
    PopoverData {
        heading: name.to_string(),
        subtitle: None,
        lines: vec![format!("{}: {}", i18n.t("ui_item_source", lang), source)],
        icon,
    }
}

/// A small reference panel opened by tapping an NPC or item name; the full
/// backdrop closes it.
#[component]
pub fn ReferencePopover(data: PopoverData, on_close: EventHandler<()>) -> Element {
    rsx! {
        div { class: "fixed inset-0 z-50 flex items-end sm:items-center justify-center",
            button {
                class: "absolute inset-0 bg-black/60 cursor-default",
                onclick: move |_| on_close.call(()),
            }
            div { class: "relative z-10 w-full sm:max-w-sm m-3 sm:m-0 rounded-2xl bg-slate-900 border border-slate-700 shadow-2xl p-5 flex flex-col space-y-3",
                div { class: "flex items-start justify-between space-x-3",
                    div { class: "flex items-center space-x-3",
                        if let Some(icon) = data.icon.clone() {
                            img { class: "w-12 h-12 rounded-lg", src: "{icon}", alt: "{data.heading}" }
                        }
                        h3 { class: "text-base sm:text-lg font-bold text-amber-300", "{data.heading}" }
                    }
                    button {
                        class: "text-slate-400 hover:text-slate-200 cursor-pointer text-sm px-2 py-0.5",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }
                if let Some(subtitle) = data.subtitle.clone() {
                    p { class: "text-xs text-slate-400", "{subtitle}" }
                }
                div { class: "flex flex-col space-y-1",
                    for line in data.lines.iter() {
                        p { key: "{line}", class: "text-xs sm:text-sm text-slate-300", "{line}" }
                    }
                }
            }
        }
    }
}
