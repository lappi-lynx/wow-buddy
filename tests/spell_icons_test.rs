use wow_buddy::assets::registry::AssetRegistry;
use wow_buddy::models::gear_planner::GearPlannerData;
use wow_buddy::models::{Faction, GameData};

#[test]
fn test_every_core_spell_has_a_resolvable_icon() {
    let reg = AssetRegistry::load_embedded();
    let data = GameData::load_embedded();
    let mut count = 0;
    for class in &data.classes {
        for spell in &class.core_spells {
            count += 1;
            let icon_id = spell.icon_id.as_ref().unwrap_or_else(|| {
                panic!("{}/{} missing icon_id", class.id, spell.name)
            });
            let uri = reg.art_uri(icon_id).unwrap_or_else(|| {
                panic!("{}/{} icon {icon_id} is declared but not generated", class.id, spell.name)
            });
            assert!(uri.starts_with("data:"), "{} icon must be embedded", spell.name);
        }
    }
    assert_eq!(count, 27, "expected 27 core spells");
}

#[test]
fn test_every_gear_weapon_icon_resolves() {
    let reg = AssetRegistry::load_embedded();
    let gear = GearPlannerData::load_embedded();
    for faction in [Faction::Alliance, Faction::Horde] {
        for band in gear.bands_for("warrior", faction) {
            for weapon in &band.weapons {
                if let Some(icon_id) = &weapon.icon_id {
                    assert!(
                        reg.item_icon(icon_id).starts_with("data:"),
                        "{} icon {icon_id} must resolve",
                        weapon.name
                    );
                }
            }
        }
    }
}
