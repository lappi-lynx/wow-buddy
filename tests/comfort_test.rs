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
