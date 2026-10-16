use crate::i18n::{I18nService, Language};
use crate::models::gear::{slider_dots, DotKind, SliderDot};
use dioxus::prelude::*;

/// The shared 1–60 level slider with progress dots: amber dots where the
/// recommended set changes, blue dots at the leveling milestones — filled once
/// they are behind the current level, hollow when still ahead. Dragging never
/// registers as the back-swipe gesture (pointerdown is stopped here).
#[component]
pub fn LevelSlider(
    level: Signal<u8>,
    band_starts: Vec<u8>,
    current_lang: Signal<Language>,
    i18n: Signal<I18nService>,
    #[props(default)] aria_value_text: String,
) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let current = *level.read();
    let dots = slider_dots(&band_starts);

    let dot_style = |dot: &SliderDot| -> String {
        let pct = (dot.level as f32 - 1.0) / 59.0 * 100.0;
        let (size, top, color) = match dot.kind {
            DotKind::SetChange => (12.0, 15.0, "#f59e0b"),
            DotKind::Milestone => (9.0, 30.0, "#60a5fa"),
        };
        let (background, border) = if dot.level <= current {
            (color, "2px solid #0f172a".to_string())
        } else {
            ("transparent", format!("2px solid {color}"))
        };
        format!(
            "position: absolute; left: {pct:.2}%; top: {top}px; width: {size}px; height: {size}px; \
             transform: translateX(-50%); border-radius: 9999px; background: {background}; \
             border: {border};"
        )
    };

    rsx! {
        div { class: "relative",
            input {
                r#type: "range",
                min: "1",
                max: "60",
                step: "1",
                value: "{current}",
                class: "w-full accent-amber-500 cursor-pointer h-11 bg-slate-800 rounded-lg",
                "aria-valuetext": "{aria_value_text}",
                onpointerdown: move |evt: Event<PointerData>| evt.stop_propagation(),
                oninput: move |evt: Event<FormData>| {
                    if let Ok(lvl) = evt.value().parse::<u8>() {
                        level.set(lvl);
                    }
                },
            }
            div {
                style: "position: absolute; top: 0; left: 9px; right: 9px; height: 44px; pointer-events: none;",
                for dot in dots.iter() {
                    span {
                        key: "{dot.level}-{dot.kind:?}",
                        style: "{dot_style(dot)}",
                    }
                }
            }
            div { class: "flex items-center gap-3 mt-1 text-[10px] text-slate-500",
                span { class: "flex items-center gap-1",
                    span {
                        style: "width: 8px; height: 8px; border-radius: 9999px; background: #f59e0b; display: inline-block;",
                    }
                    "{i18n_svc.t(\"ui_dot_set_change\", lang)}"
                }
                span { class: "flex items-center gap-1",
                    span {
                        style: "width: 8px; height: 8px; border-radius: 9999px; background: #60a5fa; display: inline-block;",
                    }
                    "{i18n_svc.t(\"ui_dot_milestone\", lang)}"
                }
            }
        }
    }
}
