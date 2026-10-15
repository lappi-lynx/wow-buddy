use wow_buddy::i18n::Language;
use wow_buddy::models::fishing::FishingData;

#[test]
fn fishing_card_has_three_trilingual_steps() {
    let d = FishingData::load_embedded();
    assert_eq!(d.steps.len(), 3);
    for step in &d.steps {
        for l in [Language::En, Language::Ru, Language::Fi] {
            assert!(!step.text.get(l).trim().is_empty(), "{} {l:?}", step.id);
        }
    }
    for l in [Language::En, Language::Ru, Language::Fi] {
        assert!(!d.title.get(l).trim().is_empty(), "title {l:?}");
        assert!(!d.tie_in.get(l).trim().is_empty(), "tie_in {l:?}");
    }
    assert!(d.tie_in.en.contains("Cooking"));
}
