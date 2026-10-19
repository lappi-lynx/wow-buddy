use std::collections::{BTreeSet, HashMap};

use wow_buddy::i18n::{I18nService, Language};

fn dict(raw: &str) -> HashMap<String, String> {
    serde_json::from_str(raw).expect("locale json must parse")
}

/// `{name}` placeholder tokens found in a template string.
fn placeholders(s: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = s;
    while let Some(start) = rest.find('{') {
        if let Some(end) = rest[start..].find('}') {
            let tok = &rest[start..start + end + 1];
            if tok.len() < 40 && !tok.contains(' ') {
                out.insert(tok.to_string());
            }
            rest = &rest[start + end + 1..];
        } else {
            break;
        }
    }
    out
}

/// Every EN key must exist non-empty in RU/FI, must not echo its own key, and
/// must keep every `{placeholder}` token the EN template uses.
#[test]
fn test_locales_are_complete_against_english() {
    let en = dict(include_str!("../assets/i18n/en.json"));
    let ru = dict(include_str!("../assets/i18n/ru.json"));
    let fi = dict(include_str!("../assets/i18n/fi.json"));

    let mut missing = vec![];
    let mut placeholder_gaps = vec![];
    for (key, en_val) in &en {
        for (name, d) in [("ru", &ru), ("fi", &fi)] {
            match d.get(key) {
                None => missing.push(format!("{name}: {key}")),
                Some(v) => {
                    if v.trim().is_empty() {
                        missing.push(format!("{name}: {key} (empty)"));
                    }
                    if v == key {
                        missing.push(format!("{name}: {key} (echoes key)"));
                    }
                    for tok in placeholders(en_val) {
                        if !v.contains(&tok) {
                            placeholder_gaps.push(format!("{name}: {key} missing {tok}"));
                        }
                    }
                }
            }
        }
    }
    assert!(missing.is_empty(), "missing keys:\n{}", missing.join("\n"));
    assert!(
        placeholder_gaps.is_empty(),
        "placeholder gaps:\n{}",
        placeholder_gaps.join("\n")
    );

    // Reverse direction: keys that exist only in RU/FI are dead weight and
    // usually the sign of a rename that missed a locale.
    let mut extras = vec![];
    for (name, d) in [("ru", &ru), ("fi", &fi)] {
        for key in d.keys() {
            if !en.contains_key(key) {
                extras.push(format!("{name}: {key}"));
            }
        }
    }
    assert!(extras.is_empty(), "extra keys:\n{}", extras.join("\n"));
}

