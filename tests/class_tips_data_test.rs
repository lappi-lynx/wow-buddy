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
fn test_localized_tip_has_all_three_languages() {
    let t = ClassTipsData::load_embedded();
    let spec = t
        .specs_for("druid")
        .iter()
        .find(|s| s.id == "balance")
        .unwrap();
    assert!(!spec.tips.is_empty());
    for tip in &spec.tips {
        assert!(!tip.get(Language::En).is_empty());
        assert!(!tip.get(Language::Ru).is_empty());
        assert!(!tip.get(Language::Fi).is_empty());
    }
    assert!(spec.provenance.is_some(), "spec card must record its KB provenance");
}

#[test]
fn test_coverage_reporter_lists_unauthored_specs() {
    let t = ClassTipsData::load_embedded();
    let missing = t.missing_coverage();
    assert!(!missing.is_empty());
    // сидированный спек не считается пропущенным
    assert!(!missing.iter().any(|m| m.starts_with("druid/balance")));
    // ненаполненный класс перечислен
    assert!(missing.iter().any(|m| m.starts_with("warrior/")));
}

#[test]
fn test_spec_info_records_the_dual_spec_uncertainty() {
    let t = ClassTipsData::load_embedded();
    let info = t.spec_info.as_ref().expect("spec_info must be seeded");
    assert!(!info.get(Language::En).is_empty());
    assert!(info.provenance.is_some());
}
