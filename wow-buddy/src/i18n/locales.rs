use serde::{Deserialize, Serialize};

/// Supported interface languages.
/// Default is English (`Language::En`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    Ru,
    Fi,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

impl Language {
    /// Return the ISO 639-1 language code.
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Ru => "ru",
            Language::Fi => "fi",
        }
    }

    /// Return the native human-readable label for language pickers/dropdowns.
    pub fn label(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Ru => "Русский",
            Language::Fi => "Suomi",
        }
    }

    /// Parse a language code into a `Language` variant.
    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_lowercase().as_str() {
            "en" => Some(Language::En),
            "ru" => Some(Language::Ru),
            "fi" => Some(Language::Fi),
            _ => None,
        }
    }

    /// List all currently registered languages in the catalog.
    pub fn all() -> &'static [Language] {
        &[Language::En, Language::Ru, Language::Fi]
    }
}
