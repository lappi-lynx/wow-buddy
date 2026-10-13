use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSpell {
    pub name: String,
    pub description: String,
    pub icon: String,
    /// Embeddable icon asset id (Wowhead icon name); `icon` (emoji) is the fallback.
    #[serde(default)]
    pub icon_id: Option<String>,
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
