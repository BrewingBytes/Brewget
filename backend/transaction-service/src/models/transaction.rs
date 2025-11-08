use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared_types::enums::TransactionType;
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a transaction stored in the database
///
/// This struct maps to the `transactions` table and contains all transaction-specific
/// information for the Brewget application.
///
/// # Fields
///
/// * `id` - Unique identifier of the transaction
/// * `user_id` - Unique identifier of the user who owns this transaction
/// * `wallet_id` - Unique identifier of the wallet associated with this transaction
/// * `amount` - Amount of the transaction (positive for income, negative or positive based on type)
/// * `transaction_type` - Type of transaction (Income, Expense, Transfer)
/// * `category` - Category of the transaction (can be built-in or custom)
/// * `description` - Optional description of the transaction
/// * `transaction_date` - Date when the transaction occurred
/// * `created_at` - Timestamp when the transaction was created
/// * `updated_at` - Timestamp when the transaction was last updated
#[derive(FromRow, Clone, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub transaction_type: String,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Represents a request to create a new transaction
///
/// This struct is used when creating a new transaction.
///
/// # Fields
///
/// * `wallet_id` - ID of the wallet associated with the transaction
/// * `amount` - Amount of the transaction
/// * `transaction_type` - Type of transaction (enum type-safe)
/// * `category` - Category of the transaction (can be built-in or custom string)
/// * `description` - Optional description
/// * `transaction_date` - Optional date (defaults to now)
#[derive(Deserialize)]
pub struct CreateTransaction {
    pub wallet_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub transaction_type: TransactionType,
    pub category: String,
    pub description: Option<String>,
    pub transaction_date: Option<NaiveDateTime>,
}

/// Represents updates to a transaction
///
/// This struct is used for partial updates to transactions. All fields are optional,
/// allowing for selective updates without affecting unchanged fields.
///
/// # Fields
///
/// * `amount` - Optional new amount
/// * `transaction_type` - Optional new transaction type (enum type-safe)
/// * `category` - Optional new category (can be built-in or custom string)
/// * `description` - Optional new description
/// * `transaction_date` - Optional new transaction date
#[derive(Deserialize)]
pub struct UpdateTransaction {
    pub amount: Option<rust_decimal::Decimal>,
    pub transaction_type: Option<TransactionType>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub transaction_date: Option<NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_transaction_deserialization() {
        let json = r#"{
            "wallet_id": "123e4567-e89b-12d3-a456-426614174000",
            "amount": 100.50,
            "transaction_type": "Income",
            "category": "Salary",
            "description": "Monthly salary"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.transaction_type, TransactionType::Income);
        assert_eq!(create_transaction.category, "Salary");
        assert_eq!(
            create_transaction.description,
            Some("Monthly salary".to_string())
        );
    }

    #[test]
    fn test_create_transaction_no_description() {
        let json = r#"{
            "wallet_id": "123e4567-e89b-12d3-a456-426614174000",
            "amount": 50.00,
            "transaction_type": "Expense",
            "category": "Food"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.transaction_type, TransactionType::Expense);
        assert_eq!(create_transaction.category, "Food");
        assert_eq!(create_transaction.description, None);
    }

    #[test]
    fn test_create_transaction_custom_category() {
        let json = r#"{
            "wallet_id": "123e4567-e89b-12d3-a456-426614174000",
            "amount": 25.00,
            "transaction_type": "Expense",
            "category": "My Custom Category"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.transaction_type, TransactionType::Expense);
        assert_eq!(create_transaction.category, "My Custom Category");
    }

    #[test]
    fn test_update_transaction_partial() {
        let json = r#"{
            "amount": 75.00,
            "category": "Transport"
        }"#;

        let update: UpdateTransaction = serde_json::from_str(json).unwrap();
        assert!(update.amount.is_some());
        assert_eq!(update.category, Some("Transport".to_string()));
        assert_eq!(update.transaction_type, None);
    }
}
