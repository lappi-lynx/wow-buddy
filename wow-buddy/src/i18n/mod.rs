pub mod locales;

pub use locales::Language;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const EN_JSON: &str = include_str!("../../assets/i18n/en.json");
const RU_JSON: &str = include_str!("../../assets/i18n/ru.json");
const FI_JSON: &str = include_str!("../../assets/i18n/fi.json");

/// Internationalization service holding dictionary mappings per language code.
#[derive(Debug, Clone, PartialEq)]
pub struct I18nService {
    dictionaries: HashMap<String, HashMap<String, String>>,
}

impl I18nService {
    /// Load all embedded locales (EN, RU, FI).
    pub fn load_embedded() -> Self {
        let mut service = Self {
            dictionaries: HashMap::new(),
        };

        service
            .register_locale(Language::En.code(), EN_JSON)
            .expect("Valid embedded EN translations");
        service
            .register_locale(Language::Ru.code(), RU_JSON)
            .expect("Valid embedded RU translations");
        service
            .register_locale(Language::Fi.code(), FI_JSON)
            .expect("Valid embedded FI translations");

        service
    }

    /// Register or update translations for a specific language code from JSON.
    /// This allows dynamically adding more languages at runtime or build time.
    pub fn register_locale(
        &mut self,
        lang_code: &str,
        json_content: &str,
    ) -> Result<(), serde_json::Error> {
        let dict: HashMap<String, String> = serde_json::from_str(json_content)?;
        self.dictionaries.insert(lang_code.to_lowercase(), dict);
        Ok(())
    }

    /// Translate a key for the given `Language`.
    /// Falls back to English (`Language::En`), then to the raw `key` if missing.
    pub fn t(&self, key: &str, lang: Language) -> String {
        self.t_by_code(key, lang.code())
    }

    /// Translate a key by language code with fallback to "en" and raw key.
    pub fn t_by_code(&self, key: &str, lang_code: &str) -> String {
        if let Some(dict) = self.dictionaries.get(lang_code) {
            if let Some(val) = dict.get(key) {
                return val.clone();
            }
        }

        // Fallback to English if not already checked
        if lang_code != Language::En.code() {
            if let Some(en_dict) = self.dictionaries.get(Language::En.code()) {
                if let Some(val) = en_dict.get(key) {
                    return val.clone();
                }
            }
        }

        key.to_string()
    }
}

/// Helper for storing and persisting language preference on device.
#[derive(Debug, Clone)]
pub struct LanguageStorage {
    current: Language,
    storage_path: Option<PathBuf>,
}

impl LanguageStorage {
    /// Creates an in-memory storage, useful for testing or ephemeral sessions.
    pub fn in_memory() -> Self {
        Self {
            current: Language::default(),
            storage_path: None,
        }
    }

    /// Creates a persistent storage using a local cache file path.
    pub fn with_file_path(path: PathBuf) -> Self {
        let mut storage = Self {
            current: Language::default(),
            storage_path: Some(path),
        };
        storage.load_from_disk();
        storage
    }

    /// Default device storage in local cache directory.
    pub fn new() -> Self {
        if let Some(mut cache_dir) = dirs_cache_dir() {
            cache_dir.push("wow-buddy");
            let _ = fs::create_dir_all(&cache_dir);
            cache_dir.push("language_preference.json");
            Self::with_file_path(cache_dir)
        } else {
            Self::in_memory()
        }
    }

    /// Read the selected language.
    pub fn get_language(&self) -> Language {
        self.current
    }

    /// Set and persist the selected language.
    pub fn set_language(&mut self, lang: Language) {
        self.current = lang;
        self.save_to_disk();
    }

    fn load_from_disk(&mut self) {
        if let Some(ref path) = self.storage_path {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(lang) = serde_json::from_str::<Language>(&content) {
                    self.current = lang;
                }
            }
        }
    }

    fn save_to_disk(&self) {
        if let Some(ref path) = self.storage_path {
            if let Ok(content) = serde_json::to_string(&self.current) {
                let _ = fs::write(path, content);
            }
        }
    }
}

impl Default for LanguageStorage {
    fn default() -> Self {
        Self::new()
    }
}

fn dirs_cache_dir() -> Option<PathBuf> {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache"))
        })
}
