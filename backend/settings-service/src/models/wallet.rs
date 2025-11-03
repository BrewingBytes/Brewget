use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a wallet stored in the database
///
/// This struct maps to the `wallets` table and contains all wallet data
/// for managing user budgets and financial accounts.
///
/// # Fields
///
/// * `id` - Unique identifier of the wallet
/// * `user_id` - UUID of the user who owns this wallet
/// * `name` - Display name of the wallet (e.g., "Checking Account", "Savings")
/// * `balance` - Current balance in the wallet
/// * `currency` - Currency code (e.g., "USD", "EUR", "GBP")
/// * `wallet_type` - Type of wallet (e.g., "checking", "savings", "cash", "general")
/// * `created_at` - Timestamp when the wallet was created
/// * `updated_at` - Timestamp when the wallet was last updated
#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub balance: sqlx::types::Decimal,
    pub currency: String,
    pub wallet_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request data for creating a new wallet
///
/// This struct is used when a user creates a new wallet.
///
/// # Fields
///
/// * `name` - Display name for the wallet
/// * `balance` - Optional initial balance (defaults to 0.00)
/// * `currency` - Optional currency code (defaults to user's currency preference or "USD")
/// * `wallet_type` - Optional wallet type (defaults to "general")
#[derive(Deserialize)]
pub struct CreateWallet {
    pub name: String,
    pub balance: Option<sqlx::types::Decimal>,
    pub currency: Option<String>,
    pub wallet_type: Option<String>,
}

/// Request data for updating an existing wallet
///
/// This struct is used for partial updates to wallet information.
/// All fields are optional, allowing for selective updates.
///
/// # Fields
///
/// * `name` - Optional new name for the wallet
/// * `balance` - Optional new balance
/// * `currency` - Optional new currency
/// * `wallet_type` - Optional new wallet type
#[derive(Deserialize)]
pub struct UpdateWallet {
    pub name: Option<String>,
    pub balance: Option<sqlx::types::Decimal>,
    pub currency: Option<String>,
    pub wallet_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_wallet_serialization() {
        let wallet = Wallet {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Checking Account".to_string(),
            balance: sqlx::types::Decimal::new(10000, 2), // 100.00
            currency: "USD".to_string(),
            wallet_type: "checking".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&wallet).unwrap();
        assert!(serialized.contains("Checking Account"));
        assert!(serialized.contains("USD"));
        assert!(serialized.contains("checking"));
    }

    #[test]
    fn test_create_wallet_deserialization_all_fields() {
        let json = r#"{
            "name": "Savings",
            "balance": 500.00,
            "currency": "EUR",
            "wallet_type": "savings"
        }"#;

        let create: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create.name, "Savings");
        assert!(create.balance.is_some());
        assert_eq!(create.currency, Some("EUR".to_string()));
        assert_eq!(create.wallet_type, Some("savings".to_string()));
    }

    #[test]
    fn test_create_wallet_deserialization_minimal() {
        let json = r#"{
            "name": "Cash Wallet"
        }"#;

        let create: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create.name, "Cash Wallet");
        assert_eq!(create.balance, None);
        assert_eq!(create.currency, None);
        assert_eq!(create.wallet_type, None);
    }

    #[test]
    fn test_update_wallet_deserialization_partial() {
        let json = r#"{
            "name": "Updated Name",
            "balance": 250.50
        }"#;

        let update: UpdateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert!(update.balance.is_some());
        assert_eq!(update.currency, None);
        assert_eq!(update.wallet_type, None);
    }

    #[test]
    fn test_update_wallet_deserialization_empty() {
        let json = r#"{}"#;

        let update: UpdateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, None);
        assert_eq!(update.balance, None);
        assert_eq!(update.currency, None);
        assert_eq!(update.wallet_type, None);
    }
}
