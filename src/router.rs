use crate::components::{BottomNav, NavTab};
use crate::i18n::{I18nService, LanguageStorage};
use crate::models::{Faction, GameData, RaceInfo};
use crate::selection::{effective_class_id, SelectionState};
use crate::styles::APP_CSS;
use crate::views::{
    AboutView, DressingRoomView, DungeonCodexView, FactionPickerView, FishingGuideView,
    GearPlannerView, PetsView, ProgressionGuideView, RaceSelectorView,
};
use dioxus::prelude::*;
use dioxus::document as document;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    FactionPicker {},
    #[route("/race")]
    RaceSelector {},
    #[route("/dressing")]
    DressingRoom {},
    #[route("/progression")]
    ProgressionGuide {},
    #[route("/dungeons")]
    DungeonCodex {},
    #[route("/gear")]
    GearPlanner {},
    #[route("/pets")]
    Pets {},
    #[route("/fishing")]
    Fishing {},
    #[route("/about")]
    About {},
}

/// The parent of each view — the target of the floating back button and the
/// horizontal swipe gesture (`None` on the root, where there is nowhere back to).
fn back_route(route: &Route) -> Option<Route> {
    match route {
        Route::FactionPicker {} => None,
        Route::RaceSelector {} => Some(Route::FactionPicker {}),
        Route::DressingRoom {} => Some(Route::RaceSelector {}),
        Route::ProgressionGuide {} => Some(Route::DressingRoom {}),
        Route::DungeonCodex {} => Some(Route::ProgressionGuide {}),
        Route::GearPlanner {} => Some(Route::DressingRoom {}),
        Route::Pets {} => Some(Route::DressingRoom {}),
        Route::Fishing {} => Some(Route::ProgressionGuide {}),
        Route::About {} => Some(Route::FactionPicker {}),
    }
}

