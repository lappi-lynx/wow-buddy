/// A tap-to-explain glossary term. `id` is the stable slug used by the UI;
/// `text_key`/`short_key` are i18n keys (term name + one-line explanation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Term {
    pub id: &'static str,
    pub text_key: &'static str,
    pub short_key: &'static str,
}

pub const TERMS: &[Term] = &[
    Term { id: "tank", text_key: "glossary_tank", short_key: "glossary_tank_short" },
    Term { id: "melee_dps", text_key: "glossary_melee_dps", short_key: "glossary_melee_dps_short" },
    Term { id: "rage", text_key: "glossary_rage", short_key: "glossary_rage_short" },
    Term { id: "aggro", text_key: "glossary_aggro", short_key: "glossary_aggro_short" },
    Term { id: "taunt", text_key: "glossary_taunt", short_key: "glossary_taunt_short" },
    Term { id: "respec", text_key: "glossary_respec", short_key: "glossary_respec_short" },
    Term { id: "specialization", text_key: "glossary_specialization", short_key: "glossary_specialization_short" },
    Term { id: "legacy_points", text_key: "glossary_legacy_points", short_key: "glossary_legacy_points_short" },
    Term { id: "rested_xp", text_key: "glossary_rested_xp", short_key: "glossary_rested_xp_short" },
    Term { id: "hearthstone", text_key: "glossary_hearthstone", short_key: "glossary_hearthstone_short" },
    Term { id: "flight_path", text_key: "glossary_flight_path", short_key: "glossary_flight_path_short" },
    Term { id: "need_greed", text_key: "glossary_need_greed", short_key: "glossary_need_greed_short" },
    Term { id: "tier2", text_key: "glossary_tier2", short_key: "glossary_tier2_short" },
];

pub fn get_term(id: &str) -> Option<&'static Term> {
    TERMS.iter().find(|t| t.id == id)
}
