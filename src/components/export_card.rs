use crate::i18n::{I18nService, Language};
use crate::models::{ClassInfo, Faction, RaceInfo};
use crate::views::race_selector::format_race_display_name;
use dioxus::prelude::*;

/// Serializes the chosen build into the shared hand-off string.
///
/// The human-readable `FACTION | RACE | CLASS` layout is provisional: KB §5
/// (Phase 2 addon exchange) suggests a Base64/JSON wire format, which is
/// deferred until the addon parser is built. Faction uses an explicit
/// display-name method rather than `Debug` formatting.
pub fn build_export_string(faction: Faction, race_display_name: &str, class_name: &str) -> String {
    format!(
        "FACTION: {} | RACE: {race_display_name} | CLASS: {class_name}",
        faction.display_name()
    )
}

/// Copy `text` to the system clipboard, resolving to whether the copy
/// succeeded. On web the Clipboard API is asynchronous, so the returned
/// promise is awaited: a rejection (denied permission, insecure context, etc.)
/// is surfaced as a failure instead of an optimistic "copied!" toast.
async fn copy_to_clipboard(text: String) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let promise = window.navigator().clipboard().write_text(&text);
            return wasm_bindgen_futures::JsFuture::from(promise).await.is_ok();
        }
        false
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => clipboard.set_text(text).is_ok(),
            Err(_) => false,
        }
    }
}

#[component]
pub fn BlueprintExport(
    faction: Faction,
    race: RaceInfo,
    class: ClassInfo,
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
) -> Element {
    let mut copied = use_signal(|| false);
    let mut copy_failed = use_signal(|| false);

    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    let display_race = format_race_display_name(&race, faction);
    let export_string = build_export_string(faction, &display_race, &class.name);
    let export_string_for_click = export_string.clone();

    rsx! {
        div { class: "w-full max-w-lg bg-gradient-to-br from-slate-900 via-slate-900/90 to-slate-950 border-2 border-amber-500/30 rounded-2xl p-5 shadow-xl flex flex-col space-y-3 relative overflow-hidden",
            div {
                class: match faction {
                    Faction::Alliance => "absolute -right-16 -bottom-16 w-48 h-48 rounded-full blur-3xl opacity-20 bg-blue-500",
                    Faction::Horde => "absolute -right-16 -bottom-16 w-48 h-48 rounded-full blur-3xl opacity-20 bg-red-600",
                }
            }
            div { class: "relative z-10 flex flex-col space-y-3",
                div { class: "flex flex-col space-y-1",
                    span { class: "text-xs font-bold tracking-widest text-slate-500 uppercase",
                        "{i18n_svc.t(\"ui_export_title\", lang)}"
                    }
                    h3 { class: "text-lg sm:text-xl font-extrabold text-white tracking-wide",
                        "{display_race} {class.name}"
                    }
                }

                code { class: "block w-full select-all text-xs sm:text-sm text-amber-200 bg-slate-950/70 border border-slate-800 rounded-xl px-3 py-2 break-all",
                    "{export_string}"
                }

                button {
                    class: "w-full py-3 rounded-xl font-bold text-xs sm:text-sm uppercase tracking-wider bg-slate-800 hover:bg-slate-700 text-amber-200 border border-amber-500/30 shadow-lg transition-transform active:scale-95 cursor-pointer",
                    onclick: move |_| {
                        copied.set(false);
                        copy_failed.set(false);
                        let text = export_string_for_click.clone();
                        spawn(async move {
                            if copy_to_clipboard(text).await {
                                copied.set(true);
                            } else {
                                copy_failed.set(true);
                            }
                        });
                    },
                    "📋 {i18n_svc.t(\"ui_export_plan\", lang)}"
                }

                if *copied.read() {
                    span { class: "text-xs font-medium text-emerald-400",
                        "{i18n_svc.t(\"ui_export_copied\", lang)}"
                    }
                }
                if *copy_failed.read() {
                    span { class: "text-xs font-medium text-amber-400",
                        "{i18n_svc.t(\"ui_export_hint\", lang)}"
                    }
                }
            }
        }
    }
}
