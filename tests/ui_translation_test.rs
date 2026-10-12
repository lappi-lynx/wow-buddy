use wow_buddy::i18n::{I18nService, Language};

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
    assert_eq!(i18n.t("ui_gear_evolution", Language::En), "Gear Evolution (Level {level})");
    assert_eq!(i18n.t("ui_drag_rotate", Language::En), "Drag to rotate • Pinch to zoom");
    assert_eq!(i18n.t("ui_core_spells", Language::En), "3 Core Beginner Spells");

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
}
