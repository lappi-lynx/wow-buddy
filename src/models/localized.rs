use serde::{Deserialize, Serialize};

use crate::i18n::Language;

/// A trilingual string in the compact `{en, ru, fi}` shape used by the
/// Track E data files (packable data, unlike the i18n key files).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tr {
    pub en: String,
    pub ru: String,
    pub fi: String,
}

impl Tr {
    pub fn get(&self, lang: Language) -> &str {
        match lang {
            Language::En => &self.en,
            Language::Ru => &self.ru,
            Language::Fi => &self.fi,
        }
    }
}