#[test]
fn test_complete_ui_labels_translation() {
    let i18n = I18nService::load_embedded();

    // Faction Picker View Labels
    assert_eq!(i18n.t("ui_choose_allegiance", Language::En), "Choose Your Allegiance");
    assert_eq!(i18n.t("ui_choose_allegiance", Language::Ru), "Выберите свою сторону");
    assert_eq!(i18n.t("ui_choose_allegiance", Language::Fi), "Valitse allegianssisi");

    // Race Selector View Labels
    assert_eq!(i18n.t("ui_racial_culture_heritage", Language::En), "Racial Culture & Heritage");
    assert_eq!(i18n.t("ui_racial_culture_heritage", Language::Ru), "Культура и наследие расы");
    assert_eq!(i18n.t("ui_racial_culture_heritage", Language::Fi), "Rodun kulttuuri ja perintö");

    assert_eq!(i18n.t("ui_available_classes", Language::En), "Available Classes");
    assert_eq!(i18n.t("ui_available_classes", Language::Ru), "Доступные классы");
    assert_eq!(i18n.t("ui_available_classes", Language::Fi), "Käytettävissä olevat luokat");

    assert_eq!(i18n.t("ui_effect", Language::En), "Effect:");
    assert_eq!(i18n.t("ui_effect", Language::Ru), "Эффект:");
    assert_eq!(i18n.t("ui_effect", Language::Fi), "Vaikutus:");

    // Dressing Room View Labels
    assert_eq!(i18n.t("ui_gear_at_level", Language::En), "Gear at level {level}");
    assert_eq!(i18n.t("ui_drag_rotate", Language::En), "Drag to rotate • Pinch to zoom");
    assert_eq!(i18n.t("ui_core_spells", Language::En), "3 Core Beginner Spells");
    assert_eq!(i18n.t("ui_glossary_title", Language::En), "Terminology");
    assert_eq!(i18n.t("ui_glossary_title", Language::Ru), "Термины");
    assert_eq!(i18n.t("ui_glossary_title", Language::Fi), "Termistö");

    assert_eq!(i18n.t("ui_pets_title", Language::En), "Pets & Companions");
    assert_eq!(i18n.t("ui_pets_title", Language::Ru), "Питомцы и спутники");
    assert_eq!(i18n.t("ui_pets_title", Language::Fi), "Lemmikit ja kumppanit");

    // Bottom navigation
    assert_eq!(i18n.t("nav_gear", Language::En), "Gear");
    assert_eq!(i18n.t("nav_gear", Language::Ru), "Снаряжение");
    assert_eq!(i18n.t("nav_gear", Language::Fi), "Varusteet");
    assert_eq!(i18n.t("nav_fishing", Language::En), "Fishing");
    assert_eq!(i18n.t("nav_fishing", Language::Ru), "Рыбалка");
    assert_eq!(i18n.t("nav_fishing", Language::Fi), "Kalastus");

    // Progression Guide View Labels
    assert_eq!(i18n.t("ui_starter_glade_checklist", Language::En), "Starter Glade Checklist (Lvl 1–6)");
    assert_eq!(i18n.t("ui_phase_1_entry", Language::En), "Phase 1: Entry");
    assert_eq!(i18n.t("ui_next_horizon", Language::En), "Next Horizon Direction (Levels 6–12)");
    assert_eq!(i18n.t("ui_auto_compass", Language::En), "Auto Compass");

    // Dungeon Codex View Labels
    assert_eq!(i18n.t("ui_dungeon_survival_codex", Language::En), "Dungeon Survival Codex");
    assert_eq!(i18n.t("ui_estimated_duration", Language::En), "Estimated Duration");
    assert_eq!(i18n.t("ui_bags_preparation", Language::En), "Bags Preparation");
    assert_eq!(i18n.t("ui_communication", Language::En), "Communication");
    assert_eq!(i18n.t("ui_party_chat_only", Language::En), "/p party chat only");
    assert_eq!(i18n.t("ui_loot_etiquette_test", Language::En), "Interactive Simulator: Loot Etiquette Test");
    assert_eq!(i18n.t("ui_golden_rule", Language::En), "Golden Rule");
    assert_eq!(i18n.t("ui_need_warning", Language::En), "⚠️ Warning: Rolling Need for vendor gold will get you kicked and blacklisted on your realm!");
    assert_eq!(i18n.t("ui_greed_success", Language::En), "✓ Perfect! Never roll Need unless it is a direct upgrade you will equip immediately.");

    // Dungeon readiness (Track E)
    assert_eq!(i18n.t("ui_readiness_title", Language::En), "Dungeon readiness");
    assert_eq!(i18n.t("ui_readiness_title", Language::Ru), "Готовность к подземелью");
    assert_eq!(i18n.t("ui_readiness_title", Language::Fi), "Luolastovalmius");
}

/// EN and FI locales must not contain Cyrillic script (the RU locale may
/// keep Latin game terms, but not the reverse: no Russian words in EN/FI).
#[test]
fn test_en_and_fi_locales_contain_no_cyrillic() {
    for (name, raw) in [
        ("en", include_str!("../assets/i18n/en.json")),
        ("fi", include_str!("../assets/i18n/fi.json")),
    ] {
        let map: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(raw).expect("locale json must parse");
        for (key, value) in map {
            if let Some(text) = value.as_str() {
                assert!(
                    !text.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c)),
                    "{name}.json [{key}] contains Cyrillic: {text}"
                );
            }
        }
    }
}
