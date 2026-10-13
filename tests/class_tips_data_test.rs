use wow_buddy::i18n::Language;
use wow_buddy::models::class_tips::ClassTipsData;

#[test]
fn test_specs_for_class() {
    let t = ClassTipsData::load_embedded();
    let specs = t.specs_for("druid");
    assert_eq!(specs.len(), 3);
    assert!(specs.iter().any(|s| s.id == "balance"));
}

#[test]
fn test_unknown_class_has_no_specs() {
    let t = ClassTipsData::load_embedded();
    assert!(t.specs_for("bard").is_empty());
}

#[test]
fn test_every_seeded_tip_has_all_three_languages() {
    let t = ClassTipsData::load_embedded();
    let mut specs_seen = 0;
    for (class_id, class) in t.classes.iter() {
        for spec in class.specs.iter() {
            specs_seen += 1;
            assert!(!spec.tips.is_empty(), "{class_id}/{} has no tips", spec.id);
            for tip in &spec.tips {
                for lang in [Language::En, Language::Ru, Language::Fi] {
                    assert!(
                        !tip.get(lang).trim().is_empty(),
                        "{class_id}/{}: blank {lang:?} text",
                        spec.id
                    );
                }
            }
            assert!(
                spec.provenance.is_some(),
                "{class_id}/{} missing KB provenance",
                spec.id
            );
        }
    }
    assert!(specs_seen >= 3, "expected the seeded druid slice (3 specs)");
    let info = t.spec_info.as_ref().expect("spec_info must be seeded");
    for lang in [Language::En, Language::Ru, Language::Fi] {
        assert!(
            !info.get(lang).trim().is_empty(),
            "spec_info blank for {lang:?}"
        );
    }
}

#[test]
fn test_coverage_reporter_is_self_consistent() {
    let t = ClassTipsData::load_embedded();
    let total: usize = wow_buddy::models::class_tips::EXPECTED_SPECS
        .iter()
        .map(|(_, specs)| specs.len())
        .sum();
    let authored: usize = t.classes.values().map(|c| c.specs.len()).sum();
    let missing = t.missing_coverage();
    assert_eq!(
        missing.len(),
        total - authored,
        "reporter must equal expected minus authored"
    );
    // каждый пропущенный id валиден по EXPECTED_SPECS
    for m in &missing {
        let (class_id, spec_id) = m.split_once('/').unwrap();
        assert!(wow_buddy::models::class_tips::EXPECTED_SPECS
            .iter()
            .any(|(c, specs)| *c == class_id && specs.contains(&spec_id)));
    }
    // druid — гарантированный срез
    assert!(!missing.iter().any(|m| m.starts_with("druid/")));
}

#[test]
fn test_spec_info_records_the_dual_spec_uncertainty() {
    let t = ClassTipsData::load_embedded();
    let info = t.spec_info.as_ref().expect("spec_info must be seeded");
    assert!(!info.get(Language::En).is_empty());
    assert!(info.provenance.is_some());
}
