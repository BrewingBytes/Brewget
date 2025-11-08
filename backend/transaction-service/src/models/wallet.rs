use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use shared_types::enums::{Currency, WalletType};

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
/// * `currency` - Currency code for the wallet matching shared-types Currency enum (USD, EUR, GBP, CAD, JPY, RON)
/// * `wallet_type` - Type of wallet matching shared-types WalletType enum (Account, Savings, Deposit, CreditCard, Loan)
/// * `created_at` - Timestamp when the wallet was created
/// * `updated_at` - Timestamp when the wallet was last updated
#[derive(FromRow, Clone, Serialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub balance: rust_decimal::Decimal,
    pub currency: String,
    pub wallet_type: String,
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
/// * `currency` - Currency for the wallet (enum type-safe)
/// * `wallet_type` - Type of wallet (enum type-safe, defaults to Account)
#[derive(Deserialize)]
pub struct CreateWallet {
    pub name: String,
    #[serde(default)]
    pub balance: Option<rust_decimal::Decimal>,
    pub currency: Currency,
    #[serde(default)]
    pub wallet_type: WalletType,
}

/// Represents updates to a wallet
///
/// This struct is used for partial updates to wallets. All fields are optional,
/// allowing for selective updates without affecting unchanged fields.
///
/// # Fields
///
/// * `name` - Optional new name for the wallet
/// * `currency` - Optional new currency (enum type-safe)
/// * `wallet_type` - Optional new wallet type (enum type-safe)
#[derive(Deserialize)]
pub struct UpdateWallet {
    pub name: Option<String>,
    pub currency: Option<Currency>,
    pub wallet_type: Option<WalletType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_wallet_deserialization() {
        let json = r#"{
            "name": "Savings",
            "balance": 500.00,
            "currency": "EUR"
        }"#;

        let create_wallet: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create_wallet.name, "Savings");
        assert_eq!(create_wallet.currency, Currency::Eur);
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
        assert_eq!(create_wallet.currency, Currency::Gbp);
        assert!(create_wallet.balance.is_none());
    }

    #[test]
    fn test_create_wallet_default_wallet_type() {
        let json = r#"{
            "name": "Main Wallet",
            "currency": "USD"
        }"#;

        let create_wallet: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create_wallet.wallet_type, WalletType::Account);
    }

    #[test]
    fn test_update_wallet_deserialization_partial() {
        let json = r#"{
            "name": "Updated Name"
        }"#;

        let update: UpdateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert_eq!(update.currency, None);
        assert_eq!(update.wallet_type, None);
    }
}
