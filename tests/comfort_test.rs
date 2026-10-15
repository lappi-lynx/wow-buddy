use wow_buddy::components::pick_index_seeded;
use wow_buddy::i18n::Language;
use wow_buddy::models::comfort_tips::ComfortTipsData;

#[test]
fn comfort_deck_has_at_least_8_trilingual_tips() {
    let d = ComfortTipsData::load_embedded();
    assert!(d.tips.len() >= 8, "tips: {}", d.tips.len());
    for tip in &d.tips {
        for l in [Language::En, Language::Ru, Language::Fi] {
            assert!(!tip.text.get(l).trim().is_empty(), "{} {l:?}", tip.id);
        }
    }
}

#[test]
fn random_tip_position_is_always_in_range() {
    for len in 0..=12 {
        for seed in [0_u64, 1, 7, 42, 999, u64::MAX] {
            let i = pick_index_seeded(seed, len);
            assert!(len == 0 || i < len, "seed {seed} len {len} -> {i}");
        }
    }
}

#[test]
fn seeds_spread_across_the_deck() {
    let distinct: std::collections::BTreeSet<usize> =
        (0..40_u64).map(|s| pick_index_seeded(s, 10)).collect();
    assert!(distinct.len() >= 5, "positions: {distinct:?}");
}
