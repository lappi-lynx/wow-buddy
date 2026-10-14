use wow_buddy::models::milestones::{milestones_around, next_milestone, MILESTONES};

#[test]
fn next_milestone_and_around() {
    assert_eq!(next_milestone(38).unwrap().level, 40);
    assert!(next_milestone(60).is_none());
    assert!(milestones_around(38, 5).iter().any(|m| m.level == 40));
    assert!(milestones_around(13, 5).iter().any(|m| m.level == 10));
    // window is inclusive at the edge
    assert!(milestones_around(35, 5).iter().any(|m| m.level == 40));
    assert!(!milestones_around(35, 4).iter().any(|m| m.level == 40));
}

#[test]
fn milestones_are_sorted_with_keys() {
    let mut prev = 0u8;
    for m in MILESTONES {
        assert!(!m.key.is_empty(), "empty key at level {}", m.level);
        assert!(m.level > prev, "not strictly increasing at {}", m.level);
        prev = m.level;
    }
    assert_eq!(MILESTONES.len(), 7);
}

#[test]
fn legacy_unlock_is_verified() {
    let legacy = MILESTONES.iter().find(|m| m.key == "ms.legacy").unwrap();
    assert_eq!(legacy.level, 25);
    assert!(legacy.verified);
}
