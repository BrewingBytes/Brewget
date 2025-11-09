use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared_types::enums::{TransactionCategory, TransactionType};
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
/// * `wallet_id` - Unique identifier of the wallet this transaction belongs to
/// * `amount` - Transaction amount (positive for income, can be positive for expenses)
/// * `transaction_type` - Type of transaction (Income, Expense, Transfer)
/// * `category` - Category of the transaction
/// * `description` - Optional description of the transaction
/// * `transaction_date` - Date when the transaction occurred
/// * `destination_wallet_id` - For transfers, the destination wallet ID
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
    pub destination_wallet_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Represents a request to create a new transaction
///
/// This struct is used when creating a new transaction.
///
/// # Fields
///
/// * `wallet_id` - ID of the wallet this transaction belongs to
/// * `amount` - Transaction amount
/// * `transaction_type` - Type of transaction (enum type-safe)
/// * `category` - Category of the transaction (enum type-safe)
/// * `description` - Optional description
/// * `transaction_date` - Optional date (defaults to now)
/// * `destination_wallet_id` - For transfers, the destination wallet ID
#[derive(Deserialize)]
pub struct CreateTransaction {
    pub wallet_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub transaction_type: TransactionType,
    pub category: TransactionCategory,
    pub description: Option<String>,
    pub transaction_date: Option<NaiveDateTime>,
    pub destination_wallet_id: Option<Uuid>,
}

/// Represents updates to a transaction
///
/// This struct is used for partial updates to transactions. All fields are optional,
/// allowing for selective updates without affecting unchanged fields.
///
/// # Fields
///
/// * `amount` - Optional new amount
/// * `category` - Optional new category (enum type-safe)
/// * `description` - Optional new description
/// * `transaction_date` - Optional new transaction date
#[derive(Deserialize)]
pub struct UpdateTransaction {
    pub amount: Option<rust_decimal::Decimal>,
    pub category: Option<TransactionCategory>,
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
            "wallet_id": "550e8400-e29b-41d4-a716-446655440000",
            "amount": 100.50,
            "transaction_type": "Income",
            "category": "Salary",
            "description": "Monthly salary"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.amount.to_string(), "100.5");
        assert_eq!(create_transaction.transaction_type, TransactionType::Income);
        assert_eq!(create_transaction.category, TransactionCategory::Salary);
        assert_eq!(
            create_transaction.description,
            Some("Monthly salary".to_string())
        );
    }

    #[test]
    fn test_create_transaction_deserialization_expense() {
        let json = r#"{
            "wallet_id": "550e8400-e29b-41d4-a716-446655440000",
            "amount": 50.00,
            "transaction_type": "Expense",
            "category": "Food"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.transaction_type, TransactionType::Expense);
        assert_eq!(create_transaction.category, TransactionCategory::Food);
        assert_eq!(create_transaction.description, None);
    }

    #[test]
    fn test_create_transaction_deserialization_transfer() {
        let json = r#"{
            "wallet_id": "550e8400-e29b-41d4-a716-446655440000",
            "amount": 200.00,
            "transaction_type": "Transfer",
            "category": "OtherExpense",
            "destination_wallet_id": "650e8400-e29b-41d4-a716-446655440000"
        }"#;

        let create_transaction: CreateTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(create_transaction.transaction_type, TransactionType::Transfer);
        assert!(create_transaction.destination_wallet_id.is_some());
    }

    #[test]
    fn test_update_transaction_deserialization_partial() {
        let json = r#"{
            "amount": 75.50,
            "description": "Updated description"
        }"#;

        let update: UpdateTransaction = serde_json::from_str(json).unwrap();
        assert!(update.amount.is_some());
        assert_eq!(update.amount.unwrap().to_string(), "75.5");
        assert_eq!(
            update.description,
            Some("Updated description".to_string())
        );
        assert_eq!(update.category, None);
    }
}
