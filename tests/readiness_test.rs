use wow_buddy::i18n::Language;
use wow_buddy::models::dungeon::dungeons_for_level;
use wow_buddy::models::readiness::{readiness_level, ReadinessData, ReadinessLevel};

#[test]
fn readiness_meter_thresholds() {
    assert!(matches!(readiness_level(0, 8, 5), ReadinessLevel::NotReady));
    assert!(matches!(readiness_level(4, 8, 5), ReadinessLevel::NotReady));
    assert!(matches!(readiness_level(5, 8, 5), ReadinessLevel::Almost));
    assert!(matches!(readiness_level(8, 8, 5), ReadinessLevel::Ready));
}

#[test]
fn readiness_dataset_is_8_items_and_verified_group_finding() {
    let d = ReadinessData::load_embedded();
    assert_eq!(d.items.len(), 8);
    assert!(
        d.group_finding.verified,
        "the KB confirmed the premade Group Finder (session 542bc6e7)"
    );
    assert!(d
        .group_finding
        .provenance
        .as_deref()
        .map(|p| !p.trim().is_empty())
        .unwrap_or(false));
    for it in &d.items {
        for l in [Language::En, Language::Ru, Language::Fi] {
            assert!(!it.text.get(l).trim().is_empty(), "{} {l:?}", it.id);
        }
    }
    for l in [Language::En, Language::Ru, Language::Fi] {
        assert!(!d.group_finding.text.get(l).trim().is_empty(), "group_finding {l:?}");
    }
}

#[test]
fn dungeons_for_level_filters_by_range() {
    assert_eq!(dungeons_for_level(14).len(), 2); // RFC + Hall of Thanes
    assert!(dungeons_for_level(20).iter().any(|d| d.id == "deadmines"));
    assert!(dungeons_for_level(20).iter().any(|d| d.id == "shadowfang_keep"));
    assert!(!dungeons_for_level(20).iter().any(|d| d.id == "ragefire_chasm"));
}
