use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported currency types across the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    #[sqlx(rename = "usd")]
    Usd,
    #[sqlx(rename = "eur")]
    Eur,
    #[sqlx(rename = "ron")]
    Ron,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::Usd => write!(f, "usd"),
            Currency::Eur => write!(f, "eur"),
            Currency::Ron => write!(f, "ron"),
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::Usd
    }
}

/// Supported wallet types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "lowercase")]
pub enum WalletType {
    #[sqlx(rename = "general")]
    General,
    #[sqlx(rename = "savings")]
    Savings,
    #[sqlx(rename = "business")]
    Business,
    #[sqlx(rename = "personal")]
    Personal,
}

impl fmt::Display for WalletType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletType::General => write!(f, "general"),
            WalletType::Savings => write!(f, "savings"),
            WalletType::Business => write!(f, "business"),
            WalletType::Personal => write!(f, "personal"),
        }
    }
}

impl Default for WalletType {
    fn default() -> Self {
        WalletType::General
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::Usd.to_string(), "usd");
        assert_eq!(Currency::Eur.to_string(), "eur");
        assert_eq!(Currency::Ron.to_string(), "ron");
    }

    #[test]
    fn test_currency_default() {
        assert_eq!(Currency::default(), Currency::Usd);
    }

    #[test]
    fn test_wallet_type_display() {
        assert_eq!(WalletType::General.to_string(), "general");
        assert_eq!(WalletType::Savings.to_string(), "savings");
        assert_eq!(WalletType::Business.to_string(), "business");
        assert_eq!(WalletType::Personal.to_string(), "personal");
    }

    #[test]
    fn test_wallet_type_default() {
        assert_eq!(WalletType::default(), WalletType::General);
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency::Eur;
        let json = serde_json::to_string(&currency).unwrap();
        assert_eq!(json, r#""eur""#);
    }

    #[test]
    fn test_currency_deserialization() {
        let json = r#""usd""#;
        let currency: Currency = serde_json::from_str(json).unwrap();
        assert_eq!(currency, Currency::Usd);
    }

    #[test]
    fn test_wallet_type_serialization() {
        let wallet_type = WalletType::Business;
        let json = serde_json::to_string(&wallet_type).unwrap();
        assert_eq!(json, r#""business""#);
    }

    #[test]
    fn test_wallet_type_deserialization() {
        let json = r#""savings""#;
        let wallet_type: WalletType = serde_json::from_str(json).unwrap();
        assert_eq!(wallet_type, WalletType::Savings);
    }
}
