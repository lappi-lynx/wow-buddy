use wow_buddy::models::class::{availability, CoreSpell, SpellAvailability};
use wow_buddy::models::GameData;

fn spell(name: &str, learned_at: u8) -> CoreSpell {
    CoreSpell {
        name: name.to_string(),
        description: "x".to_string(),
        icon: "⚡".to_string(),
        icon_id: None,
        learned_at,
        verified: true,
    }
}

#[test]
fn availability_is_now_or_at_level() {
    let hs = spell("Heroic Strike", 1);
    assert!(matches!(availability(&hs, 1), SpellAvailability::Now));
    assert!(matches!(availability(&hs, 20), SpellAvailability::Now));
    let charge = spell("Charge", 4);
    assert!(matches!(availability(&charge, 1), SpellAvailability::AtLevel(4)));
    assert!(matches!(availability(&charge, 10), SpellAvailability::Now));
}

#[test]
fn every_core_spell_has_a_learn_level() {
    let data = GameData::load_embedded();
    let mut n = 0;
    for class in &data.classes {
        for s in &class.core_spells {
            n += 1;
            assert!(s.learned_at >= 1, "{}/{} missing learned_at", class.id, s.name);
        }
    }
    assert_eq!(n, 27);
}

#[test]
fn level_one_warrior_knows_only_heroic_strike() {
    let data = GameData::load_embedded();
    let warrior = data.classes.iter().find(|c| c.id == "warrior").unwrap();
    let known = warrior
        .core_spells
        .iter()
        .filter(|s| matches!(availability(s, 1), SpellAvailability::Now))
        .map(|s| s.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(known, vec!["Heroic Strike"]);
}
