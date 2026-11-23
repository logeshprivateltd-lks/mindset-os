//! Pure Rust System-Wide Internationalization (i18n) Module
//!
//! Provides zero-cost compile-time embedded dictionaries for English,
//! Tamil (தமிழ்), and Malayalam (മലയാളം) with fallback to English.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppLanguage {
    English,   // "en"
    Tamil,     // "ta" — தமிழ்
    Malayalam, // "ml" — മലയാളം
}

impl AppLanguage {
    pub fn code(&self) -> &'static str {
        match self {
            AppLanguage::English => "en",
            AppLanguage::Tamil => "ta",
            AppLanguage::Malayalam => "ml",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLanguage::English => "English",
            AppLanguage::Tamil => "தமிழ் (Tamil)",
            AppLanguage::Malayalam => "മലയാളം (Malayalam)",
        }
    }

    pub fn short_label(&self) -> &'static str {
        match self {
            AppLanguage::English => "EN",
            AppLanguage::Tamil => "தமிழ்",
            AppLanguage::Malayalam => "മല",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.trim().to_lowercase().as_str() {
            "ta" | "tamil" => AppLanguage::Tamil,
            "ml" | "malayalam" => AppLanguage::Malayalam,
            _ => AppLanguage::English,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            AppLanguage::English => AppLanguage::Tamil,
            AppLanguage::Tamil => AppLanguage::Malayalam,
            AppLanguage::Malayalam => AppLanguage::English,
        }
    }
}

pub struct I18n {
    current: AppLanguage,
    dictionaries: HashMap<AppLanguage, HashMap<String, String>>,
}

impl Default for I18n {
    fn default() -> Self {
        Self::new(AppLanguage::English)
    }
}

impl I18n {
    pub fn new(lang: AppLanguage) -> Self {
        let mut dictionaries = HashMap::new();

        let en_str = include_str!("en.json");
        let ta_str = include_str!("ta.json");
        let ml_str = include_str!("ml.json");

        if let Ok(en_map) = serde_json::from_str::<HashMap<String, String>>(en_str) {
            dictionaries.insert(AppLanguage::English, en_map);
        }
        if let Ok(ta_map) = serde_json::from_str::<HashMap<String, String>>(ta_str) {
            dictionaries.insert(AppLanguage::Tamil, ta_map);
        }
        if let Ok(ml_map) = serde_json::from_str::<HashMap<String, String>>(ml_str) {
            dictionaries.insert(AppLanguage::Malayalam, ml_map);
        }

        Self {
            current: lang,
            dictionaries,
        }
    }

    pub fn current(&self) -> AppLanguage {
        self.current
    }

    pub fn set_language(&mut self, lang: AppLanguage) {
        self.current = lang;
    }

    pub fn t(&self, key: &str) -> String {
        self.dictionaries
            .get(&self.current)
            .and_then(|d| d.get(key))
            .or_else(|| {
                self.dictionaries
                    .get(&AppLanguage::English)
                    .and_then(|d| d.get(key))
            })
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn greeting(&self, hour: u32, student_name: &str) -> String {
        let prefix_key = if hour < 12 {
            "greeting_morning"
        } else if hour < 17 {
            "greeting_afternoon"
        } else {
            "greeting_evening"
        };

        let prefix = self.t(prefix_key);
        format!("{}, {}!", prefix, student_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_translation_and_fallback() {
        let mut i18n = I18n::new(AppLanguage::English);
        assert_eq!(i18n.t("greeting_morning"), "Good Morning");
        assert_eq!(i18n.greeting(9, "Karthik"), "Good Morning, Karthik!");

        i18n.set_language(AppLanguage::Tamil);
        assert_eq!(i18n.t("greeting_morning"), "காலை வணக்கம்");
        assert_eq!(i18n.greeting(18, "Karthik"), "மாலை வணக்கம், Karthik!");

        i18n.set_language(AppLanguage::Malayalam);
        assert_eq!(i18n.t("greeting_morning"), "സുപ്രഭാതം");
        assert_eq!(i18n.greeting(14, "Karthik"), "ശുഭദിനം, Karthik!");

        // Fallback key
        assert_eq!(i18n.t("non_existent_key_123"), "non_existent_key_123");
    }

    #[test]
    fn test_language_cycling() {
        let lang = AppLanguage::English;
        assert_eq!(lang.next(), AppLanguage::Tamil);
        assert_eq!(lang.next().next(), AppLanguage::Malayalam);
        assert_eq!(lang.next().next().next(), AppLanguage::English);
    }
}
