use crate::components::AppFooter;
use crate::i18n::{I18nService, Language};
use dioxus::prelude::*;

/// The required World of Warcraft Fan Site Kit disclaimer — verbatim from
/// `assets/ATTRIBUTION.md` (enforced by `tests/about_view_test.rs`).
pub const REQUIRED_DISCLAIMER_LINE: &str = "World of Warcraft® and Blizzard Entertainment® are all trademarks or registered trademarks of Blizzard Entertainment in the United States and/or other countries. These terms and all related materials, logos, and images are copyright © Blizzard Entertainment. This site is in no way associated with or endorsed by Blizzard Entertainment®.";

/// Asset-source lines mirroring the ATTRIBUTION.md sources table (kept in
/// English: they are the license statements themselves).
const SOURCE_LINES: &[&str] = &[
    "World of Warcraft Fan Site Kit and community re-hosts — race, class and faction artwork; © Blizzard Entertainment, free non-commercial fan use.",
    "Blizzard item icons, spell icons, race and class portraits via the Blizzard fan-art CDN (full hosts in assets/ATTRIBUTION.md); © Blizzard Entertainment.",
    "Game content verified against the project knowledge base; sources are recorded under docs/kb-sources/ and docs/wowhead-sources/.",
];

#[component]
pub fn AboutView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",

            main { class: "w-full max-w-3xl mx-auto flex flex-col flex-grow space-y-5",
                div { class: "text-center",
                    h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide",
                        "{i18n_svc.t(\"ui_about_title\", lang)}"
                    }
                    p { class: "text-xs sm:text-sm text-slate-400 mt-1",
                        "{i18n_svc.t(\"ui_about_version\", lang)} {env!(\"CARGO_PKG_VERSION\")}"
                    }
                }

                div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                    span { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                        "{i18n_svc.t(\"ui_about_sources\", lang)}"
                    }
                    for line in SOURCE_LINES.iter() {
                        p { key: "{line}", class: "text-xs sm:text-sm text-slate-300 leading-relaxed", "{line}" }
                    }
                }

                div { class: "w-full bg-slate-900/80 border border-slate-800 rounded-2xl p-5 text-xs sm:text-sm text-slate-400 leading-relaxed",
                    "{REQUIRED_DISCLAIMER_LINE}"
                }

                p { class: "text-xs text-slate-500 text-center",
                    "{i18n_svc.t(\"ui_about_offline\", lang)}"
                }
            }

            AppFooter { current_lang: current_lang, i18n: i18n }        }
    }
}
