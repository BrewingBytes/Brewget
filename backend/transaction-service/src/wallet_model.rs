use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_types::{Currency, WalletType};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub balance: f64,
    pub currency: Currency,
    pub wallet_type: WalletType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateWallet {
    pub name: String,
    pub balance: Option<f64>,
    pub currency: Currency,
    pub wallet_type: Option<WalletType>,
}

#[derive(Deserialize)]
pub struct UpdateWallet {
    pub name: Option<String>,
    pub balance: Option<f64>,
    pub currency: Option<Currency>,
    pub wallet_type: Option<WalletType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_wallet_serialization() {
        let wallet_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let wallet = Wallet {
            id: wallet_id,
            user_id,
            name: "My Savings".to_string(),
            balance: 1500.50,
            currency: Currency::Usd,
            wallet_type: WalletType::Savings,
            created_at: now,
            updated_at: now,
        };

        let serialized = serde_json::to_string(&wallet).unwrap();
        assert!(serialized.contains("My Savings"));
        assert!(serialized.contains("usd"));
        assert!(serialized.contains(&wallet_id.to_string()));
    }

    #[test]
    fn test_create_wallet_deserialization() {
        let json = r#"{
            "name": "Business Expenses",
            "balance": 500.0,
            "currency": "eur",
            "wallet_type": "business"
        }"#;

        let create: CreateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(create.name, "Business Expenses");
        assert_eq!(create.balance, Some(500.0));
        assert_eq!(create.currency, Currency::Eur);
        assert_eq!(create.wallet_type, Some(WalletType::Business));
    }

    #[test]
    fn test_update_wallet_deserialization_partial() {
        let json = r#"{
            "name": "Updated Name",
            "balance": 2000.0
        }"#;

        let update: UpdateWallet = serde_json::from_str(json).unwrap();
        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert_eq!(update.balance, Some(2000.0));
        assert_eq!(update.currency, None);
        assert_eq!(update.wallet_type, None);
    }

    #[test]
    fn test_wallet_clone() {
        let wallet_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let wallet = Wallet {
            id: wallet_id,
            user_id,
            name: "Test Wallet".to_string(),
            balance: 100.0,
            currency: Currency::Ron,
            wallet_type: WalletType::General,
            created_at: now,
            updated_at: now,
        };

        let cloned = wallet.clone();

        let serialized_original = serde_json::to_string(&wallet).unwrap();
        let serialized_cloned = serde_json::to_string(&cloned).unwrap();
        assert_eq!(serialized_original, serialized_cloned);
    }
}
