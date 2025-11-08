use serde::{Deserialize, Serialize};

/// Supported currencies in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    /// United States Dollar
    #[serde(rename = "USD")]
    Usd,
    /// Euro
    #[serde(rename = "EUR")]
    Eur,
    /// British Pound
    #[serde(rename = "GBP")]
    Gbp,
    /// Canadian Dollar
    #[serde(rename = "CAD")]
    Cad,
    /// Japanese Yen
    #[serde(rename = "JPY")]
    Jpy,
    /// Romanian Leu
    #[serde(rename = "RON")]
    Ron,
}

impl Currency {
    /// Returns the currency code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Eur => "EUR",
            Currency::Gbp => "GBP",
            Currency::Cad => "CAD",
            Currency::Jpy => "JPY",
            Currency::Ron => "RON",
        }
    }

    /// Returns all supported currencies
    pub fn all() -> &'static [Currency] {
        &[
            Currency::Usd,
            Currency::Eur,
            Currency::Gbp,
            Currency::Cad,
            Currency::Jpy,
            Currency::Ron,
        ]
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported languages in the application
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English
    #[serde(rename = "en")]
    En,
    /// Spanish
    #[serde(rename = "es")]
    Es,
    /// French
    #[serde(rename = "fr")]
    Fr,
    /// German
    #[serde(rename = "de")]
    De,
    /// Romanian
    #[serde(rename = "ro")]
    Ro,
}

impl Language {
    /// Returns the language code as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Es => "es",
            Language::Fr => "fr",
            Language::De => "de",
            Language::Ro => "ro",
        }
    }

    /// Returns all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::En,
            Language::Es,
            Language::Fr,
            Language::De,
            Language::Ro,
        ]
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_serialization() {
        let usd = Currency::Usd;
        let json = serde_json::to_string(&usd).unwrap();
        assert_eq!(json, r#""USD""#);

        let eur = Currency::Eur;
        let json = serde_json::to_string(&eur).unwrap();
        assert_eq!(json, r#""EUR""#);
    }

    #[test]
    fn test_currency_deserialization() {
        let json = r#""USD""#;
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::Usd);

        let json = r#""EUR""#;
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::Eur);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::Usd.to_string(), "USD");
        assert_eq!(Currency::Eur.to_string(), "EUR");
        assert_eq!(Currency::Gbp.to_string(), "GBP");
    }

    #[test]
    fn test_currency_all() {
        let all = Currency::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&Currency::Usd));
        assert!(all.contains(&Currency::Eur));
    }

    #[test]
    fn test_language_serialization() {
        let en = Language::En;
        let json = serde_json::to_string(&en).unwrap();
        assert_eq!(json, r#""en""#);

        let es = Language::Es;
        let json = serde_json::to_string(&es).unwrap();
        assert_eq!(json, r#""es""#);
    }

    #[test]
    fn test_language_deserialization() {
        let json = r#""en""#;
        let language: Language = serde_json::from_str(json).unwrap();
        assert_eq!(language, Language::En);

        let json = r#""es""#;
        let language: Language = serde_json::from_str(json).unwrap();
        assert_eq!(language, Language::Es);
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::En.to_string(), "en");
        assert_eq!(Language::Es.to_string(), "es");
        assert_eq!(Language::Fr.to_string(), "fr");
    }

    #[test]
    fn test_language_all() {
        let all = Language::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&Language::En));
        assert!(all.contains(&Language::Es));
    }
}
