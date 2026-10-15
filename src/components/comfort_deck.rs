use crate::i18n::{I18nService, Language};
use crate::models::comfort_tips::ComfortTipsData;
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Rotates the time seed so quick remounts (same millisecond) never repeat a tip.
static TIP_ROTATION: AtomicU64 = AtomicU64::new(0);

/// Maps a seed onto a 0-based deck position. Pure, so the mapping is unit-tested.
pub fn pick_index_seeded(seed: u64, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (seed % len as u64) as usize
    }
}

#[cfg(target_arch = "wasm32")]
fn clock_seed() -> u64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now() as u64)
        .unwrap_or(0)
}

#[cfg(not(target_arch = "wasm32"))]
fn clock_seed() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// A fresh tip position for every mount: time-based, rotated by an in-process
/// counter so two visits in the same millisecond still differ.
pub fn pick_index(len: usize) -> usize {
    pick_index_seeded(
        clock_seed().wrapping_add(TIP_ROTATION.fetch_add(1, Ordering::Relaxed)),
        len,
    )
}

/// The comfort tips card: one random tip per visit — no arrows, no counter.
#[component]
pub fn ComfortDeck(current_lang: Signal<Language>, i18n: Signal<I18nService>) -> Element {
    let lang = *current_lang.read();
    let i18n_svc = i18n.read();
    let data = ComfortTipsData::load_embedded();
    let total = data.tips.len();
    let tip_index = use_signal(|| pick_index(total));

    let current = (*tip_index.read()).min(total.saturating_sub(1));
    let tip = &data.tips[current];

    rsx! {
        div { class: "w-full bg-slate-900/90 border border-slate-800 rounded-2xl p-5 sm:p-6 shadow-xl flex flex-col space-y-3",
            h3 { class: "text-base sm:text-lg font-bold text-amber-300 border-b border-slate-800 pb-3",
                "💡 {i18n_svc.t(\"ui_comfort_title\", lang)}"
            }
            p { key: "{tip.id}", class: "text-sm text-slate-200 min-h-[3rem]", "{tip.text.get(lang)}" }
        }
    }
}
