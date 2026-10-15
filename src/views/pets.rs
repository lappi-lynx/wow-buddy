use crate::components::LanguagePicker;
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
    on_back: EventHandler<()>,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = PetsData::load_embedded();
    let kind = pet_card_kind(&class.id);
    let mut families_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-between p-4 sm:p-8",
            header { class: "w-full max-w-5xl mx-auto flex justify-between items-center py-3 border-b border-slate-800/80 mb-4",
                button {
                    class: "flex items-center justify-center w-9 h-9 text-base text-slate-400 hover:text-amber-300 transition-colors rounded-lg border border-slate-800 hover:border-slate-700 bg-slate-900/60 cursor-pointer",
                    onclick: move |_| on_back.call(()),
                    "←"
                }
                h1 { class: "text-base sm:text-lg font-bold text-amber-300", "{i18n_svc.t(\"ui_pets_title\", lang)}" }
            }
            main { class: "w-full max-w-3xl mx-auto flex flex-col space-y-4 flex-grow",
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
                                div { class: "flex flex-wrap gap-2",
                                    for pick in data.first_pick.iter() {
                                        if let Some(f) = data.families.iter().find(|f| &f.id == pick) {
                                            span { key: "{f.id}", class: "text-xs px-2 py-1 rounded-lg bg-slate-800 border border-slate-700 text-slate-200", "{f.name}" }
                                        }
                                    }
                                }
                                div { class: "flex items-center justify-between",
                                    h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_families\", lang)}" }
                                    button {
                                        class: "text-xs text-amber-300 hover:text-amber-200 cursor-pointer",
                                        onclick: move |_| {
                                            let next = !families_open();
                                            families_open.set(next);
                                        },
                                        if families_open() {
                                            "{i18n_svc.t(\"ui_pets_hide_all\", lang)}"
                                        } else {
                                            "{i18n_svc.t(\"ui_pets_show_all\", lang)}"
                                        }
                                    }
                                }
                                if families_open() {
                                    for f in data.families.iter() {
                                        div { key: "{f.id}", class: "flex items-start justify-between p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                            div { class: "flex flex-col",
                                                span { class: "text-sm font-bold text-slate-200", "{f.name}" }
                                                span { class: "text-xs text-slate-400", "{f.note.get(lang)}" }
                                            }
                                            div { class: "flex flex-col items-end shrink-0 ml-3",
                                                span { class: "text-[10px] uppercase tracking-wider text-slate-400",
                                                    "{i18n_svc.t(\"ui_pets_type\", lang)}: {data.legends.types.get(f.pet_type.as_str()).map(|t| t.get(lang).to_string()).unwrap_or_default()}"
                                                }
                                                span { class: "text-[10px] text-slate-500",
                                                    "{i18n_svc.t(\"ui_pets_diet\", lang)}: {f.diet.iter().map(|d| data.legends.diets.get(d.as_str()).map(|l| l.get(lang).to_string()).unwrap_or_default()).collect::<Vec<_>>().join(\" · \")}"
                                                }
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
                                    div { key: "{p.id}", class: "flex items-start justify-between p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                        div { class: "flex flex-col",
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
                        },
                        PetsSection::Minions => rsx! {
                            div { class: "bg-slate-900/90 border border-slate-800 rounded-2xl p-5 flex flex-col space-y-3",
                                h2 { class: "text-xs font-semibold text-slate-400 uppercase tracking-wider", "{i18n_svc.t(\"ui_pets_minions\", lang)}" }
                                for m in data.minions.iter() {
                                    div { key: "{m.id}", class: "flex items-start justify-between p-2.5 rounded-xl bg-slate-950/60 border border-slate-800/80",
                                        div { class: "flex flex-col",
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
            footer { class: "w-full max-w-5xl mx-auto text-center py-3 text-xs text-slate-400 border-t border-slate-900 mt-6 flex flex-col items-center space-y-2",
                LanguagePicker { current_lang: current_lang }
                "{i18n_svc.t(\"footer_offline\", lang)}"
            }
        }
    }
}
