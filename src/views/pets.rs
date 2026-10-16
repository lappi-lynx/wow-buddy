use crate::assets::AssetRegistry;
use crate::components::AppFooter;
use crate::i18n::{I18nService, Language};
use crate::models::pets::{pet_card_kind, pets_sections, PetCardKind, PetsData, PetsSection};
use crate::models::ClassInfo;
use dioxus::prelude::*;

/// `/pets` — families, tame basics, notable leveling pets, warlock minions and
/// the companion teaser. Which sections render is decided by the tested
/// `pets_sections` helper; a deep link without a hero gets the companion view.
#[component]
pub fn PetsView(
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    class: ClassInfo,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = PetsData::load_embedded();
    let kind = pet_card_kind(&class.id);
    let registry = AssetRegistry::load_embedded();

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8",
            main { class: "w-full max-w-3xl mx-auto flex flex-col space-y-4 flex-grow",
                h2 { class: "text-2xl sm:text-3xl font-extrabold text-amber-300 tracking-wide text-center",
                    "{i18n_svc.t(\"ui_pets_title\", lang)}"
                }
                for section in pets_sections(kind).iter() {
                    match *section {
                        PetsSection::Basics => rsx! {
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_basics\", lang)}" }
                                for b in data.basics.iter() {
                                    div { key: "{b.id}", class: "p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-sm font-bold text-slate-200", "{b.title.get(lang)}" }
                                            if let Some(lvl) = b.level_at {
                                                span { class: "text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400",
                                                    "{i18n_svc.t_with(\"ui_pets_level\", lang, &[(\"level\", lvl.to_string())])}"
                                                }
                                            }
                                        }
                                        p { class: "text-xs text-slate-400 mt-1", "{b.body.get(lang)}" }
                                    }
                                }
                            }
                        },
                        PetsSection::Families => rsx! {
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_first_pick\", lang)}" }
                                div { class: "grid grid-cols-1 sm:grid-cols-3 gap-3",
                                    for pick in data.first_pick.iter() {
                                        if let Some(f) = data.families.iter().find(|f| &f.id == pick) {
                                            div { key: "{f.id}", class: "p-3 rounded-xl bg-slate-950/60 border border-slate-800/80 flex flex-col items-center text-center space-y-1",
                                                img { class: "w-11 h-11 rounded-lg", src: "{registry.item_icon(&f.icon_id)}", alt: "{f.name}" }
                                                span { class: "text-sm font-bold text-slate-200", "{f.name}" }
                                                span { class: "text-xs text-slate-400", "{f.note.get(lang)}" }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_families\", lang)}" }
                                div { class: "grid grid-cols-2 sm:grid-cols-4 gap-2",
                                    for f in data.families.iter() {
                                        div { key: "{f.id}", class: "p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80 flex flex-col items-start space-y-1",
                                            img { class: "w-8 h-8 rounded-md", src: "{registry.item_icon(&f.icon_id)}", alt: "{f.name}" }
                                            span { class: "text-xs sm:text-sm font-bold text-slate-200", "{f.name}" }
                                            span { class: "text-[10px] leading-tight text-slate-400",
                                                "{data.legends.types.get(f.pet_type.as_str()).map(|t| t.get(lang).to_string()).unwrap_or_default()} · {f.diet.iter().map(|d| data.legends.diets.get(d.as_str()).map(|l| l.get(lang).to_string()).unwrap_or_default()).collect::<Vec<_>>().join(\" / \")}"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        PetsSection::Notable => rsx! {
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_notable\", lang)}" }
                                for p in data.notable.iter() {
                                    {
                                        let icon_id = data
                                            .families
                                            .iter()
                                            .find(|f| f.id == p.family_id)
                                            .map(|f| f.icon_id.clone())
                                            .unwrap_or_default();
                                        rsx! {
                                            div { key: "{p.id}", class: "flex items-start space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                                img { class: "w-11 h-11 rounded-lg shrink-0", src: "{registry.item_icon(&icon_id)}", alt: "{p.name}" }
                                                div { class: "flex flex-col flex-1",
                                                    span { class: "text-sm font-bold text-slate-200", "{p.name}" }
                                                    span { class: "text-xs text-slate-400", "{p.where_hint.get(lang)}" }
                                                    span { class: "text-xs text-slate-400 mt-0.5", "{p.why.get(lang)}" }
                                                }
                                                span { class: "shrink-0 ml-3 text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400",
                                                    "{i18n_svc.t_with(\"ui_pets_level\", lang, &[(\"level\", p.min_level.to_string())])}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        PetsSection::Minions => rsx! {
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_minions\", lang)}" }
                                for m in data.minions.iter() {
                                    div { key: "{m.id}", class: "flex items-start space-x-3 p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                        img { class: "w-11 h-11 rounded-lg shrink-0", src: "{registry.item_icon(&m.icon_id)}", alt: "{m.name}" }
                                        div { class: "flex flex-col flex-1",
                                            span { class: "text-sm font-bold text-slate-200", "{m.name}" }
                                            span { class: "text-xs text-slate-400", "{m.role.get(lang)}" }
                                        }
                                        span { class: "shrink-0 ml-3 text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-full bg-slate-800 border border-slate-700 text-slate-400",
                                            "{i18n_svc.t_with(\"ui_pets_level\", lang, &[(\"level\", m.learned_at.to_string())])}"
                                        }
                                    }
                                }
                            }
                        },
                        PetsSection::Companion => rsx! {
                            if let Some(c) = data.companion.as_ref() {
                                div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-2",
                                    h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_companion\", lang)}" }
                                    span { class: "text-sm font-bold text-slate-200", "{c.title.get(lang)}" }
                                    p { class: "text-xs text-slate-400", "{c.body.get(lang)}" }
                                    if matches!(kind, PetCardKind::Companion) {
                                        p { class: "text-xs text-slate-500", "{i18n_svc.t(\"ui_pets_companion_note\", lang)}" }
                                    }
                                }
                            }
                        },
                    }
                }
            }
            AppFooter { current_lang: current_lang, i18n: i18n }        }
    }
}
