use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSpell {
    pub name: String,
    pub description: String,
    pub icon: String,
    /// Embeddable icon asset id (Wowhead icon name); `icon` (emoji) is the fallback.
    #[serde(default)]
    pub icon_id: Option<String>,
    /// Character level at which the first rank is learned (1 = at creation).
    /// `0` means missing data — the schema test treats it as a tripwire.
    #[serde(default)]
    pub learned_at: u8,
    /// Whether the learn level is client/DB-verified.
    #[serde(default)]
    pub verified: bool,
}

/// Whether a core spell is usable at a given character level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellAvailability {
    Now,
    AtLevel(u8),
}

pub fn availability(spell: &CoreSpell, level: u8) -> SpellAvailability {
    if level >= spell.learned_at {
        SpellAvailability::Now
    } else {
        SpellAvailability::AtLevel(spell.learned_at)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassInfo {
    pub id: String,
    pub name: String,
    pub role: String,
    pub difficulty: String,
    pub core_spells: Vec<CoreSpell>,
    pub icon: String,
}
