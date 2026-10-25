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
            assert_provenance(class_id, &spec.id, &spec.provenance);
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

/// Every spec's provenance must follow the repo convention:
/// `kb:<class>-<spec>@YYYY-MM-DD (<sources>)`.
fn assert_provenance(class_id: &str, spec_id: &str, provenance: &Option<String>) {
    let value = provenance.as_deref().unwrap_or_default();
    let prefix = format!("kb:{class_id}-{spec_id}@");
    assert!(
        value.starts_with(&prefix),
        "{class_id}/{spec_id} provenance must start with {prefix:?}, got {value:?}"
    );
}

/// Shared assertion for every class batch: exactly 3 specs, ≥3 tips each,
/// well-formed provenance, and the class gone from the coverage report.
fn assert_class_authored(data: &ClassTipsData, class_id: &str) {
    let specs = data.specs_for(class_id);
    assert_eq!(specs.len(), 3, "{class_id} must have exactly 3 specs");
    for spec in specs {
        assert!(
            spec.tips.len() >= 3,
            "{class_id}/{} has fewer than 3 tips",
            spec.id
        );
        assert_provenance(class_id, &spec.id, &spec.provenance);
    }
    let prefix = format!("{class_id}/");
    assert!(!data
        .missing_coverage()
        .iter()
        .any(|m| m.starts_with(&prefix)));
}

#[test]
fn test_warrior_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "warrior");
}

#[test]
fn test_paladin_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "paladin");
}

#[test]
fn test_hunter_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "hunter");
}

#[test]
fn test_rogue_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "rogue");
}

#[test]
fn test_priest_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "priest");
}

#[test]
fn test_shaman_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "shaman");
}

#[test]
fn test_mage_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "mage");
}

#[test]
fn test_warlock_specs_are_authored() {
    assert_class_authored(&ClassTipsData::load_embedded(), "warlock");
}

#[test]
fn test_all_expected_specs_are_authored() {
    let t = ClassTipsData::load_embedded();
    assert!(
        t.missing_coverage().is_empty(),
        "unfilled: {:?}",
        t.missing_coverage()
    );
    let mut count = 0;
    for (class_id, class) in t.classes.iter() {
        for spec in class.specs.iter() {
            count += 1;
            assert!(spec.tips.len() >= 3, "{class_id}/{} has <3 tips", spec.id);
            assert_provenance(class_id, &spec.id, &spec.provenance);
        }
    }
    assert_eq!(count, 27);
}

#[test]
fn test_every_class_has_a_kb_source_artifact() {
    for (class_id, _) in wow_buddy::models::class_tips::EXPECTED_SPECS {
        let path = format!(
            "{}/docs/kb-sources/{class_id}.md",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing KB source artifact for {class_id}"));
        assert!(
            content.to_lowercase().contains(class_id),
            "source artifact for {class_id} looks wrong"
        );
    }
}

/// The October 2026 racial update replaced the weapon-specific Human/Orc/Dwarf
/// crits with flat passives. No tip may keep sending players to swords/axes/maces
/// for a crit bonus that no longer exists.
/// Source: docs/wowhead-sources/racial-updates-2026-10-16.md
#[test]
fn test_class_tips_do_not_reference_replaced_weapon_specializations() {
    let t = ClassTipsData::load_embedded();
    let stale = [
        "→ swords",
        "→ axes",
        "→ maces",
        "→ мечи",
        "→ топоры",
        "→ дробящее",
        "→ miekat",
        "→ kirveet",
        "→ nuijat",
    ];
    for (class_id, class) in t.classes.iter() {
        for spec in class.specs.iter() {
            for tip in spec.tips.iter() {
                for lang in [Language::En, Language::Ru, Language::Fi] {
                    let text = tip.get(lang);
                    for marker in stale {
                        assert!(
                            !text.contains(marker),
                            "{class_id}/{} keeps stale weapon-racial advice: {marker:?}",
                            spec.id
                        );
                    }
                }
            }
        }
    }
}