/// Root application shell maintaining character state across views.
#[component]
pub fn App() -> Element {
    let current_lang = use_signal(|| {
        let storage = LanguageStorage::default();
        storage.get_language()
    });
    let i18n = use_signal(I18nService::load_embedded);
    let game_data = use_signal(GameData::load_embedded);

    let mut selected_faction = use_signal(|| Faction::Alliance);
    let mut selected_race = use_signal(|| {
        let data = GameData::load_embedded();
        data.get_race_by_id("human").cloned()
    });
    let mut selected_class = use_signal(|| {
        let data = GameData::load_embedded();
        data.get_class_by_id("warrior").cloned()
    });

    let mut current_route = use_signal(|| Route::FactionPicker {});
    let back_target = back_route(&current_route.read());
    // Swipe-back gesture state: the pointer-press origin and the accumulated
    // horizontal trackpad wheel delta.
    let mut gesture_start = use_signal(|| None::<(f64, f64)>);
    let mut wheel_accum = use_signal(|| 0.0_f64);

    let nav_to_tab = move |tab: NavTab| {
        match tab {
            NavTab::DressingRoom => current_route.set(Route::DressingRoom {}),
            NavTab::Progression => current_route.set(Route::ProgressionGuide {}),
            NavTab::Dungeons => current_route.set(Route::DungeonCodex {}),
            NavTab::Fishing => current_route.set(Route::Fishing {}),
        }
    };

    let active_tab = match *current_route.read() {
        Route::DressingRoom {} => Some(NavTab::DressingRoom),
        Route::ProgressionGuide {} => Some(NavTab::Progression),
        Route::DungeonCodex {} => Some(NavTab::Dungeons),
        Route::Fishing {} => Some(NavTab::Fishing),
        _ => None,
    };

    let race_obj = selected_race
        .read()
        .clone()
        .unwrap_or_else(|| game_data.read().races[0].clone());

    let class_obj = selected_class
        .read()
        .clone()
        .unwrap_or_else(|| game_data.read().classes[0].clone());

    rsx! {
        document::Style {
            "{APP_CSS}"
        }
        div {
            class: "min-h-screen bg-slate-950 text-slate-100 pb-20 font-sans antialiased",
            onpointerdown: move |evt: Event<PointerData>| {
                let point = evt.client_coordinates();
                gesture_start.set(Some((point.x, point.y)));
            },
            onpointerup: move |evt: Event<PointerData>| {
                let start = *gesture_start.read();
                gesture_start.set(None);
                if let Some((sx, sy)) = start {
                    let point = evt.client_coordinates();
                    let (dx, dy) = (point.x - sx, point.y - sy);
                    if dx.abs() > 80.0 && dx.abs() > dy.abs() * 1.5 {
                        let target = back_route(&current_route.read());
                        if let Some(target) = target {
                            current_route.set(target);
                        }
                    }
                }
            },
            onpointercancel: move |_| gesture_start.set(None),
            onwheel: move |evt: Event<WheelData>| {
                let delta = evt.delta().strip_units();
                if delta.x.abs() > 1.0 && delta.x.abs() > delta.y.abs() {
                    let accumulated = *wheel_accum.read() + delta.x;
                    if accumulated.abs() > 140.0 {
                        wheel_accum.set(0.0);
                        let target = back_route(&current_route.read());
                        if let Some(target) = target {
                            current_route.set(target);
                        }
                    } else {
                        wheel_accum.set(accumulated);
                    }
                }
            },

            if let Some(target) = back_target.clone() {
                button {
                    class: "fixed z-40 flex items-center justify-center w-9 h-9 rounded-full bg-slate-900/80 border border-slate-800 text-slate-400 hover:text-amber-300 hover:border-slate-700 transition-colors cursor-pointer",
                    style: "top: 0.75rem; left: 0.75rem;",
                    "aria-label": "Back",
                    onclick: move |_| current_route.set(target.clone()),
                    "←"
                }
            }

            match *current_route.read() {
                Route::FactionPicker {} => rsx! {
                    FactionPickerView {
                        current_lang: current_lang,
                        i18n: i18n,
                        on_open_about: move |_| current_route.set(Route::About {}),
                        on_faction_selected: move |f: Faction| {
                            let current = SelectionState {
                                faction: *selected_faction.read(),
                                race_id: selected_race
                                    .read()
                                    .clone()
                                    .map(|r| r.id)
                                    .unwrap_or_default(),
                                class_id: selected_class
                                    .read()
                                    .clone()
                                    .map(|c| c.id)
                                    .unwrap_or_default(),
                            };
                            let reconciled = current.after_faction_change(&game_data.read(), f);
                            selected_faction.set(reconciled.faction);
                            selected_race.set(
                                game_data.read().get_race_by_id(&reconciled.race_id).cloned(),
                            );
                            selected_class.set(
                                game_data.read().get_class_by_id(&reconciled.class_id).cloned(),
                            );
                            current_route.set(Route::RaceSelector {});
                        },
                    }
                },
                Route::RaceSelector {} => rsx! {
                    RaceSelectorView {
                        current_lang: current_lang,
                        i18n: i18n,
                        game_data: game_data,
                        faction: *selected_faction.read(),
                        selected_race_id: use_signal(|| Some(race_obj.id.clone())),
                        selected_class: selected_class,
                        on_race_selected: move |r: RaceInfo| {
                            let faction = *selected_faction.read();
                            let current_id = selected_class.read().as_ref().map(|c| c.id.clone());
                            if let Some(id) = effective_class_id(faction, &r, current_id.as_deref()) {
                                if let Some(cls) = game_data.read().get_class_by_id(&id) {
                                    selected_class.set(Some(cls.clone()));
                                }
                            }
                            selected_race.set(Some(r.clone()));
                            current_route.set(Route::DressingRoom {});
                        },
                    }
                },
                Route::DressingRoom {} => rsx! {
                    DressingRoomView {
                        current_lang: current_lang,
                        i18n: i18n,
                        race: race_obj.clone(),
                        class: class_obj.clone(),
                        faction: *selected_faction.read(),
                        on_continue_to_guide: move |_| current_route.set(Route::ProgressionGuide {}),
                        on_open_gear_planner: move |_| current_route.set(Route::GearPlanner {}),
                        on_open_pets: move |_| current_route.set(Route::Pets {}),
                    }
                },
                Route::ProgressionGuide {} => rsx! {
                    ProgressionGuideView {
                        current_lang: current_lang,
                        i18n: i18n,
                        game_data: game_data,
                        race: race_obj.clone(),
                        faction: *selected_faction.read(),
                        on_continue_to_dungeons: move |_| current_route.set(Route::DungeonCodex {}),
                        on_open_fishing: move |_| current_route.set(Route::Fishing {}),
                    }
                },
                Route::DungeonCodex {} => rsx! {
                    DungeonCodexView {
                        current_lang: current_lang,
                        i18n: i18n,
                    }
                },
                Route::GearPlanner {} => rsx! {
                    GearPlannerView {
                        current_lang: current_lang,
                        i18n: i18n,
                        faction: *selected_faction.read(),
                        class_id: class_obj.id.clone(),
                    }
                },
                Route::Pets {} => rsx! {
                    PetsView {
                        current_lang: current_lang,
                        i18n: i18n,
                        class: class_obj.clone(),
                    }
                },
                Route::Fishing {} => rsx! {
                    FishingGuideView {
                        current_lang: current_lang,
                        i18n: i18n,
                    }
                },
                Route::About {} => rsx! {
                    AboutView {
                        current_lang: current_lang,
                        i18n: i18n,
                    }
                },
            }

            if let Some(tab) = active_tab {
                BottomNav {
                    active_tab: tab,
                    current_lang: current_lang,
                    i18n: i18n,
                    on_select_tab: nav_to_tab,
                }
            }
        }
    }
}

// Router stub components to satisfy Dioxus #[derive(Routable)]
#[component]
fn FactionPicker() -> Element {
    rsx! { div {} }
}

#[component]
fn RaceSelector() -> Element {
    rsx! { div {} }
}

#[component]
fn DressingRoom() -> Element {
    rsx! { div {} }
}

#[component]
fn ProgressionGuide() -> Element {
    rsx! { div {} }
}

#[component]
fn DungeonCodex() -> Element {
    rsx! { div {} }
}

#[component]
fn GearPlanner() -> Element {
    rsx! { div {} }
}

#[component]
fn Pets() -> Element {
    rsx! { div {} }
}

#[component]
fn Fishing() -> Element {
    rsx! { div {} }
}

#[component]
fn About() -> Element {
    rsx! { div {} }
}
