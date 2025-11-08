use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a wallet stored in the database
///
/// This struct maps to the `wallets` table and contains all wallet-specific
/// information for the Brewget application.
///
/// # Fields
///
/// * `id` - Unique identifier of the wallet
/// * `user_id` - Unique identifier of the user who owns this wallet
/// * `name` - Name of the wallet (e.g., "Savings", "Checking")
/// * `balance` - Current balance of the wallet
/// * `currency` - Currency code for the wallet (e.g., "USD", "EUR", "GBP")
/// * `category` - Optional category for grouping wallets (e.g., "Personal", "Business")
/// * `created_at` - Timestamp when the wallet was created
/// * `updated_at` - Timestamp when the wallet was last updated
#[derive(FromRow, Clone, Serialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub balance: rust_decimal::Decimal,
    pub currency: String,
    pub category: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Represents a request to create a new wallet
///
/// This struct is used when creating a new wallet.
///
/// # Fields
///
/// * `name` - Name of the wallet
/// * `balance` - Optional initial balance (defaults to 0.00)
/// * `currency` - Currency code for the wallet
/// * `category` - Optional category for grouping wallets
#[derive(Deserialize)]
pub struct CreateWallet {
    pub name: String,
    #[serde(default)]
    pub balance: Option<rust_decimal::Decimal>,
    pub currency: String,
    pub category: Option<String>,
}

/// Represents updates to a wallet
///
/// This struct is used for partial updates to wallets. All fields are optional,
/// allowing for selective updates without affecting unchanged fields.
///
/// # Fields
///
/// * `name` - Optional new name for the wallet
/// * `currency` - Optional new currency code
/// * `category` - Optional new category
#[derive(Deserialize)]
pub struct UpdateWallet {
    pub name: Option<String>,
    pub currency: Option<String>,
    pub category: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use serde_json;
    use std::str::FromStr;

    #[test]
    fn test_wallet_serialization() {
        let wallet_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        let wallet = Wallet {
            id: wallet_id,
            user_id,
            name: "Main Wallet".to_string(),
            balance: Decimal::from_str("1000.50").unwrap(),
            currency: "USD".to_string(),
            created_at: now,
            updated_at: now,
        };

        let serialized = serde_json::to_string(&wallet).unwrap();
        assert!(serialized.contains("Main Wallet"));
        assert!(serialized.contains("USD"));
        assert!(serialized.contains(&wallet_id.to_string()));
    }

    #[test]
    fn test_create_wallet_deserialization() {
        let json = r#"{
            "name": "Savings",
            "balance": 500.00,
            "currency": "EUR"
        }"#;

        let create_wallet: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create_wallet.name, "Savings");
        assert_eq!(create_wallet.currency, "EUR");
        assert!(create_wallet.balance.is_some());
    }

    #[test]
    fn test_create_wallet_deserialization_no_balance() {
        let json = r#"{
            "name": "Checking",
            "currency": "GBP"
        }"#;

        let create_wallet: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create_wallet.name, "Checking");
        assert_eq!(create_wallet.currency, "GBP");
        assert!(create_wallet.balance.is_none());
    }

    #[test]
    fn test_update_wallet_deserialization_partial() {
        let json = r#"{
            "name": "Updated Name"
        }"#;

        let update: UpdateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert_eq!(update.balance, None);
        assert_eq!(update.currency, None);
    }

    #[test]
    fn test_wallet_clone() {
        let wallet_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        let wallet = Wallet {
            id: wallet_id,
            user_id,
            name: "Test Wallet".to_string(),
            balance: Decimal::from_str("750.25").unwrap(),
            currency: "CAD".to_string(),
            created_at: now,
            updated_at: now,
        };

        let cloned = wallet.clone();

        let serialized_original = serde_json::to_string(&wallet).unwrap();
        let serialized_cloned = serde_json::to_string(&cloned).unwrap();
        assert_eq!(serialized_original, serialized_cloned);
    }
}
