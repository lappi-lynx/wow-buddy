use crate::models::dungeon::DungeonRule;
use dioxus::prelude::*;

#[component]
pub fn DungeonRuleCard(rule: DungeonRule) -> Element {
    rsx! {
        div { class: "flex items-start space-x-3.5 p-4 rounded-2xl bg-slate-900/90 border border-slate-800 shadow-md hover:border-slate-700 transition-colors",
            div { class: "w-10 h-10 rounded-xl bg-slate-950 flex items-center justify-center border border-slate-800 flex-shrink-0 text-2xl shadow-inner",
                "{rule.icon}"
            }
            div { class: "flex flex-col space-y-1",
                h4 { class: "text-sm sm:text-base font-bold text-slate-100",
                    "{rule.title}"
                }
                p { class: "text-xs sm:text-sm text-slate-400 leading-relaxed",
                    "{rule.description}"
                }
            }
        }
    }
}
