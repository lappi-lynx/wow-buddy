use crate::assets::AssetRegistry;
use crate::components::AppFooter;
use crate::i18n::{I18nService, Language};
use crate::models::fishing::FishingData;
use dioxus::prelude::*;

/// `/fishing` — the beginner's fishing guide: how to fish, what to buy, every
/// trainer, the 1–300 route and the Nat Pagle artisan quest. Content is
/// sourced and verified (see `assets/data/fishing.json` → `sources`).
#[component]
pub fn FishingGuideView(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = FishingData::load_embedded();
    let registry = AssetRegistry::load_embedded();

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8 select-none",
            main { class: "w-full max-w-3xl mx-auto flex flex-col space-y-4 flex-grow",
                h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide text-center",
                    "🎣 {i18n_svc.t(\"ui_fishing_guide_title\", lang)}"
                }
                div { class: "text-center",
                    span { class: "text-[10px] font-bold uppercase tracking-wider text-amber-400/80",
                        "{i18n_svc.t(\"ui_beta_may_change\", lang)}"
                    }
                }

                // How to fish
                if !data.basics.is_empty() {
                    div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-2",
                        h3 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                            "{i18n_svc.t(\"ui_fishing_basics\", lang)}"
                        }
                        for (i, step) in data.basics.iter().enumerate() {
                            div { key: "{step.id}", class: "flex items-start space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                span { class: "shrink-0 w-6 h-6 rounded-full bg-amber-500/20 border border-amber-500/40 text-amber-300 text-xs font-bold flex items-center justify-center",
                                    "{i + 1}"
                                }
                                span { class: "text-xs sm:text-sm text-slate-300", "{step.text.get(lang)}" }
                            }
                        }
                    }
                }

                // What to buy
                if !data.shop.is_empty() {
                    div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-2",
                        h3 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                            "{i18n_svc.t(\"ui_fishing_shop\", lang)}"
                        }
                        for item in data.shop.iter() {
                            div { key: "{item.item_id}", class: "flex items-center space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                img { class: "w-9 h-9 rounded-md shrink-0", src: "{registry.item_icon(&item.icon_id)}", alt: "{item.name}" }
                                span { class: "text-xs sm:text-sm font-bold text-slate-200", "{item.name}" }
                            }
                        }
                    }
                }

                // Trainers
                if let Some(trainers) = data.trainers.as_ref() {
                    div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                        h3 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                            "{i18n_svc.t(\"ui_fishing_trainers\", lang)}"
                        }
                        div { class: "flex flex-col space-y-2",
                            span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500",
                                "{i18n_svc.t(\"ui_fishing_capitals\", lang)}"
                            }
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-2",
                                for trainer in trainers.capitals.iter() {
                                    {
                                        let faction_key = if trainer.faction == "horde" { "horde_name" } else { "alliance_name" };
                                        rsx! {
                                            div { key: "{trainer.city}", class: "p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80 flex flex-col",
                                                span { class: "text-xs font-bold text-slate-200", "{trainer.trainer}" }
                                                span { class: "text-[10px] text-slate-400", "{trainer.city} · {i18n_svc.t(faction_key, lang)}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "flex flex-col space-y-2",
                            span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500",
                                "{i18n_svc.t(\"ui_fishing_starting_zones\", lang)}"
                            }
                            for zone in trainers.starting_zones.iter() {
                                div { key: "{zone.zone}", class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-1 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                    div { class: "flex flex-col",
                                        span { class: "text-xs font-bold text-slate-200", "{zone.trainer} · {zone.zone}" }
                                        span { class: "text-[10px] text-slate-400", "{zone.where_hint}" }
                                    }
                                    span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-400", "{zone.races}" }
                                }
                            }
                        }
                    }
                }

                // Leveling route
                if !data.tiers.is_empty() {
                    div { class: "flex flex-col space-y-3",
                        h3 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider px-1",
                            "{i18n_svc.t(\"ui_fishing_route\", lang)}"
                        }
                        for tier in data.tiers.iter() {
                            div { key: "{tier.id}", class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-2",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-sm sm:text-base font-extrabold text-amber-300",
                                        "{i18n_svc.t_with(\"ui_fishing_skill\", lang, &[(\"min\", tier.min.to_string()), (\"max\", tier.max.to_string())])}"
                                    }
                                    span { class: "text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400", "{tier.id}" }
                                }
                                p { class: "text-xs sm:text-sm text-slate-300", "{tier.learn.get(lang)}" }
                                if !tier.zones.is_empty() {
                                    div { class: "flex flex-col space-y-1",
                                        span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500",
                                            "{i18n_svc.t(\"ui_fishing_where\", lang)}"
                                        }
                                        div { class: "flex flex-wrap gap-2",
                                            for zone in tier.zones.iter() {
                                                span { key: "{zone}", class: "text-xs px-2 py-1 rounded-lg bg-slate-800 border border-slate-700 text-slate-200", "{zone}" }
                                            }
                                        }
                                    }
                                }
                                if !tier.lures.is_empty() {
                                    div { class: "flex flex-col space-y-1",
                                        span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500",
                                            "{i18n_svc.t(\"ui_fishing_lures\", lang)}"
                                        }
                                        for lure in tier.lures.iter() {
                                            div { key: "{lure.item_id}", class: "flex items-center space-x-2",
                                                img { class: "w-7 h-7 rounded-md shrink-0", src: "{registry.item_icon(&lure.icon_id)}", alt: "{lure.name}" }
                                                span { class: "text-xs text-slate-200", "{lure.name}" }
                                            }
                                        }
                                    }
                                }
                                if let Some(tips) = tier.tips.as_ref() {
                                    p { class: "text-xs text-slate-400", "💡 {tips.get(lang)}" }
                                }
                            }
                        }
                    }
                }

                // Artisan quest
                if let Some(q) = data.artisan_quest.as_ref() {
                    div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-2",
                        h3 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider",
                            "{i18n_svc.t(\"ui_fishing_artisan\", lang)}"
                        }
                        div { class: "flex flex-wrap items-center gap-2",
                            span { class: "text-sm font-bold text-amber-300", "{q.npc}" }
                            span { class: "text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400", "Quest {q.quest_id}" }
                            span { class: "text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400", "Lvl {q.min_level}" }
                            span { class: "text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400", "Fishing {q.min_skill}" }
                        }
                        p { class: "text-xs sm:text-sm text-slate-300", "{q.text.get(lang)}" }
                        span { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500",
                            "{i18n_svc.t(\"ui_fishing_quest_fish\", lang)}"
                        }
                        for fish in q.fish.iter() {
                            div { key: "{fish.item_id}", class: "flex items-center space-x-2",
                                img { class: "w-7 h-7 rounded-md shrink-0", src: "{registry.item_icon(&fish.icon_id)}", alt: "{fish.name}" }
                                span { class: "text-xs text-slate-200", "{fish.name}" }
                            }
                        }
                    }
                }

                // Cooking tie-in
                p { class: "text-xs text-slate-400 px-1", "{data.tie_in.get(lang)}" }
            }

            AppFooter { current_lang: current_lang, i18n: i18n }
        }
    }
}
